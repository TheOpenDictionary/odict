package core

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/imdario/mergo"
)

// MergeDictionaries merges the entries of two dictionaries.
func MergeDictionaries(dest *types.Dictionary, srcs ...*types.Dictionary) types.DictionaryRepresentable {
	dst := dest.AsRepresentable()

	for i := range srcs {
		src := srcs[i].AsRepresentable()
		if err := mergo.Map(&dst, src, mergo.WithAppendSlice); err != nil {
			utils.Check(err)
		}
	}

	return dst
}
