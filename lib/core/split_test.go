package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestSplit(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict1 := ReadDictionaryFromPath("../../examples/example1.odict")

	entries := Split(dict1, "catdog", 1)

	assert.Equal(t, 2, len(entries))
	assert.Equal(t, "cat", string(entries[0].Term()))
	assert.Equal(t, "dog", string(entries[1].Term()))

	test.CleanupTest()
}

func TestSplitWithNumbers(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict1 := ReadDictionaryFromPath("../../examples/example1.odict")

	entries := Split(dict1, "2cat8dog", 1)

	assert.Equal(t, 2, len(entries))
	assert.Equal(t, "cat", string(entries[0].Term()))
	assert.Equal(t, "dog", string(entries[1].Term()))

	test.CleanupTest()
}
