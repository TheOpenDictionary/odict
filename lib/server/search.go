package server

import (
	"encoding/json"
	"net/http"

	"github.com/TheOpenDictionary/odict/lib/search"
	"github.com/TheOpenDictionary/odict/lib/types"
)

func handleSearch(pathOrAlias string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		dictionary, err := getDictionary(pathOrAlias, r)

		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		query := r.URL.Query()
		queries := query.Get("query")
		entries, err := search.SearchDictionary(search.SearchDictionaryRequest{
			Dictionary: dictionary,
			Query:      queries,
			Exact:      false,
		})

		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		representable := types.EntriesToRepresentables(entries)

		// Return the result as JSON
		w.Header().Set("Content-Type", "application/json")

		json.NewEncoder(w).Encode(representable)
	}
}
