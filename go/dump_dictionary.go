package odict

import "encoding/xml"

// DumpDictionary converts an OpenDictionary struct
// to its original ODXML
func DumpDictionary(dict OpenDictionary) string {
	str, err := xml.MarshalIndent(&dict, "", " ")

	Check(err)

	return string(str)
}
