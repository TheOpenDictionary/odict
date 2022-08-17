package odict

import (
	"encoding/json"
)

func lookup(dict Dictionary, query string, splitThreshold int) string {
	entries := []Entry{}

	if dict.Entries.Has(query) {
		entries = []Entry{dict.Entries.Get(query)}
	} else if splitThreshold > 0 {
		start := 0
		end := len(query)

		for ok := true; ok; ok = start < end {
			substr := query[start:end]

			if dict.Entries.Has(substr) && len(substr) >= splitThreshold {
				entry := dict.Entries.Get(query[start:end])
				entries = append(entries, entry)
				start = end
				end = len(query)
			} else {
				end--
			}
		}
	}

	b, err := json.MarshalIndent(&entries, "", " ")

	Check(err)

	return string(b)
}
