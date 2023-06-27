/*
Adapted from https://raw.githubusercontent.com/Akumzy/ipc/master/ipc.go

# MIT License

# Copyright (c) 2019 Akuma Isaac Akuma

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
package cli

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"strings"
	"sync"
	"time"

	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/google/uuid"
)

// IPC channel
type IPC struct {
	sendChannel           chan payload
	receiveListerners     map[string][]Handler
	receiveSendListerners map[string][]HandlerWithReply
}

var (
	rLock  sync.Mutex
	rRLock sync.Mutex
)

type replyChannel struct {
	Event string `json:"event"`
	ID    string `json:"id"`
}

// Payload this is the payload structure
type payload struct {
	ID    string `json:"id"`
	Event string `json:"event"`
	// If the data received from the parent is a literal value `Data`
	//type will be equals to the underlining type for example:
	// JavaScripts === Go
	// `null  === nil`
	// `undefined === nil`
	// `number(int) === int`
	// `string === string`
	// else if the `Data` is an Object in JavaScript
	// data will be a JSON string
	Data  interface{} `json:"data"`
	Error interface{} `json:"error"`
	SR    bool        `json:"SR"` // send and receive
	RS    bool        `json:"RC"` // receive and send
}

// Handler When the underline type of data is being
//
//	access through `type assertion` if the data has a
//	literal value the underlining type will be return
//	else a `JSON` representative of the data will be return
type Handler func(data interface{})

// HandlerWithReply  When the underline type of data is being
//
//	access through `type assertion` if the data has a literal
//	value the underlining type will be return else a `JSON` representative of
//	the data will be return.
//	`replyChannel` is the event name you'll pass to `ipc.Reply` method to respond
//	 to the sender
type HandlerWithReply func(channel replyChannel, data interface{})

// PayloadReceive this is the payload structure
type payloadReceive struct {
	ID    string      `json:"id"`
	Event string      `json:"event"`
	Data  interface{} `json:"data"`
	SR    bool        `json:"SR"` //send and receive
}

// Send data to parent process
func (ipc IPC) Send(event string, data interface{}) {
	ipc.sendChannel <- payload{ID: uuid.NewString(), Event: event, Data: data}
}

// Reply back to sender
func (ipc IPC) Reply(channel replyChannel, data, err interface{}) {
	ipc.sendChannel <- payload{ID: channel.ID, Event: channel.Event, Data: data, SR: true, Error: err}
}

// On listens for events from parent process
func (ipc IPC) On(event string, handler Handler) {
	rLock.Lock()
	defer rLock.Unlock()
	h := ipc.receiveListerners[event]
	h = append(h, handler)
	ipc.receiveListerners[event] = h
}

// OnReceiveAndReply listen for an events and as well reply back to
// the same sender with the help of `ipc.Reply` method
func (ipc IPC) OnReceiveAndReply(event string, handler HandlerWithReply) {
	rRLock.Lock()
	defer rRLock.Unlock()
	h := ipc.receiveSendListerners[event]
	h = append(h, handler)
	ipc.receiveSendListerners[event] = h
}

// SendAndReceive send and listen for reply event
func (ipc IPC) SendAndReceive(event string, data interface{}, handler Handler) {
	id := uuid.NewString()
	ipc.sendChannel <- payload{ID: id, Event: event, Data: data, RS: true}
	channel := event + ":" + id
	ipc.On(channel, handler)
}

// RemoveListener remove listener
func (ipc IPC) RemoveListener(event string) {
	delete(ipc.receiveListerners, event)
}

// Start `ipc`
// the `ipc.Start` method will blocks executions
// so is either you put in a separate `Go routine` or put you own code in
// a different `Go routine`
func (ipc IPC) Start() {

	go func() {
		for {
			msg := <-ipc.sendChannel
			data, err := Marshal(msg)
			if err != nil {
				log.Println(err)
			} else {
				text := data + "\\n"

				if os.Getenv("ODICT_DEBUG_IPC") == "true" {
					utils.AppendToFile("ipc.log", "GO SENT: "+text)
				}

				fmt.Print(text)
			}
		}
	}()

	reader := bufio.NewReader(os.Stdin)

	for {
		text, err := reader.ReadString('\n')

		if err != nil {
			log.Println(err)
			continue
		}

		if text != "" {
			var payload payloadReceive

			text = strings.TrimSuffix(text, "\n")

			if os.Getenv("ODICT_DEBUG_IPC") == "true" {
				utils.AppendToFile("ipc.log", "GO RECEIVED: "+text)
			}

			// check if the text is not empty string
			if text != "" {
				if err := json.Unmarshal([]byte(text), &payload); err != nil {
					log.Println(err)
					continue
				}

				if payload.Event == "___EXIT___" {
					os.Exit(0)
				}

				// Run the handlers in a goroutine to prevent
				// https://github.com/Akumzy/ipc/issues/1
				go func() {
					if payload.SR {
						for _, handler := range ipc.receiveSendListerners[payload.Event] {
							handler(replyChannel{ID: payload.ID, Event: payload.Event}, payload.Data)
						}
					} else {
						for _, handler := range ipc.receiveListerners[payload.Event+":"+payload.ID] {
							handler(payload.Data)
						}
						println(payload.Event)
						println(payload.SR)
						println(len(ipc.receiveListerners[payload.Event]))
						for _, handler := range ipc.receiveListerners[payload.Event] {
							println(handler)
							handler(payload.Data)
						}
					}
				}()
			}

		}
	}
}

// Marshal to json
func Marshal(v interface{}) (string, error) {
	buf := new(bytes.Buffer)
	enc := json.NewEncoder(buf)
	enc.SetEscapeHTML(false)
	if err := enc.Encode(&v); err != nil {
		return "", err
	}
	log.Println(buf.String())
	return strings.TrimSpace(buf.String()), nil
}

// pingPong is used to eliminate zombies
// ping the parent process every 20 seconds
func pingPong(ipc *IPC) {
	isActive := true
	ipc.On("pong", func(d interface{}) {
		isActive = true
	})
	for {
		time.Sleep(20 * time.Second)
		if isActive {
			isActive = false
		} else {
			log.Println("[IPC] Timeout closing process.")
			os.Exit(0)
		}
		ipc.Send("ping", nil)
	}
}

// New return now ipc
func NewIPC() *IPC {
	ipc := &IPC{}
	ipc.sendChannel = make(chan payload)
	ipc.receiveListerners = make(map[string][]Handler)
	ipc.receiveSendListerners = make(map[string][]HandlerWithReply)
	go pingPong(ipc)
	return ipc
}
