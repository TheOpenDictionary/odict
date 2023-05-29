package utils

import (
	"encoding/json"
	"encoding/xml"
)

// Serializes any interface to JSON if possible
func SerializeToJSON(any interface{}, prettyPrint bool) string {
	if prettyPrint {
		b, err := json.MarshalIndent(&any, "", " ")
		Check(err)
		return string(b)
	} else {
		b, err := json.Marshal(&any)
		Check(err)
		return string(b)
	}
}

// Serializes any interface to XML if possible
func SerializeToXML(any interface{}, prettyPrint bool) string {
	if prettyPrint {
		str, err := xml.MarshalIndent(&any, "", " ")
		Check(err)
		return string(str)
	} else {
		str, err := xml.Marshal(&any)
		Check(err)
		return string(str)
	}
}
