package validator

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	xsdvalidate "github.com/terminalstatic/go-xsd-validate"
)

func TestValidatorFails(t *testing.T) {
	InitializeValidator()

	defer CleanupValidator()

	err := Validate("<xml></xml>")

	assert.Equal(t, err, xsdvalidate.ValidationError(xsdvalidate.ValidationError{Errors: []xsdvalidate.StructError{{Code: 1845, Message: "Element 'xml': No matching global declaration available for the validation root.", Level: 2, Line: 1, NodeName: "xml"}}}))
}

func TestValidatorPasses(t *testing.T) {
	InitializeValidator()

	defer CleanupValidator()

	body, readErr := os.ReadFile("../../examples/example1.xml")

	err := Validate(string(body))

	assert.Equal(t, readErr, nil)
	assert.Equal(t, err, nil)
}
