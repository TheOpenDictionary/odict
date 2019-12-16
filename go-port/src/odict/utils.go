package odict

import "encoding/binary"

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func Assert(condition bool, errorMessage string) {
	if !condition {
		panic("Assertion failed: " + errorMessage)
	}
}

func Uint16ToBytes(n uint16) []byte {
	bytes := make([]byte, 2)

	// TODO: normalize
	binary.LittleEndian.PutUint16(bytes, uint16(n))

	return bytes
}

func Uint32ToBytes(n uint32) []byte {
	bytes := make([]byte, 4)

	// TODO: normalize
	binary.LittleEndian.PutUint32(bytes, uint32(n))

	return bytes
}
