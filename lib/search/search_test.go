package search

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestSearch(t *testing.T) {
	core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict := core.ReadDictionaryFromPath("../../examples/example1.odict")

	Index(dict, true, true)

	entries := SearchDictionary(string(dict.Id()), "run", false)

	assert.Equal(t, string(entries[0].Term()), "run")

	test.CleanupTest()
}
