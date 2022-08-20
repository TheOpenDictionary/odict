package odict

// import (
// 	"testing"

// 	"github.com/stretchr/testify/assert"
// )

// func TestDump(t *testing.T) {
// 	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

// 	dict := ReadDictionaryFromPath("../examples/example1.odict")
// 	dictr := dict.AsRepresentable()
// 	dump := dict.Dump()

// 	WriteDictionary(dump, "../examples/example1_generated.odict")

// 	newdict := ReadDictionaryFromPath("../examples/example1_generated.odict")
// 	newdictr := newdict.AsRepresentable()

// 	// We need the IDs to match seeing they will definitely be different
// 	// due to dictionary regeneration
// 	newdictr.ID = dictr.ID

// 	assert.Equal(t, dict, newdict)

// 	CleanupTest()
// }
