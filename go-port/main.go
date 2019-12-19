package main

import (
	"fmt"

	odict "github.com/odict/odict/api"
)

func main() {
	odict.WriteDictionary("example.xml", "example.odict")
	dict := odict.LoadDictionary("example.odict")

	fmt.Printf("ODict Version %d\n", dict.Version)

	println(dict.Entries[1].Term)
	// println(dict.AsJSON())
}
