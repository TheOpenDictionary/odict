import { spawn } from "node:child_process";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { ODictMethod } from "./fb";

type ProcessWrapper = {
  run: (method: ODictMethod, buffer?: Uint8Array) => Promise<string>;
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

/**
 * Executes the ODict binary
 *
 * @param args Arguments to pass to the executable
 * @returns The raw stdout output string
 */
export function startService(dictionaryPath?: string) {
  if (dictionaryPath && openDictionaries.has(dictionaryPath)) {
    return openDictionaries.get(dictionaryPath) as ProcessWrapper;
  }

  let executable = "odict";

  if (process.env.RUNTIME_ENV === "test") {
    executable = join(fileURLToPath(import.meta.url), "../../../bin/odict");
  }

  const service = spawn(executable, ["service", dictionaryPath ?? ""], {
    windowsHide: true,
    stdio: ["pipe", "pipe", "inherit"],
    cwd: process.cwd(),
  });

  function stop() {
    service?.stdin.end();
    service?.kill();
  }

  service.on("exit", () => {
    if (dictionaryPath) {
      openDictionaries.delete(dictionaryPath);
    }
  });

  service.on("disconnect", () => {
    if (dictionaryPath) {
      openDictionaries.delete(dictionaryPath);
    }
  });

  const processWrapper: ProcessWrapper = {
    stop,
    run(method, payload) {
      return new Promise((resolve, reject) => {
        service.stdin.write(
          `${method};${
            payload ? Buffer.from(payload).toString("base64") : ""
          }\n`
        );

        service.stdout.once("data", (data) => {
          // Kill service if we aren't opening to a dictionary
          if (!dictionaryPath) stop();
          resolve(data);
        });

        service.once("error", (error) => {
          console.log(error.message);
          reject(error);
        });
      });
    },
  };

  if (dictionaryPath) {
    openDictionaries.set(dictionaryPath, processWrapper);
  }

  return processWrapper;
}
