package dump

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/stretchr/testify/assert"
)

func TestDictionaryToXML(t *testing.T) {
	_, err := core.CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	assert.Equal(t, nil, err)

	dict, err := core.ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	dict_r := dict.AsRepresentable()
	dump, err := AsXML(dict)

	assert.Equal(t, nil, err)

	core.WriteDictionaryFromXML(dump, "../../examples/example1_generated.odict")

	newdict, err := core.ReadDictionary("../../examples/example1_generated.odict")

	assert.Equal(t, nil, err)

	newdict_r := newdict.AsRepresentable()

	// We need the IDs to match seeing they will definitely be different
	// due to dictionary regeneration
	newdict_r.ID = dict_r.ID

	assert.Equal(t, dict_r, newdict_r)

}
