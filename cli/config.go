package cli

import (
	"os"
	"path"

	"github.com/TheOpenDictionary/odict/lib/utils"
)

func GetConfigDir() string {
	home, err := os.UserHomeDir()

	utils.Check(err)

	dir := path.Join(home, ".odict")

	if _, err := os.Stat(dir); os.IsNotExist(err) {
		err := os.Mkdir(dir, 0755)
		utils.Check(err)
	}

	return dir
}
