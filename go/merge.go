package odict

import (
	"github.com/imdario/mergo"
)

// MergeDictionaries merges the entries of two dictionaries.
func MergeDictionaries(dest *Dictionary, srcs ...*Dictionary) DictionaryRepresentable {
	dst := dest.AsRepresentable()

	for i := range srcs {
		src := srcs[i].AsRepresentable()
		if err := mergo.Map(&dst, src, mergo.WithAppendSlice); err != nil {
			Check(err)
		}
	}

	return dst
}
