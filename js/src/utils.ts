import { basename, dirname, extname, join } from "node:path";

import { Query } from "./types.js";

export function generateOutputPath(xmlPath: string) {
  const dir = dirname(xmlPath);
  const filename = basename(xmlPath, extname(xmlPath));

  return join(dir, [filename, ".odict"].join(""));
}

export function queryToString(query: Query | string) {
  if (typeof query === "string") {
    return query;
  }

  return `${query.word.trim()} (${query.fallback.trim()})`;
}
