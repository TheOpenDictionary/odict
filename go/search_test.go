package odict

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSearch(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	IndexDictionary(dict, true)
	entries := SearchDictionary(dict.ID, "run", false)

	assert.NotEmpty(t, entries)

	CleanupTest()
}
