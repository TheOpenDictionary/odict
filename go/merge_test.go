package odict

// import (
// 	"testing"

// 	"github.com/stretchr/testify/assert"
// )

// func TestMerge(t *testing.T) {
// 	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")
// 	CompileDictionary("../examples/example2.xml", "../examples/example2.odict")

// 	dict1 := ReadDictionaryFromPath("../examples/example1.odict")
// 	dict2 := ReadDictionaryFromPath("../examples/example2.odict")

// 	assert.Equal(t, dict1.Entries.Size(), 4)
// 	assert.Equal(t, dict2.Entries.Size(), 1)
// 	assert.Equal(t, len(dict1.Entries.Get("run").Etymologies), 1)
// 	assert.Equal(t, len(dict2.Entries.Get("runner").Etymologies), 1)

// 	merged := MergeDictionaries(dict1, dict2)

// 	assert.Equal(t, merged.Entries.Size(), 5)

// 	CleanupTest()
// }
