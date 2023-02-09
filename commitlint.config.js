module.exports = {
	"extends": [ "@commitlint/config-conventional" ],
	"rules": {
		"body-case": [ 2, "always", "sentence-case" ],
		// It is better to limit to 72 characters. However, the limit was increased to allow insertion
		// of long links.
		"body-max-line-length": [ 2, "always", 150 ],
		"signed-off-by": [ 2, "always", "" ],
		"type-enum": [
			2,
			"always",
			[
				"feat",
				"perf",
				"fix",
				"refactor",
				"docs",
				"build",
				"chore",
				"test",
				"style",
				"ci",
				"dprct",
				"remove",
				"dx",
				"security",
				"revert",
				"internal",
				// This will not be included in the change log.
				"release"
			]
		],
		"type-max-length": [ 2, "never", 8 ]
	}
};
