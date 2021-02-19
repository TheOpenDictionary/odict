package odict

import (
	"fmt"
	"os"
	"strings"

	"github.com/blevesearch/bleve/v2"
	query "github.com/blevesearch/bleve/v2/search/query"
)

// SearchDictionary searches a dictionary model using Bleve using
// it's unique dictionary ID
func SearchDictionary(dictionaryID string, queryStr string, exact bool) []Entry {
	indexPath := getIndexPath(dictionaryID)
	_, err := os.Stat(indexPath)

	if os.IsNotExist(err) {
		panic("Index path does not exist. Did you call IndexDictionary() first?")
	}

	index, openErr := bleve.Open(indexPath)

	defer index.Close()

	Check(openErr)

	var query query.Query = bleve.NewMatchQuery(queryStr)

	if exact {
		query = bleve.NewDocIDQuery([]string{strings.ToLower(queryStr)})
	}

	search := bleve.NewSearchRequest(query)
	search.Fields = []string{"_source"}
	searchResults, searchErr := index.Search(search)

	Check(searchErr)

	hits := searchResults.Hits

	entries := make([]Entry, len(hits))

	for i, x := range hits {
		b, ok := x.Fields["_source"]

		if ok {
			entries[i] = DecodeEntry([]byte(fmt.Sprintf("%v", b)))
		}
	}

	return entries
}
