package odict

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLookup(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := dict.Lookup([]string{"run", "poo"}, 0)

	assert.Equal(t, 2, len(entries))
	assert.Equal(t, "run", string(entries[0].Term()))
	assert.Equal(t, "poo", string(entries[1].Term()))

	CleanupTest()
}

func TestLookupSplitting(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	entries := dict.Lookup([]string{"catdog"}, 2)

	assert.Equal(t, 2, len(entries))
	assert.Equal(t, "cat", string(entries[0].Term()))
	assert.Equal(t, "dog", string(entries[1].Term()))

	CleanupTest()
}
