package odict

import (
	"os"

	"github.com/blevesearch/bleve"
)

func SearchDictionary(dictionary OpenDictionary, queryStr string) []OpenDictionaryEntry {
	indexPath := getIndexPath(dictionary)
	_, err := os.Stat(indexPath)

	if os.IsNotExist(err) {
		panic("Index path does not exist. Did you call LoadDictionary() first?")
	}

	index, openErr := bleve.Open(indexPath)

	defer index.Close()

	Check(openErr)

	query := bleve.NewMatchQuery(queryStr)
	search := bleve.NewSearchRequest(query)
	searchResults, searchErr := index.Search(search)

	Check(searchErr)

	hits := searchResults.Hits
	hitIDs := make([]string, len(hits))

	for i := range hits {
		hitIDs[i] = hits[i].ID
	}

	results := make([]OpenDictionaryEntry, 0)

	// PLEASE tell me there's a better way
	for _, entry := range dictionary.Entries {
		for _, id := range hitIDs {
			if id == entry.ID {
				results = append(results, entry)
				break
			}
		}
	}

	return results
}
