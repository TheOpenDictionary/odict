package search

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/blevesearch/bleve/v2"
	"github.com/blevesearch/bleve/v2/document"
	"github.com/blevesearch/bleve/v2/index/scorch"
	idx "github.com/blevesearch/bleve_index_api"
	"github.com/schollz/progressbar/v3"
)

type IndexRequest struct {
	Dictionary *types.Dictionary
	Overwrite  bool
	Quiet      bool
}

func getIndexPath(dictionaryID string) string {
	path := os.Getenv("ODICT_INDEX_DIR")

	if len(path) == 0 {
		path = os.TempDir()
	}

	return filepath.Join(path, "odict", "idx", dictionaryID)
}

func Index(request IndexRequest) string {
	dictionary := request.Dictionary.AsRepresentable()
	indexPath := getIndexPath(dictionary.ID)
	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		if !request.Quiet {
			fmt.Println("Indexing dictionary (this might take some time)...")
		}

		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.NewUsing(indexPath, mapping, scorch.Name, scorch.Name, nil)

		utils.Check(indexErr)

		defer index.Close()

		totalEntries := len(dictionary.Entries)

		var bar *progressbar.ProgressBar

		if !request.Quiet {
			bar = progressbar.Default(int64(totalEntries))
		}

		batch := index.NewBatch()
		batchCount := 0
		batchSize := 100

		for key := range dictionary.Entries {
			entry := dictionary.Entries[key]
			doc := document.NewDocument(strings.ToLower(entry.Term))

			mapping.MapDocument(doc, entry)

			enc := types.Serialize(&entry)
			field := document.NewTextFieldWithIndexingOptions("_source", nil, enc, idx.StoreField)
			nd := doc.AddField(field)

			batch.IndexAdvanced(nd)

			batchCount++

			if !request.Quiet {
				bar.Add(1)
			}

			time.Sleep(time.Millisecond)

			if batchCount >= batchSize {
				idxErr := index.Batch(batch)

				utils.Check(idxErr)

				batch = index.NewBatch()
				batchCount = 0
			}
		}

		idxErr := index.Batch(batch)

		utils.Check(idxErr)

		if !request.Quiet {
			fmt.Println()
		}
	} else if request.Overwrite {
		if !request.Quiet {
			fmt.Println("Purging existing index...")
		}

		os.RemoveAll(indexPath)

		return Index(IndexRequest{Dictionary: request.Dictionary, Overwrite: false, Quiet: request.Quiet})
	}

	return indexPath
}
