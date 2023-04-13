package utils

import (
	"encoding/json"
	"encoding/xml"
)

// Serializes any interface to JSON if possible
func SerializeToJSON(any interface{}) string {
	b, err := json.MarshalIndent(&any, "", " ")

	Check(err)

	return string(b)
}

// Serializes any interface to XML if possible
func SerializeToXML(any interface{}) string {
	str, err := xml.MarshalIndent(&any, "", " ")

	Check(err)

	return string(str)
}
