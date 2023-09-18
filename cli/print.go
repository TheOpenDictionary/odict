//go:build !js && !wasm

package cli

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/fatih/color"
)

var fmtFaint = color.New(color.Faint).SprintfFunc()
var fmtEtymology = color.New(color.Underline, color.Bold).SprintfFunc()
var fmtExample = color.New(color.Faint, color.Italic).SprintfFunc()
var fmtExampleTarget = color.New(color.Faint, color.Underline, color.Italic).SprintfFunc()
var fmtParenthetical = color.New(color.Italic).SprintfFunc()
var fmtNoteTitle = color.New(color.Underline, color.Bold).SprintfFunc()
var fmtPartOfSpeech = color.New(color.Italic).SprintfFunc()
var fmtEntry = color.New(color.Bold).SprintfFunc()

var parentheticalRegex = regexp.MustCompile(`^(\(.*?\))`)

type PrintFormat = string

const (
	jsonFormat  PrintFormat = "json"
	xmlFormat   PrintFormat = "xml"
	printFormat PrintFormat = "print"
)

func printNewLine() {
	fmt.Println()
}

func printExample(example string, targetWord string, indent int) {
	start := strings.Index(strings.ToLower(example), strings.ToLower(targetWord))
	caret := fmtExample("▸")

	if start >= 0 {
		end := start + len(targetWord)
		before := fmtExample(example[0:start])
		after := fmtExample(example[end:])
		target := fmtExampleTarget(example[start:end])

		fmt.Printf("%*s%s %s%s%s\n", indent, "", caret, before, target, after)
	} else {
		fmt.Printf("%*s%s %s\n", indent, "", caret, fmtExample(example))
	}

}

func printDivider() {
	fmt.Println(strings.Repeat("─", 32))
}

func printNote(note types.NoteRepresentable, targetWord string, numbering string, indent int) {
	fmtNumbering := "%" + fmt.Sprint(indent) + "s."

	fmt.Printf(fmtNumbering+" %s\n", numbering, note.Value)

	for _, example := range note.Examples {
		printExample(example, targetWord, indent+2)
	}

	fmt.Println()
}

func printDefinition(definition types.DefinitionRepresentable, numbering string, entry types.EntryRepresentable, indent int) {
	value := definition.Value
	matches := parentheticalRegex.FindAllStringIndex(value, -1)
	fmtNumbering := "%" + fmt.Sprint(indent) + "s."

	if len(matches) > 0 {
		j := 0

		fmt.Printf(fmtNumbering+" %s", numbering, value[0:matches[0][0]])

		for i := 0; i < len(matches); i += 1 {
			start := matches[i][0]
			end := matches[i][1]

			fmt.Printf("%s%s", value[j:start], fmtParenthetical(value[start:end]))

			j = end
		}

		fmt.Printf("%s\n", value[j:])
	} else {
		fmt.Printf(fmtNumbering+" %s\n", numbering, value)
	}

	for _, example := range definition.Examples {
		printExample(example, entry.Term, indent+2)
	}

	if len(definition.Notes) > 0 {
		fmt.Printf("\n%*s%s\n\n", indent+2, "", fmtNoteTitle("Notes"))
	}

	for j, note := range definition.Notes {
		printNote(note, entry.Term, string(rune('a'+j)), indent+4)
	}
}

func printGroup(group types.GroupRepresentable, i int, entry types.EntryRepresentable) {
	fmt.Printf("%4d. %s\n", i+1, group.Description)

	for j, definition := range group.Definitions {
		printDefinition(definition, string(rune('a'+j)), entry, 7)
	}
}

func printSense(sense types.SenseRepresentable, entry types.EntryRepresentable) {
	fmt.Printf("\n%s\n\n", fmtPartOfSpeech(sense.POS.Label))

	var i = 0

	for i < len(sense.Groups) {
		printGroup(sense.Groups[i], i, entry)
		i++
	}

	for i < len(sense.Definitions) {
		printDefinition(sense.Definitions[i], strconv.Itoa(i+1), entry, 4)
		i++
	}
}

func printEty(ety types.EtymologyRepresentable, i int, showTitle bool, entry types.EntryRepresentable) {
	if showTitle {
		fmt.Printf("%s\n", fmtEtymology("Etymology #%d", i+1))
	}

	if len(ety.Description) > 0 {
		fmt.Println(ety.Description)
	}

	for _, sense := range ety.Senses {
		printSense(sense, entry)
	}
}

func printEntry(entry types.EntryRepresentable) {
	printDivider()

	fmt.Println(fmtEntry(entry.Term))

	printDivider()

	fmt.Println()

	etys := entry.Etymologies

	for i, ety := range etys {
		printEty(ety, i, len(etys) > 1, entry)
	}

}

func prettyPrint(entries [][]types.EntryRepresentable) error {
	fmt.Println()

	hasEntries := false

	for _, entry := range entries {
		for _, subentry := range entry {
			hasEntries = true
			printEntry(subentry)
		}
	}

	if !hasEntries {
		return fmt.Errorf("no entries found")
	}

	return nil
}

func PrintEntries(entries [][]types.EntryRepresentable, format PrintFormat, indent bool) error {
	switch {
	case format == "json":
		json, err := utils.SerializeToJSON(entries, indent)

		if err != nil {
			return err
		}

		fmt.Print(json)
	case format == "xml":
		xml, err := utils.SerializeToXML(entries, indent)

		if err != nil {
			return err
		}

		fmt.Print(xml)
	case format == "print":
		err := prettyPrint(entries)

		if err != nil {
			return err
		}
	}

	return nil
}
