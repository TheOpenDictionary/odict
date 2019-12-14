package main

import (
	"odict/dictionary"
)

func main() {
	dictionary.WriteDictionary("example.xml", "example.odict")
	dictionary.ReadDictionary("example.odict")
}
