package odict

// Split splits a query into definable tokens greater than or equal to the specified threshold
// Example: household -> house + hold
func (dict *Dictionary) Split(query string, threshold int) []Entry {
	entries := []Entry{}

	var entry Entry
	var found bool

	start := 0
	end := len(query)

	for ok := true; ok; ok = start < end {
		substr := query[start:end]
		found = dict.EntryByKey(&entry, substr)

		if found && len(substr) >= threshold {
			entries = append(entries, entry)
			start = end
			end = len(query)
		} else {
			end--
		}
	}

	return entries
}
