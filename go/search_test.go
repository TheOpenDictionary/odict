package odict

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestSearch(t *testing.T) {
	CompileDictionary("../examples/example1.xml")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	IndexDictionary(dict, true)
	entries := SearchDictionary(dict.ID, "run", false)

	assert.NotEmpty(t, entries)

	CleanupTest()
}
