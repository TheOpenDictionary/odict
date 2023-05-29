package cli

import (
	"encoding/base64"

	"github.com/TheOpenDictionary/odict/lib/core"
	ods "github.com/TheOpenDictionary/odict/lib/search"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

type Request struct {
	Function   string            `json:"function"`
	Parameters map[string]string `json:"parameters"`
}

type Payload struct {
	Payload []int `json:"payload"`
}

func decodePayload(payload interface{}) ([]byte, bool) {
	text := payload.(string)

	var buf []byte
	var err error

	if len(text) > 0 {
		buf, err = base64.StdEncoding.DecodeString(text)
		utils.Check(err)
		return buf, true
	}

	return nil, false
}

func service(c *cli.Context) error {
	ipc := NewIPC()
	dictPath := c.Args().Get(0)

	var dict *types.Dictionary

	if len(dictPath) > 0 {
		dict = core.ReadDictionaryFromPath(dictPath)
	}

	go func() {
		// Write
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodWrite], func(reply replyChannel, payload interface{}) {
			if buf, ok := decodePayload(payload); ok {
				payload := GetRootAsWritePayload(buf, 0)
				core.WriteDictionaryFromXML(string(payload.Xml()), string(payload.Out()))
				ipc.Reply(reply, true, nil)
			}
		})

		// Split
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodSplit], func(reply replyChannel, payload interface{}) {
			if buf, ok := decodePayload(payload); ok {
				payload := GetRootAsSplitPayload(buf, 0)

				query := string(payload.Query())

				threshold := int(payload.Threshold())

				entries := core.Split(core.SplitRequest{
					Dictionary: dict,
					Query:      query,
					Threshold:  threshold,
				})

				representable := utils.Map(entries, func(entry types.Entry) types.EntryRepresentable {
					return entry.AsRepresentable()
				})

				ipc.Reply(reply, representable, nil)
			}
		})

		// Search
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodSearch], func(reply replyChannel, payload interface{}) {
			if buf, ok := decodePayload(payload); ok {
				payload := GetRootAsSearchPayload(buf, 0)
				force := payload.Force()
				exact := payload.Exact()
				query := string(payload.Query())

				ods.Index(ods.IndexRequest{Dictionary: dict, Overwrite: force, Quiet: true})

				results := ods.SearchDictionary(string(dict.Id()), query, exact)

				representable := utils.Map(results, func(entry types.Entry) types.EntryRepresentable {
					return entry.AsRepresentable()
				})

				ipc.Reply(reply, representable, nil)
			}
		})

		// Index
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodIndex], func(reply replyChannel, payload interface{}) {
			ods.Index(ods.IndexRequest{Dictionary: dict, Overwrite: true, Quiet: true})
			ipc.Reply(reply, true, nil)
		})

		// Lexicon
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodLexicon], func(reply replyChannel, payload interface{}) {
			result := core.Lexicon(dict)
			ipc.Reply(reply, result, nil)
		})

		// Lookup
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodLookup], func(reply replyChannel, payload interface{}) {
			if buf, ok := decodePayload(payload); ok && dict != nil {
				payload := GetRootAsLookupPayload(buf, 0)
				queries := make([]string, payload.QueriesLength())
				follow := payload.Follow()
				split := int(payload.Split())

				for i := 0; i < payload.QueriesLength(); i++ {
					queries[i] = string(payload.Queries(i))
				}

				entries := core.Lookup(core.LookupRequest{
					Dictionary: dict,
					Queries:    queries,
					Follow:     follow,
					Split:      split,
				})

				representable := utils.Map(entries, func(e []types.Entry) []types.EntryRepresentable {
					return utils.Map(e, func(entry types.Entry) types.EntryRepresentable {
						return entry.AsRepresentable()
					})
				})

				ipc.Reply(reply, representable, nil)
			}
		})

		// Compile
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodCompile], func(reply replyChannel, payload interface{}) {
			if buf, ok := decodePayload(payload); ok {
				payload := GetRootAsCompilePayload(buf, 0)
				core.CompileDictionary(string(payload.Path()), string(payload.Out()))
				ipc.Reply(reply, true, nil)
			}
		})

	}()

	ipc.Start()

	return nil
}
