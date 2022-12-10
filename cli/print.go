package cli

import (
	"fmt"
	"strings"

	odict "github.com/TheOpenDictionary/odict/go"
	"github.com/fatih/color"
)

var title = color.New(color.Bold)
var italic = color.New(color.Italic, color.Faint)
var italicUnderline = color.New(color.Italic, color.Faint, color.Underline)
var etyTitle = color.New(color.Bold, color.Underline)

type PrintFormat = string

const (
	json PrintFormat = "json"
	xml  PrintFormat = "xml"
	pp   PrintFormat = "pp"
)

func prettyPrint(entries [][]odict.EntryRepresentable) {
	fmt.Println()

	for _, entry := range entries {
		for _, subentry := range entry {

			Divider(32)
			title.Printf("%s\n", subentry.Term)
			Divider(32)

			etys := subentry.Etymologies

			for eidx, ety := range etys {
				etyTitle.Printf("\nEtymology #%d\n", eidx+1)

				for _, usage := range ety.Usages {
					italic.Printf("\n%s\n\n", usage.POS.Name())

					var i = 0

					for i < len(usage.Definitions) {
						def := usage.Definitions[i]

						fmt.Printf("  %d. %s\n", i+1, def.Value)

						for _, example := range def.Examples {
							term := subentry.Term
							fmt.Print("     ")
							start := strings.Index(strings.ToLower(example), strings.ToLower(term))

							if start >= 0 {
								end := start + len(term)
								italic.Print(example[0:start])
								italicUnderline.Print(example[start:end])
								italic.Print(example[end:])
								fmt.Println()
							} else {
								italic.Printf("%s\n", example)
							}
						}
						i++
					}
				}
			}

			fmt.Print("\n\n")
		}

	}
}

func PrintEntries(entries [][]odict.EntryRepresentable, format PrintFormat) {
	switch {
	case format == "json":
		fmt.Print(odict.JSON(entries))
	case format == "xml":
		fmt.Print(odict.XML(entries))
	case format == "pp":
		prettyPrint(entries)
	}
}
