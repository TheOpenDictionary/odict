package odict

import "encoding/xml"

// DumpDictionary converts an Dictionary struct
// to its original ODXML
func (dict *Dictionary) Dump() string {
	representable := dict.AsRepresentable()
	str, err := xml.MarshalIndent(&representable, "", " ")

	Check(err)

	return string(str)
}
