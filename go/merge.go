package odict

import (
	"github.com/imdario/mergo"
)

// MergeDictionaries merges the entries of two dictionaries.
// If two of the same entry exist, the additional etymologies
// are added to the entry of the source dictionary
func MergeDictionaries(dest Dictionary, srcs ...Dictionary) Dictionary {
	dst := dest

	for i := range srcs {
		mergo.Map(&dst, srcs[i], mergo.WithAppendSlice)
	}

	return dst
}
