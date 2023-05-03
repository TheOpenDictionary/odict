package main

import (
	"fmt"
	"syscall/js"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
)

var dictMap = make(map[string]*types.Dictionary)

func jsArrayToGo(jsArray js.Value) []string {
	goArray := make([]string, jsArray.Length())

	for i := 0; i < jsArray.Length(); i++ {
		goArray[i] = jsArray.Index(i).String()
	}

	return goArray
}

func loadDictionary() js.Func {
	loadFunc := js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()

		if _, ok := dictMap[name]; ok {
			return true
		}

		data := js.Global().Get("Uint8Array").New(args[1])

		dst := make([]byte, data.Get("length").Int())

		js.CopyBytesToGo(dst, data)

		dict := core.ReadDictionaryFromBytes(dst)

		dictMap[name] = dict

		return true
	})

	return loadFunc
}

func lookupWord() js.Func {
	lookupFunc := js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()

		if dict, ok := dictMap[name]; ok {
			queries := jsArrayToGo(args[1])
			split := args[2].Int()
			follow := args[3].Bool()
			entries := core.Lookup(dict, queries, split, follow)

			representable := utils.Map(entries, func(e []types.Entry) []types.EntryRepresentable {
				return utils.Map(e, func(entry types.Entry) types.EntryRepresentable {
					return entry.AsRepresentable()
				})
			})

			json := utils.SerializeToJSON(representable)

			return js.ValueOf(json)
		} else {
			fmt.Printf("Could not find any loaded dictionary called %s! Are you sure you called Dictionary.load() first?", name)
		}

		return ""
	})

	return lookupFunc
}

func getLexicon() js.Func {
	lexiconFunc := js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()

		if dict, ok := dictMap[name]; ok {
			lexicon := core.Lexicon(dict)
			return js.ValueOf(utils.SerializeToJSON(lexicon))
		} else {
			fmt.Printf("Could not find any loaded dictionary called %s! Are you sure you called Dictionary.load() first?", name)
		}

		return "[]"
	})

	return lexiconFunc
}

// func compileXML() js.Func {
// 	lexiconFunc := js.FuncOf(func(this js.Value, args []js.Value) any {
// 		name := args[0].String()
// 		xml := args[0].String()

// 		core.WriteDictionaryFromXML()
// 		if dict, ok := dictMap[name]; ok {
// 			lexicon := core.Lexicon(dict)
// 			return js.ValueOf(utils.SerializeToJSON(lexicon))
// 		} else {
// 			fmt.Printf("Could not find any loaded dictionary called %s! Are you sure you called Dictionary.load() first?", name)
// 		}

// 		return "[]"
// 	})

// 	return lexiconFunc
// }

func main() {
	js.Global().Set("odict", js.ValueOf(make(map[string]interface{})))
	module := js.Global().Get("odict")
	module.Set("loadDictionary", loadDictionary())
	module.Set("lookupWord", lookupWord())
	module.Set("getLexicon", getLexicon())
	<-make(chan bool)
}
