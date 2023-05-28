import { spawn } from "node:child_process";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { ODictMethod } from "./__generated__";
import { IPC } from "./ipc";

type ProcessWrapper = {
  run: <T>(method: ODictMethod, buffer?: Uint8Array) => Promise<T>;
  stop: () => void;
};

const openDictionaries: Map<string, ProcessWrapper> = new Map();

export function teardownServices() {
  openDictionaries.forEach((process) => process.stop());
  openDictionaries.clear();
}

process.on("beforeExit", () => {
  teardownServices();
});

function getExecutablePath() {
  return process.env.RUNTIME_ENV === "test"
    ? join(fileURLToPath(import.meta.url), "../../../bin/odict")
    : "odict";
}

/**
 * Executes the ODict binary
 *
 * @param args Arguments to pass to the executable
 * @returns The raw stdout output string
 */
export function startService(dictionaryPath?: string) {
  const cacheKey = dictionaryPath ?? "shared";

  if (openDictionaries.has(cacheKey)) {
    return openDictionaries.get(cacheKey) as ProcessWrapper;
  }

  const executable = getExecutablePath();
  const ipc = new IPC(executable);

  ipc.init(["service", dictionaryPath ?? ""]);

  function stop() {
    ipc.kill();
  }

  const processWrapper: ProcessWrapper = {
    stop,
    run(method, payload) {
      return new Promise((resolve, reject) => {
        ipc.sendAndReceive(
          ODictMethod[method],
          payload ? Buffer.from(payload).toString("base64") : "",
          (err, data) => (err ? reject(err) : resolve(data))
        );
        // service.stdin.write(
        //   `${method};${
        //     payload ? Buffer.from(payload).toString("base64") : ""
        //   }\n`
        // );
        // let outBuffer = "";
        // function handleError(error: Error) {
        //   reject(error);
        // }
        // function cleanupListeners() {
        //   service.stdout.removeListener("data", buildBuffer);
        //   service.removeListener("error", handleError);
        // }
        // function buildBuffer(data: any) {
        //   outBuffer += data.toString();
        //   if (outBuffer.endsWith("EOF\n")) {
        //     outBuffer = outBuffer.slice(0, -4);
        //     // Kill service if we aren't opening to a dictionary
        //     if (!dictionaryPath) {
        //       stop();
        //     }
        //     cleanupListeners();
        //     console.log(
        //       "event listener removed!",
        //       service.stdout.listenerCount("data")
        //     );
        //     resolve(outBuffer);
        //   }
        // }
        // service.stdout.on("data", buildBuffer);
        // console.log(
        //   "event listener added!",
        //   service.stdout.listenerCount("data")
        // );
        // service.once("error", handleError);
      });
    },
  };

  openDictionaries.set(cacheKey, processWrapper);

  return processWrapper;
}
