package odict

import flatbuffers "github.com/google/flatbuffers/go"

func Compare(offset_1 flatbuffers.UOffsetT, key []byte, buf []byte) int {
	offset_1 += flatbuffers.GetUOffsetT(buf[offset_1:])
	len_1 := int(flatbuffers.GetInt32(buf[offset_1:]))
	len_2 := len(key)
	startPos_1 := int(offset_1 + flatbuffers.SizeInt32)
	len := len_1
	if len_2 < len_1 {
		len = len_2
	}
	for i := 0; i < len; i++ {
		if buf[i+startPos_1] != key[i] {
			return int(buf[i+startPos_1]) - int(key[i])
		}
	}
	return len_1 - len_2
}

func Offset(vtableOffset flatbuffers.VOffsetT, offset flatbuffers.UOffsetT, buf []byte) flatbuffers.UOffsetT {
	vtable := flatbuffers.UOffsetT(len(buf)) - offset
	off := vtable + flatbuffers.UOffsetT(vtableOffset) - flatbuffers.GetUOffsetT(buf[vtable:])
	return flatbuffers.UOffsetT(flatbuffers.GetInt8(buf[off:])) + vtable
}

// Indirect retrieves the relative offset stored at `offset`.
func Indirect(off flatbuffers.UOffsetT, buf []byte) flatbuffers.UOffsetT {
	return off + flatbuffers.GetUOffsetT(buf[off:])
}

func lookupByKey(obj *Entry, vector flatbuffers.UOffsetT, k string, buf []byte) bool {
	key := []byte(k)
	span := flatbuffers.GetUOffsetT(buf[vector-4:])
	start := flatbuffers.UOffsetT(0)
	for ok := true; ok; ok = span != 0 {
		middle := span / 2
		tableOffset := Indirect(vector+4*(start+middle), buf)
		comp := Compare(flatbuffers.UOffsetT(Offset(4, flatbuffers.UOffsetT(len(buf))-tableOffset, buf)), key, buf)
		if comp > 0 {
			span = middle
		} else if comp < 0 {
			middle += 1
			start += middle
			span -= middle
		} else {
			obj.Init(buf, tableOffset)
			return true
		}
	}
	return false
}

func (rcv *Dictionary) EntryByKey(obj *Entry, key string) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return lookupByKey(obj, rcv._tab.Vector(o), key, rcv._tab.Bytes)
	}
	return false
}
