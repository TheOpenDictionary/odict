package core

import "github.com/TheOpenDictionary/odict/lib/types"

// Split splits a query into definable tokens greater than or equal to the specified threshold
// Example: household -> house + hold
func Split(dict *types.Dictionary, query string, threshold int) []types.Entry {
	entries := []types.Entry{}

	var entry types.Entry
	var found bool

	start := 0
	end := len(query)

	for ok := true; ok; ok = start < end {
		substr := query[start:end]
		found = dict.EntriesByKey(&entry, substr)

		if found {
			entries = append(entries, entry)
		}

		if found || len(substr) <= threshold {
			start = end
			end = len(query)
		} else {
			end--
		}

	}

	return entries
}
