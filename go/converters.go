package odict

import (
	"embed"
	"encoding/json"
	"encoding/xml"
	"fmt"

	"github.com/bokwoon95/sq"
	"github.com/bokwoon95/sqddl/ddl"
	flatbuffers "github.com/google/flatbuffers/go"
	_ "github.com/jinzhu/gorm/dialects/postgres"
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

func SQL(dict DictionaryRepresentable) string {
	dialects := []string{sq.DialectSQLite, sq.DialectPostgres, sq.DialectMySQL, sq.DialectSQLServer}
	for _, dialect := range dialects {
		fmt.Println("\n--[ " + dialect + " ]--\n")
		generateCmd := &ddl.GenerateCmd{
			Dialect:   dialect,
			DirFS:     fsys,
			Filenames: []string{"sql.go"},
			DryRun:    true,
		}
		err := generateCmd.Run()
		if err != nil {
			fmt.Println(err)
		}
	}

	return "yo"
}
