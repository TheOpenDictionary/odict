package odict

import (
	"encoding/xml"

	"github.com/odict/odict/go/models"
)

// DumpDictionary converts an Dictionary struct
// to its original ODXML
func DumpDictionary(dict models.Dictionary) string {
	str, err := xml.MarshalIndent(&dict, "", " ")

	Check(err)

	return string(str)
}
