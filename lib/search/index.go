package search

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/TheOpenDictionary/lib/config"
	"github.com/TheOpenDictionary/odict/lib/types"
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

func getIndexPath(dictionaryID string) (string, error) {
	path, err := config.GetConfigDir()

	if err != nil {
		return "", err
	}

	return filepath.Join(path, "idx", dictionaryID), nil
}

func Index(request IndexRequest) (string, error) {
	dictionary := request.Dictionary.AsRepresentable()
	indexPath, err := getIndexPath(dictionary.ID)

	if err != nil {
		return "", err
	}

	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		if !request.Quiet {
			fmt.Println("Indexing dictionary (this might take some time)...")
		}

		mapping := bleve.NewIndexMapping()

		index, indexErr := bleve.NewUsing(indexPath, mapping, scorch.Name, scorch.Name, nil)

		if indexErr != nil {
			return "", indexErr
		}

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

				if idxErr != nil {
					return "", idxErr
				}

				batch = index.NewBatch()
				batchCount = 0
			}
		}

		idxErr := index.Batch(batch)

		if idxErr != nil {
			return "", idxErr
		}

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

	return indexPath, nil
}
