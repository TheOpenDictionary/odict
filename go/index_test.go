package odict

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

	IndexDictionary(dict, true)

	_, e := os.Stat(filepath.Join(path, "odict", "idx", dict.ID))

	assert.Equal(t, e, nil)

	CleanupTest()
}
