import { appendFileSync } from "node:fs";
import { v4 as uuid } from "uuid";

import { ChildProcess, spawn } from "node:child_process";
import { EventEmitter } from "node:events";

export interface GoPayload<Data, Error> {
	id: string;
	event: string;
	data: Data;
	error: Error;
	SR: boolean;
}

export type Channel = string | { event: string; id: string };

/**
  Adapted from https://github.com/Akumzy/ipc-node/blob/master/src/index.ts

  MIT License

  Copyright (c) 2019 Akuma Isaac Akuma

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all
  copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
 */
export class IPC extends EventEmitter {
	go: ChildProcess | null;

	closed: boolean;

	constructor(private binPath: string) {
		super();

		this.go = null;

		this.closed = false;

		/**
		 * The `Golang` process will be pinging at every 20 seconds
		 * and will wait another 20 seconds for reply via `pong` event name
		 * else it will kill it's process.
		 */
		this.on("ping", () => this.send("pong"));
	}

	/**
	 * Start the child process
	 * @param arg
	 */
	public init(arg: string[] = []) {
		const self = this;
		const go = spawn(this.binPath, arg, {});

		this.closed = false;
		this.go = go;

		go.stderr.setEncoding("utf8");
		go.stdout.setEncoding("utf8");

		// emit the errors
		go.stderr.on("error", (e) => self.emit("error", e));
		go.stderr.on("data", (e) => self.emit("log", e));

		let outBuffer = "";

		go.stdout.on("data", (s) => {
			outBuffer += s;

			if (s.endsWith("}\\n")) {
				if (process.env.ODICT_DEBUG_IPC) {
					appendFileSync("ipc.log", `NODE RECEIVED: ${outBuffer}`);
				}
				self._processData(outBuffer);
				outBuffer = "";
			}
		});

		go.once("close", (_) => {
			self.closed = true;
			self.emit("close");
		});

		process.on("beforeExit", () => this.kill());

		return this;
	}

	private _processData(payload: string) {
		const _data = this.parseJSON(payload);

		if (Array.isArray(_data)) {
			for (const item of _data) {
				const { id, error, data, event } = item;

				this.emit("data", item);
				this.emit(event, data, error);
				this.emit(`${event}:${id}`, data, error);
			}
		}
	}

	/**
	 * Kill the child process
	 */
	public kill() {
		try {
			this.send("___EXIT___", null);
			this.closed = true;
			this.go?.kill();
			this.emit("killed");
		} catch (error) {
			console.log(error);
		}
	}

	/**
	 * Send message to `Golang` process
	 * @param event
	 * @param data
	 */
	public send<Data>(event: Channel, data: Data | undefined = undefined) {
		this._send(event, data, false);
	}

	/**
	 * sendRaw gives your access to a third `boolean` argument which
	 * is used to determine if this is a sendAndReceive action
	 */
	public sendRaw<Data>(event: Channel, data: Data, isSendAndReceive = false) {
		this._send(event, data, isSendAndReceive);
	}

	/**
	 *
	 * @param event
	 * @param data
	 * @param SR this tells `Go` process if this message needs an acknowledgement
	 */
	private _send<Data>(event: Channel, data: Data, SR: boolean) {
		try {
			if (this.go && !this.closed && this.go.stdin?.writable) {
				const payload =
					typeof data === "object" || Array.isArray(data)
						? JSON.stringify(data)
						: data;

				// We are converting this to `JSON` this to preserve the
				// data types
				const d = JSON.stringify({
					id: typeof event === "string" ? uuid() : event.id,
					event: typeof event === "string" ? event : event.event,
					data: payload,
					SR: !!SR,
				});

				if (this.go.stdin.writable) {
					const text = d + "\n";

					if (process.env.ODICT_DEBUG_IPC) {
						appendFileSync("ipc.log", `NODE SENT: ${text}`);
					}

					this.go.stdin.write(text);
				}
			}
		} catch (error) {
			this.emit("error", error);
		}
	}

	/**
	 *  Send and receive an acknowledgement through
	 * a callback from `Go` process
	 * @param event
	 * @param data
	 * @param cb
	 */
	public sendAndReceive (
		event: string,
		data: any,
		cb: (error: Error, data: any) => void,
	) {
		const id = uuid();

		this._send({ event, id }, data, true);

		const rc = `${event}:${id}`;

		this.once(rc, (data, error) => {
			if (typeof cb === "function") {
				cb(error, data);
			}
		});
	}

	/**
	 *  Receive and send back acknowledgement/data to `GO`
	 * a callback from `Go` process
	 * @param event
	 * @param data
	 * @param cb
	 */
	public onReceiveAnSend<Data>(
		event: string,
		cb: (channel: Channel, data: Data) => void,
	) {
		const channel = { event, id: uuid() };

		this.on(event, (data) => {
			if (typeof cb === "function") {
				cb(channel, data);
			}
		});
	}

	private parseJSON<Data, Error>(s: string): GoPayload<Data,Error>[] | null {
		try {
			let data = s.replace(/}\\n/g, "},");

			if (data.endsWith(",")) {
				data = data.slice(0, -1).trim();
			}

			return JSON.parse(`[${data}]`);
		} catch (error) {
			this.emit("parse-error", error);
			return null;
		}
	}
}
