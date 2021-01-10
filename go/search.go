package odict

import (
	"bytes"
	"encoding/gob"
	"os"

	"github.com/blevesearch/bleve"
	"github.com/odict/odict/go/models"
)

// SearchDictionary searches a dictionary model using Bleve
func SearchDictionary(dictionary models.Dictionary, queryStr string) []models.Entry {
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

	entries := make([]models.Entry, len(hits))

	for i := range hits {
		hitID := hits[i].ID
		b, err := index.GetInternal([]byte(hitID))

		if err != nil {
			panic(err)
		}

		var entry models.Entry

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
