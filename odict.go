package main

import (
	"fmt"
	"path/filepath"
	"strings"
	"time"

	odict "github.com/Linguistic/odict/go"
)

func getFileName(path string) string {
	base := filepath.Base(path)
	return strings.TrimSuffix(base, filepath.Ext(base))
}

func createDictionary(inputPath string) {
	name := getFileName(inputPath)
	outputPath := fmt.Sprintf("%s/%s.odict", filepath.Dir(inputPath), name)
	odict.WriteDictionary(inputPath, outputPath)
}

func main() {
	start := time.Now()

	createDictionary("example.xml")

	dict := odict.LoadDictionary("example.odict")

	res := odict.SearchDictionary(dict, "run")

	println(res)

	fmt.Printf("File version: %.1f\n", float64(dict.Version))

	elapsed := time.Since(start)

	fmt.Printf("Completed in %.4f seconds\n", elapsed.Seconds())

	// println(dict.AsJSON())
}
