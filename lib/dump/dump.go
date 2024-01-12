package dump

import (
	"github.com/TheOpenDictionary/odict/lib/sql"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
)

type SQLDialect = string

const (
	Postgres  SQLDialect = "postgres"
	Sqlite    SQLDialect = "sqlite"
	Mysql     SQLDialect = "mysql"
	Sqlserver SQLDialect = "sqlserver"
)

// AsXML converts an Dictionary struct
// to its original ODXML
func AsXML(dict *types.Dictionary) (string, error) {
	return utils.SerializeToXML(dict.AsRepresentable(), true)
}

// AsSQL converts an Dictionary struct
// to SQL output to seed a database
func AsSQL(dict *types.Dictionary, dialect SQLDialect, includeSchema bool) (string, error) {
	return sql.DictionaryToSQL(dict.AsRepresentable(), dialect, includeSchema)
}
