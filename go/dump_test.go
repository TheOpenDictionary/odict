package odict

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestDump(t *testing.T) {
	CompileDictionary("../examples/example1.xml")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	dump := DumpDictionary(dict)

	WriteDictionary(dump, "../examples/example1_generated.odict")

	newdict := ReadDictionaryFromPath("../examples/example1_generated.odict")

	// We need the IDs to match seeing they will definitely be different
	// due to dictionary regeneration
	newdict.ID = dict.ID

	assert.Equal(t, dict, newdict)

	CleanupTest()
}
