package cli

import (
	"encoding/base64"
	"log"
	"os"

	"github.com/TheOpenDictionary/odict/lib/core"
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

func service(c *cli.Context) error {
	ipc := NewIPC()
	dictPath := c.Args().Get(0)

	var dict *types.Dictionary

	if len(dictPath) > 0 {
		dict = core.ReadDictionaryFromPath(dictPath)
	}

	f, err := os.OpenFile("testlogfile", os.O_RDWR|os.O_CREATE|os.O_APPEND, 0666)
	if err != nil {
		log.Fatalf("error opening file: %v", err)
	}
	defer f.Close()

	log.SetOutput(f)

	go func() {
		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodLookup], func(reply string, payload interface{}) {
			text := payload.(string)

			var buf []byte

			if len(text) > 0 {
				buf, err = base64.StdEncoding.DecodeString(text)
				utils.Check(err)
			}

			if dict != nil {
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

		ipc.OnReceiveAndReply(EnumNamesODictMethod[ODictMethodCompile], func(reply string, payload interface{}) {
			ipc.Reply(reply, "Sup Node", nil)
		})
	}()

	ipc.Start()

	// reader := bufio.NewReader(os.Stdin)
	// dictPath := c.Args().Get(0)

	// var dict *types.Dictionary

	// if len(dictPath) > 0 {
	// 	dict = core.ReadDictionaryFromPath(dictPath)
	// }

	// for {
	// 	text, err := reader.ReadString('\n')
	// 	split := strings.Split(text, ";")

	// 	if len(split) > 0 {
	// 		method, err := strconv.Atoi(split[0])
	// 		utils.Check(err)

	// 		b64 := split[1]

	// 		var buf []byte

	// 		if len(b64) > 0 {
	// 			buf, err = base64.StdEncoding.DecodeString(b64)
	// 			utils.Check(err)
	// 		}

	// 		switch ODictMethod(method) {
	// 		case ODictMethodCompile:
	// 			payload := GetRootAsCompilePayload(buf, 0)
	// 			core.CompileDictionary(string(payload.Path()), string(payload.Out()))
	// 			printSuccess()
	// 			end()
	// 		case ODictMethodSplit:
	// 			if dict != nil {
	// 				payload := GetRootAsSplitPayload(buf, 0)

	// 				split_(core.SplitRequest{
	// 					Dictionary: dict,
	// 					Query:      string(payload.Query()),
	// 					Threshold:  int(payload.Threshold()),
	// 				}, false)

	// 				end()
	// 			}
	// 		case ODictMethodLexicon:
	// 			if dict != nil {
	// 				lexicon_(dict)
	// 				end()
	// 			}
	// 		case ODictMethodWrite:
	// 			payload := GetRootAsWritePayload(buf, 0)
	// 			core.WriteDictionaryFromXML(string(payload.Xml()), string(payload.Out()))
	// 			printSuccess()
	// 			end()
	// 		case ODictMethodIndex:
	// 			if dict != nil {
	// 				ods.Index(ods.IndexRequest{Dictionary: dict, Overwrite: true, Quiet: true})
	// 				printSuccess()
	// 				end()
	// 			}
	// 		case ODictMethodSearch:
	// 			if dict != nil {
	// 				payload := GetRootAsSearchPayload(buf, 0)

	// 				search_(SearchRequest{
	// 					Dictionary:  dict,
	// 					Query:       string(payload.Query()),
	// 					Force:       payload.Force(),
	// 					Quiet:       true,
	// 					Exact:       payload.Exact(),
	// 					PrettyPrint: false,
	// 				})

	// 				end()
	// 			}
	// 		case ODictMethodLookup:
	// if dict != nil {
	// 	payload := GetRootAsLookupPayload(buf, 0)
	// 	queries := make([]string, payload.QueriesLength())
	// 	follow := payload.Follow()
	// 	split := int(payload.Split())

	// 	for i := 0; i < payload.QueriesLength(); i++ {
	// 		queries[i] = string(payload.Queries(i))
	// 	}

	// 	lookup_(core.LookupRequest{
	// 		Dictionary: dict,
	// 		Queries:    queries,
	// 		Follow:     follow,
	// 		Split:      split,
	// 	}, "json", false)

	// 	end()
	// }
	// 		}
	// 	}

	// 	if err != nil {
	// 		fmt.Println(err)
	// 		continue
	// 	}
	// }

	return nil
}
