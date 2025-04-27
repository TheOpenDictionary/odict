import { describe, expect, it } from "vitest";
import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { randomUUID } from "node:crypto";

import { compile, Dictionary, FormKind } from "../index.js";

describe("Form support", () => {
  it("should handle entries with forms", async () => {
    const xmlContent = `
      <dictionary>
        <entry term="run">
          <ety>
            <sense>
              <definition value="To move quickly on foot." />
              <form kind="inflection" term="ran" />
              <form kind="superlative" term="running" />
              <form term="runs" />
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
    
    // Get the etymology and sense
    const etymology = entry.etymologies[0];
    const sense = Object.values(etymology.senses)[0];

    // Check the forms in the sense
    expect(sense.forms.length).toBe(3);

    // Forms are stored properly with terms and kinds
    expect(sense.forms[0].term).toBe("ran");
    expect(sense.forms[0].kind).toBe("inflection");

    expect(sense.forms[1].term).toBe("running");
    expect(sense.forms[1].kind).toBe("superlative");

    expect(sense.forms[2].term).toBe("runs");
    expect(sense.forms[2].kind).toBeUndefined(); // Optional kind is null when not specified
  });
});

describe("Lemma support", () => {
  it("should handle entries with lemma references", async () => {
    const xmlContent = `
      <dictionary>
        <entry term="running">
          <ety>
            <sense lemma="run">
              <definition value="To move quickly on foot." />
            </sense>
          </ety>
        </entry>
        <entry term="ran">
          <ety>
            <sense lemma="run">
              <definition value="Past tense of run." />
            </sense>
          </ety>
        </entry>
        <entry term="run">
          <form kind="past-tense" term="ran" />
          <form kind="present-participle" term="running" />
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

    // Look up the entries
    const runningResults = dict.lookup("running");
    const ranResults = dict.lookup("ran");

    // Check that we have one result for each
    expect(runningResults.length).toBe(1);
    expect(ranResults.length).toBe(1);

    // Get the entries
    const runningEntry = runningResults[0].entry;
    const ranEntry = ranResults[0].entry;

    // Extract the first etymology
    const runningEtymology = runningEntry.etymologies[0];
    const ranEtymology = ranEntry.etymologies[0];

    // Get the senses (they're in an object keyed by part of speech)
    // The default part of speech is 'n' (noun) from the XML
    const runningSense = Object.values(runningEtymology.senses)[0];
    const ranSense = Object.values(ranEtymology.senses)[0];
    
    // Verify lemma references are on the sense objects
    expect(runningSense.lemma).toBeDefined();
    expect(runningSense.lemma).toBe("run");

    expect(ranSense.lemma).toBeDefined(); 
    expect(ranSense.lemma).toBe("run");
  });
});
