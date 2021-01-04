package odict

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/blevesearch/bleve"
	"github.com/odict/odict/go/models"
)

func getIndexPath(dictionary models.Dictionary) string {
	return fmt.Sprintf("%sodict--%s", os.TempDir(), dictionary.ID)
}

func createIndex(dictionary models.Dictionary, force bool) string {
	indexPath := getIndexPath(dictionary)
	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		println("Indexing dictionary (this might take some time)...")
		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.New(indexPath, mapping)

		defer index.Close()

		Check(indexErr)

		for key := range dictionary.Entries.Iterable {
			entry := dictionary.Entries.Get(key)

			idxErr := index.Index(entry.Term, entry)

			Check(idxErr)

			b, err := json.Marshal(entry)

			Check(err)

			index.SetInternal([]byte(entry.Term), b)
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
