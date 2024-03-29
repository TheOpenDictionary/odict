// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class CompilePayload {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):CompilePayload {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsCompilePayload(bb:flatbuffers.ByteBuffer, obj?:CompilePayload):CompilePayload {
  return (obj || new CompilePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsCompilePayload(bb:flatbuffers.ByteBuffer, obj?:CompilePayload):CompilePayload {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new CompilePayload()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

path():string|null
path(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
path(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

out():string|null
out(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
out(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startCompilePayload(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addPath(builder:flatbuffers.Builder, pathOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, pathOffset, 0);
}

static addOut(builder:flatbuffers.Builder, outOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, outOffset, 0);
}

static endCompilePayload(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createCompilePayload(builder:flatbuffers.Builder, pathOffset:flatbuffers.Offset, outOffset:flatbuffers.Offset):flatbuffers.Offset {
  CompilePayload.startCompilePayload(builder);
  CompilePayload.addPath(builder, pathOffset);
  CompilePayload.addOut(builder, outOffset);
  return CompilePayload.endCompilePayload(builder);
}
}
