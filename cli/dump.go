package cli

import (
	"bufio"
	"errors"
	"os"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

type DumpFormat = string

const (
	dumpXml        DumpFormat = "xml"
	dumpPostgreSQL DumpFormat = "postgres"
	dumpSQLite     DumpFormat = "sqlite"
	dumpMySQL      DumpFormat = "mysql"
	dumpSQLServer  DumpFormat = "sqlserver"
)

func dump(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	outputFile := c.Args().Get(1)
	format := c.String("format")

	if len(inputFile) == 0 || len(outputFile) == 0 {
		return errors.New("usage: odict dump [input file] [output file]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)

		// All SQL formats and XML
		var dumped string
		if format == dumpPostgreSQL {
			dumped = dict.DumpSQL(dumpPostgreSQL)
		} else if format == dumpSQLite {
			dumped = dict.DumpSQL(dumpSQLite)
		} else if format == dumpMySQL {
			dumped = dict.DumpSQL(dumpMySQL)
		} else if format == dumpSQLServer {
			dumped = dict.DumpSQL(dumpSQLServer)
		} else {
			dumped = dict.DumpXML()
		}

		file, err := os.Create(outputFile)

		odict.Check(err)

		defer file.Close()

		writer := bufio.NewWriter(file)

		writer.Write([]byte(dumped))

		writer.Flush()
	})

	return nil
}
