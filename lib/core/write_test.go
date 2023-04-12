package core

import (
	"testing"
)

func TestEmpty(t *testing.T) {
	CompileDictionary("../examples/empty.xml", "../examples/empty.odict")
	ReadDictionaryFromPath("../examples/empty.odict")
	CleanupTest()
}
