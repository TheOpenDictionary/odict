package core

import (
	"github.com/TheOpenDictionary/odict/lib/types"
)

// MergeDictionaries merges the entries of two dictionaries.
func MergeDictionaries(dst *types.Dictionary, srcs ...*types.Dictionary) (*types.DictionaryRepresentable, error) {
	parent := dst.AsRepresentable()

	for _, src := range srcs {
		entries := src.AsRepresentable().Entries

		for _, entry := range entries {
			if _, ok := parent.Entries[entry.Term]; !ok {
				parent.Entries[entry.Term] = entry
			}
		}
	}

	return &parent, nil
}
