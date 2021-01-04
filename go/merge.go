package odict

import (
	"github.com/imdario/mergo"
	"github.com/odict/odict/go/models"
)

func MergeDictionaries(dest models.Dictionary, srcs ...models.Dictionary) models.Dictionary {
	dst := dest

	for i := range srcs {
		mergo.Map(&dst, srcs[i], mergo.WithAppendSlice)
	}

	return dst
}
