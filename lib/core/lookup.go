package core

import (
	"regexp"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
)

func lookup(dict *types.Dictionary, query string, fallback string, split int, follow bool) []types.Entry {
	entries := []types.Entry{}

	var entry types.Entry
	var found = dict.EntriesByKey(&entry, query)

	if !found && fallback != "" {
		found = dict.EntriesByKey(&entry, fallback)
	}

	if found {
		var see = entry.See()

		if len(see) > 0 && follow {
			return lookup(dict, string(see), fallback, split, follow)
		}

		entries = append(entries, entry)
	} else if split > 0 {
		entries = append(entries, Split(dict, query, split)...)
	}

	return entries
}

func Lookup(dict *types.Dictionary, queries []string, split int, follow bool) [][]types.Entry {
	entries := [][]types.Entry{}
	r, _ := regexp.Compile(`\((.+)\)$`)

	for _, query := range queries {
		match := r.FindAllStringSubmatch(query, -1)
		fallback := ""

		if len(match) > 0 {
			query = r.ReplaceAllString(query, "")
			fallback = match[0][1]
		}

		entries = append(entries, lookup(dict, strings.Trim(query, " "), fallback, split, follow))
	}

	return entries
}
