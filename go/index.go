package odict

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/blevesearch/bleve"
)

func getIndexPath(dictionaryID string) string {
	return filepath.Join(os.TempDir(), "odict", "idx", dictionaryID)
}

func IndexDictionary(dictionary Dictionary, overwrite bool) string {
	indexPath := getIndexPath(dictionary.ID)
	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		fmt.Println("Indexing dictionary (this might take some time)...")
		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.New(indexPath, mapping)

		defer index.Close()

		Check(indexErr)

		for key := range dictionary.Entries.Iterable {
			entry := dictionary.Entries.Get(key)

			idxErr := index.Index(entry.Term, entry)

			Check(idxErr)

			index.SetInternal([]byte(entry.Term), EncodeEntry(entry))
		}
	} else {
		if overwrite {
			println("Purging existing index...")
			os.RemoveAll(indexPath)
			return IndexDictionary(dictionary, false)
		}
	}

	return indexPath
}
