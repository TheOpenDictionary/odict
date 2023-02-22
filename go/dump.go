package odict

// DumpDictionaryXML converts an Dictionary struct
// to its original ODXML
func (dict *Dictionary) DumpXML() string {
	return XML(dict.AsRepresentable())
}

// DumpDictionarySQL converts an Dictionary struct
// to SQL output to seed a database
func (dict *Dictionary) DumpSQL() string {
	return SQL(dict.AsRepresentable())
}
