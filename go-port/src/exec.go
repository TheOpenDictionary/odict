package main

import (
	"encoding/json"
	"fmt"

	"github.com/odict/odict/odict"
)

func main() {
	odict.WriteDictionary("example.xml", "example.odict")
	odict.ReadDictionary("example.odict")

	dd := map[string]int{
		"rsc": 3711,
		"r":   2138,
		"gri": 1908,
		"adg": 912,
	}

	data, err := json.Marshal(dd)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println("--------\n", string(data))
	}
}
