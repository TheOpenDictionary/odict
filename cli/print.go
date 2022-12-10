package cli

import (
	"fmt"

	odict "github.com/TheOpenDictionary/odict/go"
	"github.com/fatih/color"
)

var title = color.New(color.Bold)
var pos = color.New(color.Italic, color.Faint)
var etyTitle = color.New(color.Bold, color.Underline)

type PrintFormat = string

const (
	json PrintFormat = "json"
	xml  PrintFormat = "xml"
	pp   PrintFormat = "pp"
)

func prettyPrint(entries [][]odict.EntryRepresentable) {
	for _, entry := range entries {
		for _, subentry := range entry {

			title.Printf("%s\n", subentry.Term)

			Divider(32)

			etys := subentry.Etymologies

			for eidx, ety := range etys {
				etyTitle.Printf("\nEtymology #%d\n\n", eidx+1)

				for _, usage := range ety.Usages {
					pos.Printf("%s\n\n", usage.POS.Name())

					var i = 0

					for i < len(usage.Definitions) {
						fmt.Printf("  %d. %s\n", i+1, usage.Definitions[i])
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
