package main

import (
	"fmt"
	// "log"
	// "os"
	"encoding/json"
	"path/filepath"
	"strings"

	odict "github.com/Linguistic/odict/go"
	// cli "github.com/urfave/cli/v2"
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
	// app := &cli.App{
	// 	Name:  "odict",
	// 	Usage: "make an explosive entrance",
	// 	Action: func(c *cli.Context) error {
	// 		fmt.Println("boom! I say!")
	// 		return nil
	// 	},
	// }

	// err := app.Run(os.Args)

	// if err != nil {
	// 	log.Fatal(err)
	// }
	// start := time.Now()

	// createDictionary("example.xml")

	dict := odict.LoadDictionary("example.odict")

	println(dict.ID)

	res := odict.SearchDictionary(dict, "to move swiftly")

	bytes, err := json.Marshal(res[0])
	odict.Check(err)
	println(string(bytes))

	// fmt.Printf("File version: %.1f\n", float64(dict.Version))

	// elapsed := time.Since(start)

	// fmt.Printf("Completed in %.4f seconds\n", elapsed.Seconds())

	// println(dict.AsJSON())
}
