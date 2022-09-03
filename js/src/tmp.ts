import { exec as _exec } from "node:child_process";
import { randomBytes } from "node:crypto";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { promisify } from "node:util";

import { exec } from "./exec";
import type { DictionaryOptions } from "./types";

const run = promisify(_exec);

export async function withTemporaryFile(
  cb: (path: string) => Promise<void> | void
) {
  const file = randomBytes(6).toString("hex");
  const tmp = join(tmpdir(), `${file}.xml`);

  await cb(tmp);
  await rm(tmp);
}
