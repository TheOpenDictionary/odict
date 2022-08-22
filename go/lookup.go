package odict

import (
	"regexp"
	"strings"
)

func (dict *Dictionary) lookup(query string, fallback string, split int) []Entry {
	entries := []Entry{}

	var entry Entry
	var found = dict.EntryByKey(&entry, strings.ToLower(query))

	if !found && fallback != "" {
		found = dict.EntryByKey(&entry, strings.ToLower(fallback))
	}

	if !found && split > 0 {
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
	} else if found {
		entries = append(entries, entry)
	}

	return entries
}

func (dict *Dictionary) Lookup(queries []string, split int) []Entry {
	entries := []Entry{}
	r, _ := regexp.Compile(`\((.+)\)$`)

	for _, query := range queries {
		match := r.FindAllStringSubmatch(query, -1)
		fallback := ""

		if len(match) > 0 {
			query = r.ReplaceAllString(query, "")
			fallback = match[0][1]
		}

		entries = append(entries, dict.lookup(strings.Trim(query, " "), fallback, split)...)
	}

	return entries
}
