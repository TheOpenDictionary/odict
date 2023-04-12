package core

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLexicon(t *testing.T) {
	CompileDictionary("../examples/example1.xml", "../examples/example1.odict")

	dict := ReadDictionaryFromPath("../examples/example1.odict")

	assert.Equal(t, dict.Lexicon(), []string{"cat", "dog", "poo", "run"})

	CleanupTest()
}
