package config

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path"
	"sort"

	"github.com/samber/lo"
)

type DictionaryAlias struct {
	Name string
	Path string
}

func getDictionariesFile() (string, error) {
	configDir, err := GetConfigDir()

	if err != nil {
		return "", err
	}

	return path.Join(configDir, "aliases.json"), nil
}

func getDictionariesConfig() (map[string]string, error) {
	dictionariesFile, err := getDictionariesFile()

	if err != nil {
		return nil, err
	}

	dictionariesFileBytes, err := os.ReadFile(dictionariesFile)

	if os.IsNotExist(err) {
		writeErr := os.WriteFile(dictionariesFile, []byte("{}"), 0644)

		if writeErr != nil {
			return nil, writeErr
		}

		return map[string]string{}, nil
	}

	if err != nil {
		return nil, err
	}

	var dictionaryMap map[string]string

	json.Unmarshal(dictionariesFileBytes, &dictionaryMap)

	return dictionaryMap, nil
}

func writeDictionaryConfig(config map[string]string) error {
	dictionariesFile, readErr := getDictionariesFile()

	if readErr != nil {
		return readErr
	}

	configBytes, marshalErr := json.Marshal(config)

	if marshalErr != nil {
		return marshalErr
	}

	writeErr := os.WriteFile(dictionariesFile, configBytes, 0644)

	if writeErr != nil {
		return writeErr
	}

	return nil
}

func GetDictionaryPathFromAlias(alias string) (string, error) {
	dictionaries, err := getDictionariesConfig()

	if err != nil {
		return "", err
	}

	return dictionaries[alias], nil
}

func ListDictionaries() ([]DictionaryAlias, error) {
	dictionaries, err := getDictionariesConfig()

	if err != nil {
		return nil, err
	}

	keys := lo.Keys(dictionaries)

	sort.Strings(keys)

	return lo.Map(keys, func(name string, _ int) DictionaryAlias {
		return DictionaryAlias{Name: name, Path: dictionaries[name]}
	}), nil
}

func AddDictionaryAlias(name string, path string) error {
	dictionaries, err := getDictionariesConfig()

	if err != nil {
		return err
	}

	if _, ok := dictionaries[name]; ok {
		return errors.New("a dictionary alias with that name already exists")
	}

	return SetDictionaryAlias(name, path)
}

func SetDictionaryAlias(name string, path string) error {
	dictionaries, err := getDictionariesConfig()

	if err != nil {
		return err
	}

	dictionaries[name] = path

	return writeDictionaryConfig(dictionaries)
}

func RemoveDictionaryAlias(name string) error {
	dictionaries, err := getDictionariesConfig()

	if err != nil {
		return err
	}

	if _, ok := dictionaries[name]; !ok {
		return fmt.Errorf("no dictionary alias with that name exists")
	}

	delete(dictionaries, name)

	return writeDictionaryConfig(dictionaries)
}
