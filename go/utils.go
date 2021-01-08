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

// Uint64ToBytes converts a uint64 type to a byte array
func Uint64ToBytes(n uint64) []byte {
	bytes := make([]byte, 8)

	// TODO: normalize
	binary.LittleEndian.PutUint64(bytes, uint64(n))

	return bytes
}
