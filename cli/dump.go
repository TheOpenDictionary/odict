package cli

import (
	"bufio"
	"errors"
	"os"

	odict "github.com/TheOpenDictionary/odict/lib"
	cli "github.com/urfave/cli/v2"
)

type DumpFormat = string

const (
	Xml       DumpFormat = "xml"
	Postgres  DumpFormat = "postgres"
	Sqlite    DumpFormat = "sqlite"
	Mysql     DumpFormat = "mysql"
	Sqlserver DumpFormat = "sqlserver"
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
		switch format {
		case Xml:
			dumped = dict.DumpXML()
		case Postgres:
			dumped = dict.DumpSQL(Postgres)
		case Sqlite:
			dumped = dict.DumpSQL(Sqlite)
		case Mysql:
			dumped = dict.DumpSQL(Mysql)
		case Sqlserver:
			dumped = dict.DumpSQL(Sqlserver)
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
