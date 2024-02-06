package config

import (
	"os"
	"path"
)

func GetConfigDir() (string, error) {
	configDir := os.Getenv("ODICT_CONFIG_DIR")

	if len(configDir) == 0 {
		home, err := os.UserHomeDir()

		if err != nil {
			return "", err
		}

		configDir = path.Join(home, ".odict")
	}

	if _, err := os.Stat(configDir); os.IsNotExist(err) {
		err := os.Mkdir(configDir, 0755)

		if err != nil {
			return "", err
		}

		return configDir, nil
	}

	return configDir, nil
}
