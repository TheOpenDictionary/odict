package odict

import (
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/blevesearch/bleve/v2"
	"github.com/blevesearch/bleve/v2/document"
	"github.com/blevesearch/bleve/v2/index/scorch"
	idx "github.com/blevesearch/bleve_index_api"
	"github.com/schollz/progressbar/v3"
)

func getIndexPath(dictionaryID string) string {
	path := os.Getenv("ODICT_INDEX_DIR")

	if len(path) == 0 {
		path = os.TempDir()
	}

	return filepath.Join(path, "odict", "idx", dictionaryID)
}

func IndexDictionary(dictionary Dictionary, overwrite bool) string {
	indexPath := getIndexPath(dictionary.ID)
	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		fmt.Println("Indexing dictionary (this might take some time)...")
		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.NewUsing(indexPath, mapping, scorch.Name, scorch.Name, nil)

		defer index.Close()

		Check(indexErr)

		totalEntries := len(dictionary.Entries.Iterable)

		bar := progressbar.Default(int64(totalEntries))
		batch := index.NewBatch()
		batchCount := 0
		batchSize := 100

		for key := range dictionary.Entries.Iterable {
			entry := dictionary.Entries.Get(key)
			doc := document.NewDocument(entry.Term)

			mapping.MapDocument(doc, entry)

			enc := EncodeEntry(entry)

			field := document.NewTextFieldWithIndexingOptions("_source", nil, enc, idx.StoreField)

			nd := doc.AddField(field)

			batch.IndexAdvanced(nd)

			batchCount++

			bar.Add(1)

			time.Sleep(time.Millisecond)

			if batchCount >= batchSize {
				idxErr := index.Batch(batch)

				Check(idxErr)

				batch = index.NewBatch()
				batchCount = 0
			}
		}

		idxErr := index.Batch(batch)

		Check(idxErr)

		fmt.Println()
	} else {
		if overwrite {
			println("Purging existing index...")
			os.RemoveAll(indexPath)
			return IndexDictionary(dictionary, false)
		}
	}

	return indexPath
}
