package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestLexicon(t *testing.T) {
	CompilePath("../../examples/example1.xml", "../../examples/example1.odict")

	dict, err := ReadDictionary("../../examples/example1.odict")

	assert.Equal(t, nil, err, err)
	assert.Equal(t, []string{"cat", "dog", "poo", "ran", "run"}, Lexicon(dict))

	test.CleanupTest()
}
