package odict

import (
	"github.com/imdario/mergo"
)

// type entryTransformer struct{}

// func (e entryTransformer) Transformer(typ reflect.Type) func(dst, src reflect.Value) error {
// 	if typ == reflect.TypeOf(DictionaryEntry{}) {
// 		return func(dst, src reflect.Value) error {
// 			term := dst.FieldByName("Term")

// 			if dst.CanSet() {
// 				isZero := dst.MethodByName("IsZero")
// 				result := isZero.Call([]reflect.Value{})
// 				if result[0].Bool() {
// 					dst.Set(src)
// 				}
// 			}
// 			return nil
// 		}
// 	}
// 	return nil
// }

func MergeDictionaries(dest Dictionary, srcs ...Dictionary) Dictionary {
	dst := dest

	for i := range srcs {
		mergo.Map(&dst, srcs[i], mergo.WithAppendSlice)
	}

	return dst
}
