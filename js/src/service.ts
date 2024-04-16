import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { ODictMethod } from "./__generated__/odict-method.js";
import { IPC } from "./ipc.js";

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

function getCacheKey(dictionaryPath?: string) {
  return dictionaryPath ?? "shared";
}

function createNewProcessWrapper(
  dictionaryPath?: string,
): Promise<ProcessWrapper> {
  return new Promise<ProcessWrapper>((resolve, reject) => {
    const cacheKey = getCacheKey(dictionaryPath);
    const executable = getExecutablePath();
    const ipc = new IPC(executable);

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
            (err, data) => (err ? reject(err) : resolve(data)),
          );
        });
      },
    };

    ipc.on("close", () => openDictionaries.delete(cacheKey));

    ipc.on("killed", () => openDictionaries.delete(cacheKey));

    ipc.on(ODictMethod[ODictMethod.Ready], (data, err) => {
      if (err) {
        stop();

        const msg = dictionaryPath
          ? `Encountered an error starting the ODict service for path "${dictionaryPath}": ${err}`
          : `Encountered an error starting the ODict service: ${err}`;

        reject(msg);
      } else if (data) {
        resolve(processWrapper);
      }
    });

    ipc.init(["service", dictionaryPath ?? ""]);
  });
}
/**
 * Executes the ODict binary
 *
 * @param args Arguments to pass to the executable
 * @returns The raw stdout output string
 */
export async function startService(
  dictionaryPath?: string,
): Promise<ProcessWrapper> {
  const cacheKey = getCacheKey(dictionaryPath);

  if (!openDictionaries.has(cacheKey)) {
    openDictionaries.set(
      cacheKey,
      await createNewProcessWrapper(dictionaryPath),
    );
  }

  return openDictionaries.get(cacheKey) as ProcessWrapper;
}
