package odict

import (
	"os"

	"github.com/blevesearch/bleve"
)

func SearchDictionary(dictionary OpenDictionary, queryStr string) {
	indexPath := getIndexPath(dictionary)
	_, err := os.Stat(indexPath)

	if os.IsNotExist(err) {
		panic("Index path does not exist. Did you call LoadDictionary() first?")
	}

	index, err := bleve.Open(indexPath)

	query := bleve.NewMatchQuery(queryStr)
	search := bleve.NewSearchRequest(query)
	searchResults, err := index.Search(search)

	Check(err)

	hitIDs := Map(searchResults.Hits, func(hit search.DocumentMatchCollection) {

	})
	return Filter(dictionary.Entries, func(entry OpenDictionaryEntry) {
		entry.ID
	})
	print(string(t))
	// searchResults.Hits[0].
	println(searchResults.Total)

}
