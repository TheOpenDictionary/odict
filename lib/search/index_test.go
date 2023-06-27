package search

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestIndex(t *testing.T) {
	core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict, err := core.ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	path, err := os.Getwd()

	assert.Equal(t, nil, err)

	os.Setenv("ODICT_INDEX_DIR", path)

	Index(
		IndexRequest{
			Dictionary: dict,
			Overwrite:  true,
			Quiet:      true,
		},
	)

	_, e := os.Stat(filepath.Join(path, ".odict", "idx", string(dict.Id())))

	assert.Equal(t, e, nil)

	test.CleanupTest()
}
