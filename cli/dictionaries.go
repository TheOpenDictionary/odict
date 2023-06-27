package cli

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"path"

	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

func getDictionariesFile() string {
	return path.Join(GetConfigDir(), "dictionaries.json")
}

func getDictionariesConfig() map[string]string {
	dictionariesFile := getDictionariesFile()

	dictionariesFileBytes, err := ioutil.ReadFile(dictionariesFile)

	if os.IsNotExist(err) {
		writeErr := ioutil.WriteFile(dictionariesFile, []byte("{}"), 0644)
		utils.Check(writeErr)
		return map[string]string{}
	}

	utils.Check(err)

	var dictionaryMap map[string]string

	json.Unmarshal(dictionariesFileBytes, &dictionaryMap)

	return dictionaryMap
}

func writeDictionaryConfig(config map[string]string) {
	dictionariesFile := getDictionariesFile()

	configBytes, err := json.Marshal(config)

	utils.Check(err)

	writeErr := ioutil.WriteFile(dictionariesFile, configBytes, 0644)

	utils.Check(writeErr)
}

func listDictionaries(c *cli.Context) {

}

func addDictionary(c *cli.Context) error {
	dictionaries := getDictionariesConfig()
	name := c.Args().First()
	path := c.Args().Get(1)

	if len(name) == 0 || len(path) == 0 {
		cli.ShowSubcommandHelpAndExit(c, 1)
	}

	if _, ok := dictionaries[name]; ok {
		return cli.Exit("A dictionary alias with that name already exists!", 1)
	}

	dictionaries[name] = path

	writeDictionaryConfig(dictionaries)

	fmt.Printf("Aliased \"%s\" to the dictionary at %s.\n", name, path)

	return nil
}

func removedDictionary(c *cli.Context) error {
	dictionaries := getDictionariesConfig()
	name := c.Args().First()

	if len(name) == 0 {
		cli.ShowSubcommandHelpAndExit(c, 1)
	}

	if _, ok := dictionaries[name]; !ok {
		return cli.Exit("No dictionary alias with that name exists!", 1)
	}

	delete(dictionaries, name)

	writeDictionaryConfig(dictionaries)

	return nil
}
