package odict

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/blevesearch/bleve"
)

func getIndexPath(dictionary OpenDictionary) string {
	return fmt.Sprintf("%sodict--%s", os.TempDir(), dictionary.ID)
}

func createIndex(dictionary OpenDictionary) string {
	indexPath := getIndexPath(dictionary)
	_, statErr := os.Stat(indexPath)
	println(indexPath)
	if os.IsNotExist(statErr) {
		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.New(indexPath, mapping)

		defer index.Close()

		Check(indexErr)

		for entryIdx := range dictionary.Entries {
			entry := dictionary.Entries[entryIdx]
			err := index.Index(entry.ID, entry)
			b, err := json.Marshal(entry)

			println(entry.Term)
			if err != nil {
				panic(err)
			}

			index.SetInternal([]byte(entry.ID), b)

			Check(err)
		}
	}

	return indexPath
}
