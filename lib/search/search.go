package search

import (
	"log"
	"os"

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
	indexPath, err := getIndexPath(string(request.Dictionary.Id()))

	if err != nil {
		return nil, err
	}

	_, err = os.Stat(indexPath)

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
		query = bleve.NewDocIDQuery([]string{request.Query})
	}

	search := bleve.NewSearchRequest(query)
	searchResults, searchErr := index.Search(search)

	if searchErr != nil {
		return nil, searchErr
	}

	hits := searchResults.Hits

	entries := make([]types.Entry, len(hits))

	for i, x := range hits {
		request.Dictionary.EntriesByKey(&entries[i], x.ID)
	}

	return entries, nil
}
