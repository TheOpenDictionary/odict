package search

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/blevesearch/bleve/v2"
	query "github.com/blevesearch/bleve/v2/search/query"
)

type SearchDictionaryRequest struct {
	Dictionary *types.Dictionary
	Query      string
	Exact      bool
}

// SearchDictionary searches a dictionary model using Bleve using
// it's unique dictionary ID
func SearchDictionary(request SearchDictionaryRequest) ([]types.Entry, error) {
	indexPath := getIndexPath(string(request.Dictionary.Id()))
	_, err := os.Stat(indexPath)

	if os.IsNotExist(err) {
		log.Fatalln("Index path does not exist. Did you call IndexDictionary() first?")
	}

	index, openErr := bleve.Open(indexPath)

	if openErr != nil {
		return nil, openErr
	}

	defer index.Close()

	var query query.Query = bleve.NewMatchQuery(request.Query)

	if request.Exact {
		query = bleve.NewDocIDQuery([]string{strings.ToLower(request.Query)})
	}

	search := bleve.NewSearchRequest(query)
	search.Fields = []string{"_source"}
	searchResults, searchErr := index.Search(search)

	if searchErr != nil {
		return nil, searchErr
	}

	hits := searchResults.Hits

	entries := make([]types.Entry, len(hits))

	for i, x := range hits {
		b, ok := x.Fields["_source"]

		if ok {
			entries[i] = *types.GetRootAsEntry([]byte(fmt.Sprintf("%v", b)), 0)
		}
	}

	return entries, nil
}
