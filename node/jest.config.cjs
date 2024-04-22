module.exports = {
	testEnvironment: "node",
	clearMocks: true,
	extensionsToTreatAsEsm: [".ts"],
	collectCoverage: true,
	coverageDirectory: "coverage",
	coverageProvider: "v8",
	transform: {
		"^.+\\.(t|j)sx?$": ["@swc/jest"],
	},
	moduleNameMapper: {
		"^(\\.{1,2}/.*)\\.js$": "$1",
	},
};
