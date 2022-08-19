package odict

import (
	"github.com/imdario/mergo"
)

// MergeDictionaries merges the entries of two dictionaries.
func MergeDictionaries(dest Dictionary, srcs ...Dictionary) Dictionary {
	dst := dest

	for i := range srcs {
		if err := mergo.Map(&dst, srcs[i], mergo.WithAppendSlice); err != nil {
			Check(err)
		}
	}

	return dst
}
