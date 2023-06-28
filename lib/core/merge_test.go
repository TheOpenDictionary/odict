package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestMerge(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")
	CompileDictionary("../../examples/example2.xml", "../../examples/example2.odict")

	dict1, e1 := ReadDictionary("../../examples/example1.odict")
	dict2, e2 := ReadDictionary("../../examples/example2.odict")

	assert.Equal(t, nil, e1)
	assert.Equal(t, nil, e2)

	dict1_r := dict1.AsRepresentable()
	dict2_r := dict2.AsRepresentable()

	assert.Equal(t, dict1.EntriesLength(), 5)
	assert.Equal(t, dict2.EntriesLength(), 2)
	assert.Equal(t, len(dict1_r.Entries["run"].Etymologies), 1)
	assert.Equal(t, len(dict2_r.Entries["runner"].Etymologies), 1)

	merged, err := MergeDictionaries(dict1, dict2)

	assert.Equal(t, nil, err)
	assert.Equal(t, len(merged.Entries), 7)

	test.CleanupTest()
}
