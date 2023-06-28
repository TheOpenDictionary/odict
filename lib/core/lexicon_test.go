package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestLexicon(t *testing.T) {
	CompileDictionary("../../examples/example1.xml", "../../examples/example1.odict")

	dict, err := ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, err, nil)
	assert.Equal(t, Lexicon(dict), []string{"cat", "dog", "poo", "ran", "run"})

	test.CleanupTest()
}
