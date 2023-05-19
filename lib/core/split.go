package core

import "github.com/TheOpenDictionary/odict/lib/types"

type SplitRequest struct {
	Dictionary *types.Dictionary
	Query      string
	Threshold  int
}

// Split splits a query into definable tokens greater than or equal to the specified threshold
// Example: household -> house + hold
func Split(request SplitRequest) []types.Entry {
	entries := []types.Entry{}

	var entry types.Entry
	var found bool

	start := 0
	end := len(request.Query)

	for ok := true; ok; ok = start < end {
		substr := request.Query[start:end]
		found = request.Dictionary.EntriesByKey(&entry, substr)

		if found {
			entries = append(entries, entry)
		}

		if found || len(substr) <= request.Threshold {
			start = end
			end = len(request.Query)
		} else {
			end--
		}

	}

	return entries
}
