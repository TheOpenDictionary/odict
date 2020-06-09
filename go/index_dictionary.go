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

func createIndex(dictionary OpenDictionary, force bool) string {
	indexPath := getIndexPath(dictionary)
	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		println("Indexing dictionary (this might take some time)...")
		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.New(indexPath, mapping)

		defer index.Close()

		Check(indexErr)

		for entryIdx := range dictionary.Entries {
			entry := dictionary.Entries[entryIdx]
			err := index.Index(entry.ID, entry)
			b, err := json.Marshal(entry)

			if err != nil {
				panic(err)
			}

			index.SetInternal([]byte(entry.ID), b)

			Check(err)
		}
	} else {
		if force {
			println("Purging existing index...")
			os.RemoveAll(indexPath)
			return createIndex(dictionary, false)
		}
	}

	return indexPath
}
