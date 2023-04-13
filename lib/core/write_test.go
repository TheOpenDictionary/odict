package core

import (
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
)

func TestEmpty(t *testing.T) {
	CompileDictionary("../../examples/empty.xml", "../../examples/empty.odict")
	ReadDictionaryFromPath("../../examples/empty.odict")
	test.CleanupTest()
}
