package core

import (
	"regexp"
	"strings"
	"sync"

	"github.com/TheOpenDictionary/odict/lib/types"
)

type LookupRequest struct {
	Dictionary *types.Dictionary
	Queries    []string
	Follow     bool
	Split      int
}

func lookup(dict *types.Dictionary, query string, fallback string, split int, follow bool) []types.Entry {
	entries := []types.Entry{}

	var entry types.Entry
	var found = dict.EntriesByKey(&entry, query)

	if !found && fallback != "" {
		found = dict.EntriesByKey(&entry, fallback)
	}

	if found {
		var see = entry.See()

		if len(see) > 0 && follow {
			return lookup(dict, string(see), fallback, split, follow)
		}

		entries = append(entries, entry)
	} else if split > 0 {
		entries = append(entries, Split(
			SplitRequest{
				Dictionary: dict,
				Query:      query,
				Threshold:  split,
			},
		)...)
	}

	return entries
}

func Lookup(request LookupRequest) [][]types.Entry {
	entries := make([][]types.Entry, len(request.Queries))
	r, _ := regexp.Compile(`\((.+)\)$`)

	// Create a channel to receive results
	resultChan := make(chan struct {
		index   int
		entries []types.Entry
	})

	var wg sync.WaitGroup

	for i, query := range request.Queries {
		wg.Add(1)

		go func(idx int, query string) {
			defer wg.Done()

			match := r.FindAllStringSubmatch(query, -1)
			fallback := ""

			if len(match) > 0 {
				query = r.ReplaceAllString(query, "")
				fallback = match[0][1]
			}

			resultChan <- struct {
				index   int
				entries []types.Entry
			}{idx, lookup(request.Dictionary, strings.Trim(query, " "), fallback, request.Split, request.Follow)}
		}(i, query)
	}

	go func() {
		wg.Wait()
		close(resultChan)
	}()

	// Collect and order results from the channel
	for result := range resultChan {
		entries[result.index] = result.entries
	}

	return entries
}
