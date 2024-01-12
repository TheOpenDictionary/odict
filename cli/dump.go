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
	XML       DumpFormat = "xml"
	Postgres  DumpFormat = "postgres"
	SQLite    DumpFormat = "sqlite"
	MySQL     DumpFormat = "mysql"
	SQLServer DumpFormat = "sqlserver"
)

func dump(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	outputFile := c.Args().Get(1)
	format := c.String("format")
	includeSchema := !c.Bool("no-schema")

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
		var err error

		if format == XML {
			dumped, err = dump_.AsXML(dict)
		} else {
			dumped, err = dump_.AsSQL(dict, format, includeSchema)
		}

		if err != nil {
			return err
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
