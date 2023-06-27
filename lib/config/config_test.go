package config

import (
	"os"
	"testing"

	"github.com/TheOpenDictionary/odict/lib/test"
	"github.com/stretchr/testify/assert"
)

func TestGetConfigDir(t *testing.T) {
	expected := ".odict"

	os.Setenv("ODICT_CONFIG_DIR", expected)

	configDir, err := GetConfigDir()

	assert.Nil(t, err)
	assert.Equal(t, expected, configDir)

	test.CleanupTest()
}
