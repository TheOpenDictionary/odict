package cli

import (
	"bufio"
	"errors"
	"fmt"
	"os"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

type DumpFormat = string

const (
	dumpXml DumpFormat = "xml"
	dumpSql DumpFormat = "sql"
)

func dump(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	outputFile := c.Args().Get(1)
	format := c.String("format")

	fmt.Println(format)

	if len(inputFile) == 0 || len(outputFile) == 0 {
		return errors.New("usage: odict dump [input file] [output file]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)
		dumped := dict.DumpXML()
		file, err := os.Create(outputFile)

		odict.Check(err)

		defer file.Close()

		writer := bufio.NewWriter(file)

		writer.Write([]byte(dumped))

		writer.Flush()
	})

	return nil
}
