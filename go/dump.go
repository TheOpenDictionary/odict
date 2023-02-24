package odict

type SqlDialect = string

const (
	Postgres  SqlDialect = "postgres"
	Sqlite    SqlDialect = "sqlite"
	Mysql     SqlDialect = "mysql"
	Sqlserver SqlDialect = "sqlserver"
)

// DumpDictionaryXML converts an Dictionary struct
// to its original ODXML
func (dict *Dictionary) DumpXML() string {
	return XML(dict.AsRepresentable())
}

// DumpDictionarySQL converts an Dictionary struct
// to SQL output to seed a database
func (dict *Dictionary) DumpSQL(sqlDialect SqlDialect) string {
	return sql(dict.AsRepresentable(), sqlDialect)
}
