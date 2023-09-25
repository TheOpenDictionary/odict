package types

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMDString(t *testing.T) {
	str := "**This** is a ^test^ of the _parser_"
	expected := "<strong>This</strong> is a <sup>test</sup> of the <em>parser</em>"
	output, err := MDString(str).MarshalText()

	assert.Equal(t, nil, err)
	assert.Equal(t, expected, string(output))
}

func TestMDStringDisabled(t *testing.T) {
	oldValue := IsMarkdownProcessingEnabled()

	SetMarkdownProcessingEnabled(false)

	str := "**This** is a ^test^ of the _parser_"
	output, err := MDString(str).MarshalText()

	assert.Equal(t, nil, err)
	assert.Equal(t, str, string(output))

	SetMarkdownProcessingEnabled(oldValue)
}
