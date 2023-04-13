package search

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/stretchr/testify/assert"
)

func TestIndex(t *testing.T) {
	core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict := core.ReadDictionaryFromPath("../../examples/example1.odict")

	path, err := os.Getwd()

	utils.Check(err)

	os.Setenv("ODICT_INDEX_DIR", path)

	Index(dict, true, true)

	_, e := os.Stat(filepath.Join(path, "odict", "idx", string(dict.Id())))

	assert.Equal(t, e, nil)

	test.CleanupTest()
}
