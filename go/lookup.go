package odict

import (
	"regexp"
	"strings"
)

func (dict *Dictionary) lookup(query string, fallback string, split int) []Entry {
	entries := []Entry{}

	var entry Entry
	var found = dict.EntriesByKey(&entry, strings.ToLower(query))

	if !found && fallback != "" {
		found = dict.EntriesByKey(&entry, strings.ToLower(fallback))
	}

	if !found && split > 0 {
		entries = append(entries, dict.Split(query, split)...)
	} else if found {
		entries = append(entries, entry)
	}

	return entries
}

func (dict *Dictionary) Lookup(queries []string, split int) [][]Entry {
	entries := [][]Entry{}
	r, _ := regexp.Compile(`\((.+)\)$`)

	for _, query := range queries {
		match := r.FindAllStringSubmatch(query, -1)
		fallback := ""

		if len(match) > 0 {
			query = r.ReplaceAllString(query, "")
			fallback = match[0][1]
		}

		entries = append(entries, dict.lookup(strings.Trim(query, " "), fallback, split))
	}

	return entries
}
