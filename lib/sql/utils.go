package sql

import (
	"strings"

	"github.com/bokwoon95/sq"
)

type SQLBuilder struct {
	dialect SQLDialect
	cmds    []sq.SQLWriter
}

func createSQLBuilder(dialect SQLDialect) SQLBuilder {
	return SQLBuilder{
		dialect: dialect,
		cmds:    []sq.SQLWriter{},
	}
}

func (builder *SQLBuilder) AddCommand(cmd sq.SQLWriter) {
	builder.cmds = append(builder.cmds, cmd)
}

func (builder *SQLBuilder) Build(dialect SQLDialect) (string, error) {
	var cmds []string = []string{}

	for _, cmd := range builder.cmds {
		query, _, err := sq.ToSQL(dialect, cmd, nil)

		if err != nil {
			return "", err
		}

		cmds = append(cmds, query)
	}

	return strings.Join(cmds, ";\n"), nil
}
