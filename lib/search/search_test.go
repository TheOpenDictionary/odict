package search

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestSearch(t *testing.T) {
	core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict, err := core.ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	Index(
		IndexRequest{
			Dictionary: dict,
			Overwrite:  true,
			Quiet:      true,
		},
	)

	entries, err := SearchDictionary(string(dict.Id()), "run", false)

	assert.Equal(t, nil, err)
	assert.Equal(t, string(entries[0].Term()), "run")

	test.CleanupTest()
}
