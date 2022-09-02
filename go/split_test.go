package odict

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSplit(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict1 := ReadDictionaryFromPath("../examples/example1.odict")

	entries := dict1.Split("catdog", 1)

	assert.Equal(t, 2, len(entries))
	assert.Equal(t, "cat", string(entries[0].Term()))
	assert.Equal(t, "dog", string(entries[1].Term()))

	CleanupTest()
}
