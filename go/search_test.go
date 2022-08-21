package odict

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSearch(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")

	dict.Index(true, true)

	entries := SearchDictionary(string(dict.Id()), "run", false)

	assert.Equal(t, string(entries[0].Term()), "run")

	CleanupTest()
}
