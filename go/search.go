package odict

import (
	"bytes"
	"encoding/gob"
	"os"

	"github.com/blevesearch/bleve"
)

// SearchDictionary searches a dictionary model using Bleve
func SearchDictionary(dictionary Dictionary, queryStr string) []Entry {
	indexPath := getIndexPath(dictionary)
	_, err := os.Stat(indexPath)

	if os.IsNotExist(err) {
		panic("Index path does not exist. Did you call IndexDictionary() first?")
	}

	index, openErr := bleve.Open(indexPath)

	defer index.Close()

	Check(openErr)

	query := bleve.NewMatchQuery(queryStr)
	search := bleve.NewSearchRequest(query)
	searchResults, searchErr := index.Search(search)

	Check(searchErr)

	hits := searchResults.Hits

	entries := make([]Entry, len(hits))

	for i := range hits {
		hitID := hits[i].ID
		b, err := index.GetInternal([]byte(hitID))

		if err != nil {
			panic(err)
		}

		var entry Entry

		buffer := bytes.NewBuffer(b)
		dec := gob.NewDecoder(buffer)
		decodingErr := dec.Decode(&entry)

		if decodingErr != nil {
			panic(err)
		}

		entries[i] = entry
	}

	return entries
}
