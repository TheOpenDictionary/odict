package odict

import (
	"fmt"
	"os"

	"github.com/blevesearch/bleve"
)

func getIndexPath(dictionary OpenDictionary) string {
	return fmt.Sprintf(".%s.idx", dictionary.ID)
}

func createIndex(dictionary OpenDictionary) string {
	indexPath := fmt.Sprintf(".%s.idx", dictionary.ID)
	_, statErr := os.Stat(indexPath)

	if os.IsNotExist(statErr) {
		mapping := bleve.NewIndexMapping()
		index, indexErr := bleve.New(indexPath, mapping)

		Check(indexErr)

		for entryIdx := range dictionary.Entries {
			entry := dictionary.Entries[entryIdx]
			err := index.Index(entry.ID, entry)
			Check(err)
		}
	}
	
	return indexPath
}
