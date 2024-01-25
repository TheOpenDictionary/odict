package dump

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestDictionaryToXML(t *testing.T) {
	_, err := core.CompilePath("../../examples/example1.xml", "../../examples/example1.odict")

	assert.Equal(t, nil, err)

	dict, err := core.ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err)

	dict_r := dict.Struct()
	dump, err := AsXML(dict)

	assert.Equal(t, nil, err)

	core.WriteXML(dump, "../../examples/example1_generated.odict")

	newdict, err := core.ReadDictionary("../../examples/example1_generated.odict")

	assert.Equal(t, nil, err)

	newdict_r := newdict.Struct()

	// We need the IDs to match seeing they will definitely be different
	// due to dictionary regeneration
	newdict_r.ID = dict_r.ID

	assert.Equal(t, dict_r, newdict_r)

	test.CleanupTest()
}
