package main

import (
	"log"
	"os"

	"github.com/TheOpenDictionary/odict/cli"
	"github.com/TheOpenDictionary/odict/lib/validator"
)

func main() {
	validator.InitializeValidator()

	defer validator.CleanupValidator()

	err := cli.App.Run(os.Args)

	if err != nil {
		log.Fatal(err)
	}
}
