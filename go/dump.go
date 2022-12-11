package odict

// DumpDictionary converts an Dictionary struct
// to its original ODXML
func (dict *Dictionary) Dump() string {
	return XML(dict.AsRepresentable())
}
