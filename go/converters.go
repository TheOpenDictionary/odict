package odict

import (
	"bytes"
	"embed"
	"encoding/json"
	"encoding/xml"
	"fmt"

	"github.com/bokwoon95/sq"
	"github.com/bokwoon95/sqddl/ddl"
	flatbuffers "github.com/google/flatbuffers/go"
)

//go:embed sql.go
var fsys embed.FS

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[str]; ok {
		return val
	}

	return Unknown
}

func serialize(b Serializable) []byte {
	builder := flatbuffers.NewBuilder(0)
	buffer := b.AsBuffer(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}

func JSON(any interface{}) string {
	b, err := json.MarshalIndent(&any, "", " ")

	Check(err)

	return string(b)
}

func XML(any interface{}) string {
	str, err := xml.MarshalIndent(&any, "", " ")

	Check(err)

	return string(str)
}

func SQL(dict DictionaryRepresentable, sqlDialect string) string {
	var buf bytes.Buffer
	var sqlCmds string

	generateCmd := &ddl.GenerateCmd{
		Dialect:   sqlDialect,
		DirFS:     fsys,
		Filenames: []string{"sql.go"},
		DryRun:    true,
		Stdout:    &buf,
	}
	err := generateCmd.Run()
	if err != nil {
		fmt.Println(err)
	}

	sqlCmds += buf.String()

	languages := [][2]string{
		{"abc", "def"},
		{"ghi", "jkl"},
		{"mno", "pqr"},
	}
	for _, language := range languages {
		l := sq.New[LANGUAGE]("")
		insertQuery := sq.
			InsertInto(l).
			Columns(l.CODE, l.FLAG).
			Values(sq.Literal(language[0]), sq.Literal(language[1]))
		query, _, err := sq.ToSQL(sq.DialectPostgres, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}
		fmt.Println(query + ";")
		sqlCmds += query + ";\n"
	}

	return sqlCmds
}
