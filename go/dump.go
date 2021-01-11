package odict

import "encoding/xml"

// DumpDictionary converts an Dictionary struct
// to its original ODXML
func DumpDictionary(dict Dictionary) string {
	str, err := xml.MarshalIndent(&dict, "", " ")

	Check(err)

	return string(str)
}
