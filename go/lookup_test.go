package odict

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLookup(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := dict.Lookup([]string{"run", "poo"}, 0, false)

	assert.Equal(t, 2, len(entries))
	assert.Equal(t, 1, len(entries[0]))
	assert.Equal(t, 1, len(entries[1]))
	assert.Equal(t, "run", string(entries[0][0].Term()))
	assert.Equal(t, "poo", string(entries[1][0].Term()))

	CleanupTest()
}

func TestLookupSplitting(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := dict.Lookup([]string{"catdog"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, 2, len(entries[0]))
	assert.Equal(t, "cat", string(entries[0][0].Term()))
	assert.Equal(t, "dog", string(entries[0][1].Term()))

	CleanupTest()
}

func TestFallbacks(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := dict.Lookup([]string{"catdog(run)"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	entries = dict.Lookup([]string{"(run)"}, 2, false)

	assert.Equal(t, 1, len(entries))
	assert.Equal(t, "run", string(entries[0][0].Term()))

	CleanupTest()
}

func TestFollow(t *testing.T) {
	CompileDictionary("../examples/example2.xml", "../examples/example2.odict")

	dict := ReadDictionaryFromPath("../examples/example2.odict")

	entries1 := dict.Lookup([]string{"runners"}, 2, false)

	assert.Equal(t, len(entries1), 1)
	assert.Equal(t, "runners", string(entries1[0][0].Term()))

	entries2 := dict.Lookup([]string{"runner"}, 2, true)

	assert.Equal(t, 1, len(entries2))
	assert.Equal(t, "runner", string(entries2[0][0].Term()))

	CleanupTest()
}
