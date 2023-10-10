package cli

import (
	"github.com/TheOpenDictionary/odict/lib/server"
	cli "github.com/urfave/cli/v2"
)

func serve(c *cli.Context) error {
	return server.StartServer(c.Args().First(), c.Int("port"))
}
