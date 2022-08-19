package odict

func lookup(dict Dictionary, query string, split int) []Entry {
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

	return entries
}

func Lookup(dict Dictionary, queries []string, split int) []Entry {
	entries := []Entry{}

	for _, query := range queries {
		entries = append(entries, lookup(dict, query, split)...)
	}

	return entries
}
