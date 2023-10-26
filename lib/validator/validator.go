package validator

import (
	_ "embed"
	"errors"

	xsdvalidate "github.com/terminalstatic/go-xsd-validate"
)

//go:embed odict.xsd
var xsd []byte

var xsdHandler *xsdvalidate.XsdHandler

func InitializeValidator() {
	xsdvalidate.Init()

	var err error

	xsdHandler, err = xsdvalidate.NewXsdHandlerMem(xsd, xsdvalidate.ParsErrDefault)

	if err != nil {
		panic(err)
	}
}

func CleanupValidator() {
	xsdvalidate.Cleanup()
	xsdHandler.Free()
}

func Validate(xmlStr string) error {
	if xsdHandler == nil {
		return errors.New("validator not initialized")
	}

	return xsdHandler.ValidateMem([]byte(xmlStr), xsdvalidate.ParsErrVerbose)
}
