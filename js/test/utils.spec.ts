import { Dictionary } from "../src";
import { generateOutputPath } from "../src/utils";

describe("Utilities", () => {
  describe("generateOutputPath", () => {
    it("generates path properly", () => {
      const output = generateOutputPath("/some/fake/path/mydict.xml");
      expect(output).toBe("/some/fake/path/mydict.odict");
    });
  });
});
