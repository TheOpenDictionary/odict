package odict

func (dict *Dictionary) lookup(query string, split int) []Entry {
	entries := []Entry{}

	var entry Entry
	var found = dict.EntryByKey(&entry, query)

	if found {
		entries = append(entries, entry)
	} else if split > 0 {
		start := 0
		end := len(query)

		for ok := true; ok; ok = start < end {
			substr := query[start:end]
			found = dict.EntryByKey(&entry, substr)

			if found && len(substr) >= split {
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

func (dict *Dictionary) Lookup(queries []string, split int) []Entry {
	entries := []Entry{}

	for _, query := range queries {
		entries = append(entries, dict.lookup(query, split)...)
	}

	return entries
}
