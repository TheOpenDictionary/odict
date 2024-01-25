package core

import (
	"github.com/TheOpenDictionary/odict/lib/types"
)

// MergeDictionaries merges the entries of two dictionaries.
func MergeDictionaries(dst *types.DictionaryBuffer, srcs ...*types.DictionaryBuffer) (*types.Dictionary, error) {
	parent := dst.Struct()

	for _, src := range srcs {
		entries := src.Struct().Entries

		for _, entry := range entries {
			if _, ok := parent.Entries[entry.Term]; !ok {
				parent.Entries[entry.Term] = entry
			}
		}
	}

	return &parent, nil
}
