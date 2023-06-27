package dump

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
)

type SqlDialect = string

const (
	Postgres  SqlDialect = "postgres"
	Sqlite    SqlDialect = "sqlite"
	Mysql     SqlDialect = "mysql"
	Sqlserver SqlDialect = "sqlserver"
)

// AsXML converts an Dictionary struct
// to its original ODXML
func AsXML(dict *types.Dictionary) (string, error) {
	return utils.SerializeToXML(dict.AsRepresentable(), true)
}

// AsSQL converts an Dictionary struct
// to SQL output to seed a database
func AsSQL(dict *types.Dictionary, sqlDialect SqlDialect) (string, error) {
	return sql(dict.AsRepresentable(), sqlDialect)
}
