package dump

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestDictionaryToXML(t *testing.T) {
	core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict := core.ReadDictionaryFromPath("../../examples/example1.odict")
	dict_r := dict.AsRepresentable()
	dump := AsXML(dict)

	core.WriteDictionaryFromXML(dump, "../../examples/example1_generated.odict")

	newdict := core.ReadDictionaryFromPath("../../examples/example1_generated.odict")
	newdict_r := newdict.AsRepresentable()

	// We need the IDs to match seeing they will definitely be different
	// due to dictionary regeneration
	newdict_r.ID = dict_r.ID

	assert.Equal(t, dict_r, newdict_r)

	test.CleanupTest()
}
