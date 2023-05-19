package cli

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"

	cli "github.com/urfave/cli/v2"
)

type Request struct {
	Args string `json:"input"`
}

func service(c *cli.Context) error {
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		var args []string

		if err := json.Unmarshal([]byte(scanner.Text()), &args); err != nil {
			fmt.Println(err)
			continue
		}

		fmt.Println(args)

		c.App.Run(args)
	}

	return nil
}
