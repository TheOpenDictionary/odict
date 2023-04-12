package search

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIndex(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")

	path, err := os.Getwd()

	Check(err)

	os.Setenv("ODICT_INDEX_DIR", path)

	dict.Index(true, true)

	_, e := os.Stat(filepath.Join(path, "odict", "idx", string(dict.Id())))

	assert.Equal(t, e, nil)

	CleanupTest()
}
