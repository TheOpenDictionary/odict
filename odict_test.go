package main

import (
	"testing"

	odict "github.com/odict/odict/go"
	"github.com/stretchr/testify/assert"
)

func TestReadWriteSearch(t *testing.T) {
	createDictionaryFromPath("examples/example1.xml")

	dict := odict.LoadDictionary("examples/example1.odict", true)
	entries := odict.SearchDictionary(dict, "run")

	assert.NotEmpty(t, entries)
}

func TestMerge(t *testing.T) {
	createDictionaryFromPath("examples/example1.xml")
	createDictionaryFromPath("examples/example2.xml")

	dict := odict.LoadDictionary("examples/example1.odict", true)
	entries := odict.SearchDictionary(dict, "run")

	assert.NotEmpty(t, entries)
}
