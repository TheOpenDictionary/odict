package odict

import (
	"testing"
)

func TestLookup(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	// dict := ReadDictionaryFromPath("../examples/example1.odict")
	// entries := dict.Lookup([]string{"run", "poo"}, 0)

	// assert.Equal(t, 2, len(entries))
	// assert.Equal(t, "run", entries[0].Term)
	// assert.Equal(t, "poo", entries[1].Term)

	// CleanupTest()
}

func TestLookupSplitting(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	// dict := ReadDictionaryFromPath("../examples/example1.odict")
	// entries := dict.Lookup([]string{"catdog"}, 2)

	// assert.Equal(t, 2, len(entries))
	// assert.Equal(t, "cat", entries[0].Term)
	// assert.Equal(t, "dog", entries[1].Term)

	// CleanupTest()
}
