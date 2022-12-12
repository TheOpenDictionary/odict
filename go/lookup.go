package odict

import (
	"regexp"
	"strings"
)

func (dict *Dictionary) lookup(query string, fallback string, split int, follow bool) []Entry {
	entries := []Entry{}

	var entry Entry
	var found = dict.EntriesByKey(&entry, strings.ToLower(query))

	if found {
		var see = entry.See()

		if len(see) > 0 && follow {
			return dict.lookup(string(see), fallback, split, follow)
		}
	} else if fallback != "" {
		found = dict.EntriesByKey(&entry, strings.ToLower(fallback))
	}

	if found {
		entries = append(entries, entry)
	} else if split > 0 {
		entries = append(entries, dict.Split(query, split)...)
	}

	return entries
}

func (dict *Dictionary) Lookup(queries []string, split int, follow bool) [][]Entry {
	entries := [][]Entry{}
	r, _ := regexp.Compile(`\((.+)\)$`)

	for _, query := range queries {
		match := r.FindAllStringSubmatch(query, -1)
		fallback := ""

		if len(match) > 0 {
			query = r.ReplaceAllString(query, "")
			fallback = match[0][1]
		}

		entries = append(entries, dict.lookup(strings.Trim(query, " "), fallback, split, follow))
	}

	return entries
}
