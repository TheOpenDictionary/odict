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
			"examples.go",
			"notes.go",
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

func DictionaryToSQL(dict types.DictionaryRepresentable, dialect SQLDialect) (string, error) {
	builder := createSQLBuilder(dialect)

	insertDictionary(&builder, dict)

	schema, err := createSchema(dialect)

	if err != nil {
		return "", err
	}

	insert, err := builder.Build(dialect)

	if err != nil {
		return "", err
	}

	return strings.Join([]string{schema, insert}, ";\n\n"), nil
}
