package cli

import (
	"bufio"
	"errors"
	"os"

	"github.com/TheOpenDictionary/odict/lib/core"
	dump_ "github.com/TheOpenDictionary/odict/lib/dump"
	"github.com/TheOpenDictionary/odict/lib/utils"
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
		dict := core.ReadDictionaryFromPath(inputFile)

		// All SQL formats and XML
		var dumped string
		switch format {
		case Xml:
			dumped = dump_.AsXML(dict)
		case Postgres:
			dumped = dump_.AsSQL(dict, Postgres)
		case Sqlite:
			dumped = dump_.AsSQL(dict, Sqlite)
		case Mysql:
			dumped = dump_.AsSQL(dict, Mysql)
		case Sqlserver:
			dumped = dump_.AsSQL(dict, Sqlserver)
		}

		file, err := os.Create(outputFile)

		utils.Check(err)

		defer file.Close()

		writer := bufio.NewWriter(file)

		writer.Write([]byte(dumped))

		writer.Flush()
	})

	return nil
}
