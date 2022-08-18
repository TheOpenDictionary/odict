package odict

import (
	"github.com/stretchr/testify/assert"
	"os"
	"path/filepath"
	"testing"
)

func TestIndex(t *testing.T) {
	CompileDictionary("../examples/example1.xml")

	dict := ReadDictionaryFromPath("../examples/example1.odict")

	path, err := os.Getwd()

	Check(err)

	os.Setenv("ODICT_INDEX_DIR", path)

	IndexDictionary(dict, true)

	_, e := os.Stat(filepath.Join(path, "odict", "idx", dict.ID))

	assert.Equal(t, e, nil)

	CleanupTest()
}
