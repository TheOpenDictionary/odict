import { describe, expect, it } from "vitest";
import { compile, Dictionary, PronunciationKind } from "../index";

describe("Pronunciation support", () => {
  it("should parse entries with pronunciations", async () => {
    const xml = `
      <dictionary>
        <entry term="你好">
          <pronunciation kind="pinyin" value="ni hao">
            <url src="./audio.mp3" />
          </pronunciation>
        </entry>
      </dictionary>
    `;
    
    const compiled = compile(xml);
    const dict = new Dictionary(compiled);
    
    const results = dict.lookup("你好");
    expect(results.length).toBe(1);
    
    const entry = results[0].entry;
    expect(entry.pronunciations).toBeDefined();
    expect(entry.pronunciations.length).toBe(1);
    expect(entry.pronunciations[0].kind).toBe(PronunciationKind.Pinyin);
    expect(entry.pronunciations[0].value).toBe("ni hao");
    expect(entry.pronunciations[0].urls.length).toBe(1);
    expect(entry.pronunciations[0].urls[0].src).toBe("./audio.mp3");
  });

  it("should parse examples with pronunciations", async () => {
    const xml = `
      <dictionary>
        <entry term="example">
          <ety>
            <sense pos="n">
              <definition value="An example definition">
                <example value="An example sentence">
                  <pronunciation kind="ipa" value="ɪɡˈzæmpl ˈsɛntəns">
                    <url src="./example.mp3" type="audio/mpeg" />
                  </pronunciation>
                </example>
              </definition>
            </sense>
          </ety>
        </entry>
      </dictionary>
    `;
    
    const compiled = compile(xml);
    const dict = new Dictionary(compiled);
    
    const results = dict.lookup("example");
    expect(results.length).toBe(1);
    
    const entry = results[0].entry;
    const example = entry.etymologies[0].senses["n"].definitions[0].examples[0];
    
    expect(example.pronunciations).toBeDefined();
    expect(example.pronunciations.length).toBe(1);
    expect(example.pronunciations[0].kind).toBe(PronunciationKind.IPA);
    expect(example.pronunciations[0].value).toBe("ɪɡˈzæmpl ˈsɛntəns");
    expect(example.pronunciations[0].urls.length).toBe(1);
    expect(example.pronunciations[0].urls[0].src).toBe("./example.mp3");
    expect(example.pronunciations[0].urls[0].mimeType).toBe("audio/mpeg");
  });

  it("should handle multiple pronunciations for an entry", async () => {
    const xml = `
      <dictionary>
        <entry term="hello">
          <pronunciation kind="ipa" value="həˈləʊ">
            <url src="./hello-british.mp3" />
          </pronunciation>
          <pronunciation kind="ipa" value="hɛˈloʊ">
            <url src="./hello-american.mp3" />
          </pronunciation>
          <ety>
            <sense pos="adj">
              <definition value="A greeting" />
            </sense>
          </ety>
        </entry>
      </dictionary>
    `;
    
    const compiled = compile(xml);
    const dict = new Dictionary(compiled);
    
    const results = dict.lookup("hello");
    expect(results.length).toBe(1);
    
    const entry = results[0].entry;
    expect(entry.pronunciations.length).toBe(2);
    expect(entry.pronunciations[0].value).toBe("həˈləʊ");
    expect(entry.pronunciations[1].value).toBe("hɛˈloʊ");
  });
});