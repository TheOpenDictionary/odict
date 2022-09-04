import { exec as _exec } from "node:child_process";
import { promisify } from "node:util";

const run = promisify(_exec);

/**
 * Executes the ODict binary
 *
 * @param args Arguments to pass to the executable
 * @returns The raw stdout output string
 */
export async function exec(...args: string[]) {
  let odictExecutable = "odict";

  if (process.env.RUNTIME_ENV === "test") {
    odictExecutable = "../../build/odict";
  }

  const { stderr, stdout } = await run(
    ["odict", "--quiet", ...args].join(" "),
    { shell: "/bin/bash" }
  );

  if (stderr) {
    throw new Error(stderr);
  }

  return stdout;
}
