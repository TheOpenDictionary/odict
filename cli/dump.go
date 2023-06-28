package cli

import (
	"bufio"
	"errors"
	"os"

	"github.com/TheOpenDictionary/odict/lib/core"
	dump_ "github.com/TheOpenDictionary/odict/lib/dump"
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

	return t(c, func() error {
		dict, readErr := core.ReadDictionary(inputFile)

		if readErr != nil {
			return readErr
		}

		// All SQL formats and XML
		var dumped string
		var dumpErr error

		switch format {
		case Xml:
			dumped, dumpErr = dump_.AsXML(dict)
		case Postgres:
			dumped, dumpErr = dump_.AsSQL(dict, Postgres)
		case Sqlite:
			dumped, dumpErr = dump_.AsSQL(dict, Sqlite)
		case Mysql:
			dumped, dumpErr = dump_.AsSQL(dict, Mysql)
		case Sqlserver:
			dumped, dumpErr = dump_.AsSQL(dict, Sqlserver)
		}

		if dumpErr != nil {
			return dumpErr
		}

		file, writeErr := os.Create(outputFile)

		if writeErr != nil {
			return writeErr
		}

		defer file.Close()

		writer := bufio.NewWriter(file)

		writer.Write([]byte(dumped))

		writer.Flush()

		return nil
	})
}
