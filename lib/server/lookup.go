package server

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/samber/lo"
)

func handleLookup(pathOrAlias string) {
	http.HandleFunc("/lookup", func(w http.ResponseWriter, r *http.Request) {
		dictionary, err := getDictionary(pathOrAlias, r)

		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		query := r.URL.Query()
		queries := query["query"]
		follow := query.Has("follow")
		split := 0

		if r.URL.Query().Has("split") {
			split, err = strconv.Atoi(r.URL.Query().Get("split"))

			if err != nil {
				http.Error(w, err.Error(), http.StatusBadRequest)
				return
			}
		}

		entries := core.Lookup(core.LookupRequest{
			Dictionary: dictionary,
			Queries:    queries,
			Split:      split,
			Follow:     follow,
		})

		representable := lo.Map(entries, func(e []types.Entry, _ int) []types.EntryRepresentable {
			return lo.Map(e, func(entry types.Entry, _ int) types.EntryRepresentable {
				return entry.AsRepresentable()
			})
		})

		// Return the result as JSON
		w.Header().Set("Content-Type", "application/json")

		json.NewEncoder(w).Encode(representable)
	})
}
