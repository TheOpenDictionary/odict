package sql

import (
	"bytes"
	"embed"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/bokwoon95/sqddl/ddl"
)

//go:embed definitions.go dictionaries.go
//go:embed entries.go etymologies.go
//go:embed examples.go groups.go
//go:embed notes.go senses.go
var fsys embed.FS

func createSchema(dialect SQLDialect) (string, error) {
	var buf bytes.Buffer

	// Generate SQL create statements and constraints
	generate := &ddl.GenerateCmd{
		Dialect: dialect,
		DirFS:   fsys,
		Filenames: []string{
			"dictionaries.go",
			"entries.go",
			"etymologies.go",
			"senses.go",
			"groups.go",
			"definitions.go",
			"notes.go",
			"examples.go",
		},
		DryRun: true,
		Stdout: &buf,
	}

	err := generate.Run()

	if err != nil {
		return "", err
	}

	return buf.String(), nil
}

func DictionaryToSQL(dict types.Dictionary, dialect SQLDialect, includeSchema bool) (string, error) {
	builder := createSQLBuilder(dialect)

	insertDictionary(&builder, dict)

	insert, err := builder.Build(dialect)

	if err != nil {
		return "", err
	}

	cmds := []string{insert}

	if includeSchema {
		schema, err := createSchema(dialect)

		if err != nil {
			return "", err
		}

		cmds = append([]string{schema}, cmds...)
	}

	return strings.Join(cmds, ";\n\n") + ";", nil
}
