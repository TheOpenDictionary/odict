package odict

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMerge(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")
	CompileDictionary("../examples/example2.xml", "../examples/example2.odict")

	dict1 := ReadDictionaryFromPath("../examples/example1.odict")
	dict2 := ReadDictionaryFromPath("../examples/example2.odict")

	dict1_r := dict1.AsRepresentable()
	dict2_r := dict2.AsRepresentable()

	assert.Equal(t, dict1.EntriesLength(), 4)
	assert.Equal(t, dict2.EntriesLength(), 2)
	assert.Equal(t, len(dict1_r.Entries["run"].Etymologies), 1)
	assert.Equal(t, len(dict2_r.Entries["runner"].Etymologies), 1)

	merged := MergeDictionaries(dict1, dict2)

	assert.Equal(t, len(merged.Entries), 6)

	CleanupTest()
}
