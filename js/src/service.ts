import { spawn } from "node:child_process";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

let longRunningProcess:
  | { run: (args: string[]) => Promise<string>; stop: () => void }
  | undefined;

export function stopService() {
  longRunningProcess?.stop();
}

/**
 * Executes the ODict binary
 *
 * @param args Arguments to pass to the executable
 * @returns The raw stdout output string
 */
export function startService() {
  if (!longRunningProcess) {
    let executable = "odict";

    if (process.env.RUNTIME_ENV === "test") {
      executable = join(fileURLToPath(import.meta.url), "../../../bin/odict");
    }

    const service = spawn(executable, ["service"], {
      windowsHide: true,
      stdio: ["pipe", "pipe", "inherit"],
      cwd: process.cwd(),
    });

    longRunningProcess = {
      stop() {
        service.stdin.end();
        service.kill();
        longRunningProcess = undefined;
      },
      run(args) {
        return new Promise((resolve, reject) => {
          const request = JSON.stringify([...args]);

          const f = service.stdin.write(request, (err) => {
            console.log(err);
          });

          console.log(request, f);

          service.stdout.on("data", (data) => {
            console.log(data.toString());
            resolve(data);
          });

          service.on("error", (error) => {
            console.log(error.message);
            reject(error);
          });
        });
      },
    };
  }

  return longRunningProcess;
}
