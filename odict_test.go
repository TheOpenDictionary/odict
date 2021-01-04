package main

import (
	"log"
	"os"
	"path/filepath"
	"testing"

	odict "github.com/odict/odict/go"
	"github.com/stretchr/testify/assert"
)

func cleanup() {
	dirname := "./examples"

	d, err := os.Open(dirname)

	if err != nil {
		log.Fatal(err)
	}

	defer d.Close()

	files, err := d.Readdir(-1)

	if err != nil {
		log.Fatal(err)
	}

	for _, file := range files {
		if file.Mode().IsRegular() {
			if filepath.Ext(file.Name()) == ".odict" {
				err := os.Remove(filepath.Join(dirname, file.Name()))
				if err != nil {
					log.Fatal(err)
				}
			}
		}
	}
}

func TestReadWriteSearch(t *testing.T) {
	createDictionaryFromPath("examples/example1.xml")

	dict := odict.LoadDictionary("examples/example1.odict", true)
	entries := odict.SearchDictionary(dict, "run")

	assert.NotEmpty(t, entries)

	cleanup()
}

func TestMerge(t *testing.T) {
	createDictionaryFromPath("examples/example1.xml")
	createDictionaryFromPath("examples/example2.xml")

	dict1 := odict.LoadDictionary("examples/example1.odict", true)
	dict2 := odict.LoadDictionary("examples/example2.odict", true)

	assert.Equal(t, dict1.Entries.Size(), 2)
	assert.Equal(t, dict2.Entries.Size(), 1)
	assert.Equal(t, len(dict1.Entries.Get("run").Etymologies), 1)

	merged := odict.MergeDictionaries(dict1, dict2)

	assert.Equal(t, merged.Entries.Size(), 2)
	assert.Equal(t, len(merged.Entries.Get("run").Etymologies), 2)

	cleanup()
}

func TestDump(t *testing.T) {
	createDictionaryFromPath("examples/example1.xml")

	dict := odict.LoadDictionary("examples/example1.odict", true)
	dump := odict.DumpDictionary(dict)

	odict.WriteDictionary(dump, "examples/example1_generated.odict")

	newdict := odict.LoadDictionary("examples/example1_generated.odict", true)

	// We need the IDs to match seeing they will definitely be different
	// due to dictionary regeneration
	newdict.ID = dict.ID

	assert.Equal(t, dict, newdict)

	cleanup()
}
