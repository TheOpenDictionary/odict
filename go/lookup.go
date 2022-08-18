package odict

import (
	"encoding/json"
)

func Lookup(dict Dictionary, query string, split int) string {
	entries := []Entry{}

	if dict.Entries.Has(query) {
		entries = []Entry{dict.Entries.Get(query)}
	} else if split > 0 {
		start := 0
		end := len(query)

		for ok := true; ok; ok = start < end {
			substr := query[start:end]

			if dict.Entries.Has(substr) && len(substr) >= split {
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
