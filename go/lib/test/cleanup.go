package test

import (
	"log"
	"os"
	"path/filepath"

	"github.com/stretchr/testify/assert"
)

func CleanupTest() {
	path, err := os.Getwd()

	assert.Nil(nil, err)

	os.RemoveAll(filepath.Join(path, ".odict"))

	dirname := "../../examples"

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
