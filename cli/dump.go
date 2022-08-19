package cli

import (
	"bufio"
	"errors"
	"os"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func dump(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	outputFile := c.Args().Get(1)

	if len(inputFile) == 0 || len(outputFile) == 0 {
		return errors.New("Usage: odict dump [input file] [output file]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)
		dumped := odict.DumpDictionary(dict)
		file, err := os.Create(outputFile)

		odict.Check(err)

		defer file.Close()

		writer := bufio.NewWriter(file)

		writer.Write([]byte(dumped))

		writer.Flush()
	})

	return nil
}
