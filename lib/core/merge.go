package core

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/imdario/mergo"
)

// MergeDictionaries merges the entries of two dictionaries.
func MergeDictionaries(dest *types.Dictionary, srcs ...*types.Dictionary) (*types.DictionaryRepresentable, error) {
	dst := dest.AsRepresentable()

	for i := range srcs {
		src := srcs[i].AsRepresentable()
		if err := mergo.Map(&dst, src, mergo.WithAppendSlice); err != nil {
			return nil, err
		}
	}

	return &dst, nil
}
