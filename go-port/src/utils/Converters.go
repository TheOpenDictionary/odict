package utils

import "encoding/binary"

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
