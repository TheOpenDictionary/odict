import { describe, expect, it } from "@jest/globals";

import { PartOfSpeech } from "../src";

describe("Part Of Speech", () => {
	it("generated value map properly", async () => {
		expect(PartOfSpeech.abv).toBe("abv");
	});
});
