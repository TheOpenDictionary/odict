package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestEmpty(t *testing.T) {
	CompileDictionary("../../examples/empty.xml", "../../examples/empty.odict")
	dict, err := ReadDictionary("../../examples/empty.odict")
	assert.Equal(t, nil, err)
	// Ensure a UUID ID is generated for the dictionary
	assert.Regexp(t, "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$", string(dict.Id()))
	test.CleanupTest()
}
