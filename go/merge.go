package odict

import (
	"encoding/json"
	"github.com/imdario/mergo"
)

// MergeDictionaries merges the entries of two dictionaries.
// If two of the same entry exist, the additional etymologies
// are added to the entry of the source dictionary
func MergeDictionaries(dest Dictionary, srcs ...Dictionary) Dictionary {
	dst := dest

	for i := range srcs {
		if err := mergo.Map(&dst, srcs[i], mergo.WithAppendSlice); err != nil {
			Check(err)
		}
	}

	b, err := json.MarshalIndent(&dst, "", " ")

	Check(err)

	print(string(b))

	return dst
}
