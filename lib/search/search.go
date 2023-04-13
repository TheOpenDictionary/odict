package search

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/blevesearch/bleve/v2"
	query "github.com/blevesearch/bleve/v2/search/query"
)

// SearchDictionary searches a dictionary model using Bleve using
// it's unique dictionary ID
func SearchDictionary(dictionaryID string, queryStr string, exact bool) []types.Entry {
	indexPath := getIndexPath(dictionaryID)
	_, err := os.Stat(indexPath)

	if os.IsNotExist(err) {
		log.Fatalln("Index path does not exist. Did you call IndexDictionary() first?")
	}

	index, openErr := bleve.Open(indexPath)

	utils.Check(openErr)

	defer index.Close()

	var query query.Query = bleve.NewMatchQuery(queryStr)

	if exact {
		query = bleve.NewDocIDQuery([]string{strings.ToLower(queryStr)})
	}

	search := bleve.NewSearchRequest(query)
	search.Fields = []string{"_source"}
	searchResults, searchErr := index.Search(search)

	utils.Check(searchErr)

	hits := searchResults.Hits

	entries := make([]types.Entry, len(hits))

	for i, x := range hits {
		b, ok := x.Fields["_source"]

		if ok {
			entries[i] = *types.GetRootAsEntry([]byte(fmt.Sprintf("%v", b)), 0)
		}
	}

	return entries
}
