package odict

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
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

func (dict *Dictionary) Index(overwrite bool) string {
	dictionary := dict.AsRepresentable()
	indexPath := getIndexPath(dictionary.ID)
	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		fmt.Println("Indexing dictionary (this might take some time)...")

		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.NewUsing(indexPath, mapping, scorch.Name, scorch.Name, nil)

		Check(indexErr)

		defer index.Close()

		totalEntries := len(dictionary.Entries)

		bar := progressbar.Default(int64(totalEntries))
		batch := index.NewBatch()
		batchCount := 0
		batchSize := 100

		for key := range dictionary.Entries {
			entry := dictionary.Entries[key]
			doc := document.NewDocument(strings.ToLower(entry.Term))

			mapping.MapDocument(doc, entry)

			enc := serialize(&entry)
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
	} else if overwrite {
		println("Purging existing index...")
		os.RemoveAll(indexPath)
		return dict.Index(false)
	}

	return indexPath
}
