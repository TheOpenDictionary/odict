package main

import (
	"fmt"
	"syscall/js"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
)

var dictMap = make(map[string]*types.DictionaryBuffer)

func jsArrayToGo(jsArray js.Value) []string {
	goArray := make([]string, jsArray.Length())

	for i := 0; i < jsArray.Length(); i++ {
		goArray[i] = jsArray.Index(i).String()
	}

	return goArray
}

func byteArrayToJS(data []byte) js.Value {
	length := len(data)
	uint8Array := js.Global().Get("Uint8Array").New(length)
	arrayBuffer := uint8Array.Get("buffer")
	buffer := js.Global().Get("Uint8Array").New(arrayBuffer)

	js.CopyBytesToJS(buffer, data)

	return uint8Array
}

func load() js.Func {
	f := js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()

		if _, ok := dictMap[name]; ok {
			return true
		}

		data := js.Global().Get("Uint8Array").New(args[1])

		dst := make([]byte, data.Get("length").Int())

		js.CopyBytesToGo(dst, data)

		dict, err := core.ReadDictionaryFromBytes(dst)

		if err != nil {
			fmt.Printf("Error reading dictionary: %s", err)
			return false
		}

		dictMap[name] = dict

		return true
	})

	return f
}

func lookup() js.Func {
	f := js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()

		if dict, ok := dictMap[name]; ok {
			queries := jsArrayToGo(args[1])

			split := args[2].Int()

			follow := args[3].Bool()

			entries := core.Lookup(core.LookupRequest{
				Dictionary: dict,
				Queries:    queries,
				Split:      split,
				Follow:     follow,
			})

			s := types.NestedEntriesStructs(entries)

			json, err := utils.SerializeToJSON(s, false)

			if err != nil {
				fmt.Printf("Error serializing lookup result: %s", err)
			}

			return js.ValueOf(json)
		} else {
			fmt.Printf("Could not find any loaded dictionary called %s! Are you sure you called Dictionary.load() first?", name)
		}

		return ""
	})

	return f
}

func lexicon() js.Func {
	f := js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()

		if dict, ok := dictMap[name]; ok {
			lexicon := core.Lexicon(dict)
			serialized, err := utils.SerializeToJSON(lexicon, false)

			if err != nil {
				fmt.Printf("Error serializing lexicon: %s", err)
			}

			return js.ValueOf(serialized)
		} else {
			fmt.Printf("Could not find any loaded dictionary called %s! Are you sure you called Dictionary.load() first?", name)
		}

		return "[]"
	})

	return f
}

func split() js.Func {
	f := js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()
		query := args[1].String()
		threshold := args[2].Int()

		if dict, ok := dictMap[name]; ok {
			result := core.Split(core.SplitRequest{Dictionary: dict, Query: query, Threshold: threshold})
			serialized, err := utils.SerializeToJSON(types.EntriesStructs(result), false)

			if err != nil {
				fmt.Printf("Error serializing split result: %s", err)
			}

			return js.ValueOf(serialized)
		} else {
			fmt.Printf("Could not find any loaded dictionary called %s! Are you sure you called Dictionary.load() first?", name)
		}

		return "[]"
	})

	return f
}

func compile() js.Func {
	compileFunc := js.FuncOf(func(this js.Value, args []js.Value) any {
		xml := args[0].String()
		bytes, err := core.CompileXML(xml)

		if err != nil {
			fmt.Printf("Error compiling dictionary: %s", err)
		}

		return byteArrayToJS(bytes)
	})

	return compileFunc
}

func main() {
	js.Global().Set("odict", js.ValueOf(make(map[string]interface{})))
	module := js.Global().Get("odict")
	module.Set("load", load())
	module.Set("lookup", lookup())
	module.Set("lexicon", lexicon())
	module.Set("split", split())
	module.Set("compile", compile())
	<-make(chan bool)
}
