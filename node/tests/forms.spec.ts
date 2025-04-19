import { beforeAll, describe, expect, it } from "vitest";
import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { tmpdir } from "node:os";
import { randomUUID } from "node:crypto";

import { compile, Dictionary, FormKind } from "../index.js";

describe("Form support", () => {
  it("should handle entries with forms", async () => {
    const xmlContent = `
      <dictionary>
        <entry term="run">
          <forms>
            <form kind="inflection">ran</form>
            <form kind="superlative">running</form>
            <form>runs</form>
          </forms>
          <ety>
            <sense>
              <definition value="To move quickly on foot." />
            </sense>
          </ety>
        </entry>
      </dictionary>
    `;
    
    // Create a temporary file
    const tempFile = join(tmpdir(), `${randomUUID()}.xml`);
    await writeFile(tempFile, xmlContent, "utf-8");
    
    // Compile and load the dictionary
    const compiled = compile(xmlContent);
    const dict = new Dictionary(compiled);
    
    // Look up the entry
    const results = dict.lookup("run");
    
    // Check that we have one result
    expect(results.length).toBe(1);
    
    const entry = results[0].entry;
    
    // Check the forms
    expect(entry.forms.length).toBe(3);
    
    // Forms are stored properly with terms and kinds
    expect(entry.forms[0].term).toBe("ran");
    expect(entry.forms[0].kind).toBe(FormKind.Inflection);
    
    expect(entry.forms[1].term).toBe("running");
    expect(entry.forms[1].kind).toBe(FormKind.Superlative);
    
    expect(entry.forms[2].term).toBe("runs");
    expect(entry.forms[2].kind).toBeUndefined(); // Optional kind is null when not specified
  });
});