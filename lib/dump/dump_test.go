package dump

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDump(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")
	dict_r := dict.AsRepresentable()
	dump := dict.DumpXML()

	WriteDictionaryFromXML(dump, "../examples/example1_generated.odict")

	newdict := ReadDictionaryFromPath("../examples/example1_generated.odict")
	newdict_r := newdict.AsRepresentable()

	// We need the IDs to match seeing they will definitely be different
	// due to dictionary regeneration
	newdict_r.ID = dict_r.ID

	assert.Equal(t, dict_r, newdict_r)

	CleanupTest()
}
