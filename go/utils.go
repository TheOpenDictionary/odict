package odict

import "encoding/binary"

// Check panics the program if an error exists
func Check(e error) {
	if e != nil {
		panic(e)
	}
}

// Assert panics the program if a condition is not met
func Assert(condition bool, errorMessage string) {
	if !condition {
		panic("Assertion failed: " + errorMessage)
	}
}

// Uint16ToBytes converts a uint16 type to a byte array
func Uint16ToBytes(n uint16) []byte {
	bytes := make([]byte, 2)

	// TODO: normalize
	binary.LittleEndian.PutUint16(bytes, uint16(n))

	return bytes
}

// Uint32ToBytes converts a uint32 type to a byte array
func Uint32ToBytes(n uint32) []byte {
	bytes := make([]byte, 4)

	// TODO: normalize
	binary.LittleEndian.PutUint32(bytes, uint32(n))

	return bytes
}
