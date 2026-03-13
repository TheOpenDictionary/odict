import { readFile, writeFile, unlink } from 'node:fs/promises'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { tmpdir } from 'node:os'
import test from 'ava'

import { compile, OpenDictionary, init } from '../index.js'

// Initialize with temporary directory
const storagePath = tmpdir();
init(storagePath);

async function getDictionary(name: string) {
  const xml = await readFile(join(fileURLToPath(new URL(import.meta.url)), `../../../examples/${name}.xml`), 'utf-8');
  return new OpenDictionary(storagePath, compile(storagePath, xml))
}

let dict1: OpenDictionary
let dict2: OpenDictionary
let dict3: OpenDictionary

test.before(async () => {
  dict1 = await getDictionary('example1')
  dict2 = await getDictionary('example2')
  dict3 = await getDictionary('example3')
})

test('to_bytes - returns dictionary as bytes and supports options', async (t) => {
  const bytes = dict1.toBytes()
  t.true(bytes instanceof Buffer || bytes instanceof Uint8Array)
  t.true(bytes.length > 0)

  const loadedDict = new OpenDictionary(storagePath, bytes)
  t.deepEqual(loadedDict.lexicon(), dict1.lexicon())

  const compressedBytes = dict1.toBytes({ quality: 9, windowSize: 32 })
  t.true(compressedBytes.length > 0)

  const loadedCompressedDict = new OpenDictionary(storagePath, compressedBytes)
  t.deepEqual(loadedCompressedDict.lexicon(), dict1.lexicon())
})

test('lookup - looks up terms properly', async (t) => {
  const result = dict1.lookup('cat')
  t.snapshot(result)
})

test("lookup - doesn't split unless specified", async (t) => {
  const result = dict1.lookup('catdog')
  t.is(result.length, 0)
})

test('lookup - follows terms properly', async (t) => {
  const result = dict1.lookup('ran', { follow: true })
  t.is(result[0].entry.term, 'run')
  t.is(result[0].directedFrom?.term, 'ran')
})

test('lookup - can split terms', async (t) => {
  const result = dict1.lookup('catdog', { split: 3 })
  t.snapshot(result)
})

test('lookup - is case-sensitive by default', async (t) => {
  const result = dict1.lookup('CAT')
  t.is(result.length, 0)
})

test('lookup - can perform case-insensitive lookup', async (t) => {
  const result = dict1.lookup('CAT', { insensitive: true })
  t.is(result.length, 1)
  t.is(result[0].entry.term, 'cat')
})

test('lookup - works with mixed case', async (t) => {
  const result = dict1.lookup(['DoG', 'cAT'], { insensitive: true })
  t.is(result.length, 2)
  t.is(result[0].entry.term, 'dog')
  t.is(result[1].entry.term, 'cat')
})

test('lookup - combines case-insensitivity with follow option', async (t) => {
  const result = dict1.lookup('RaN', { follow: true, insensitive: true })
  t.is(result[0].entry.term, 'run')
  t.is(result[0].directedFrom?.term, 'ran')
})

test('lookup - supports follow=true for infinite following', async (t) => {
  const result = dict1.lookup('ran', { follow: true })
  t.is(result[0].entry.term, 'run')
  t.is(result[0].directedFrom?.term, 'ran')
})

test('lookup - supports follow=false to disable following', async (t) => {
  const result = dict1.lookup('ran', { follow: false })
  t.is(result[0].entry.term, 'ran')
})

test('can return the lexicon', async (t) => {
  const result = dict1.lexicon()
  t.deepEqual(result, ['cat', 'dog', 'poo', 'ran', 'run'])
})

test('rank - returns correct min_rank for dictionary with ranks', (t) => {
  t.is(dict1.minRank, 100)
})

test('rank - returns correct max_rank for dictionary with ranks', (t) => {
  t.is(dict1.maxRank, 100)
})

test('rank - returns null min_rank for dictionary without ranks', (t) => {
  t.is(dict2.minRank, null)
})

test('rank - returns null max_rank for dictionary without ranks', (t) => {
  t.is(dict2.maxRank, null)
})

test('rank - handles mixed entries with and without ranks', async (t) => {
  const mixedXml = `<?xml version="1.0" encoding="UTF-8"?>
<dictionary>
  <entry term="high">
    <ety>
      <pos>noun</pos>
      <def>High ranking</def>
    </ety>
  </entry>
  <entry term="medium" rank="50">
    <ety>
      <pos>noun</pos>
      <def>Medium ranking</def>
    </ety>
  </entry>
  <entry term="low" rank="10">
    <ety>
      <pos>noun</pos>
      <def>Low ranking</def>
    </ety>
  </entry>
  <entry term="highest" rank="100">
    <ety>
      <pos>noun</pos>
      <def>Highest ranking</def>
    </ety>
  </entry>
</dictionary>`

  const mixedDict = new OpenDictionary(storagePath, compile(storagePath, mixedXml))
  t.is(mixedDict.minRank, 10)
  t.is(mixedDict.maxRank, 100)
})

test('should tokenize text and find entries', (t) => {
  const tokens = dict3.tokenize('你好！你是谁？')

  t.snapshot(tokens)
  t.true(tokens.length > 0)
  t.is(tokens[0].lemma, '你好')
  t.is(tokens[0].entries[0].entry.term, '你')
  t.is(tokens[0].entries[1].entry.term, '好')
})

test.serial('can index and search a dictionary', async (t) => {
  if (process.env.NAPI_RS_FORCE_WASI) {
    t.pass('Skipped due to NAPI_RS_FORCE_WASI')
    return
  }

  dict1.index()

  const results = dict1.search('run')

  t.snapshot(results)
})

test('throws errors inside JavaScript', async (t) => {
  const error = t.throws(() => {
    const dict = new OpenDictionary(storagePath, 'fake-alias')
    dict.lookup('dog')
  })

  t.true(!!error.message)
})

if (typeof OpenDictionary.load === 'function') {
  const dict1Path = join(tmpdir(), 'example1.odict')
  const dict2Path = join(tmpdir(), 'example2.odict')
  const emptyPath = join(tmpdir(), 'empty.odict')

  test.before('setup odict files', async () => {
    const dict1 = await getDictionary('example1')
    const dict2 = await getDictionary('example2')
    const empty = await getDictionary('empty')

    dict1.save(dict1Path)
    dict2.save(dict2Path)
    empty.save(emptyPath)
  })

  test('load - loads dictionary from local .odict file path', async (t) => {
    const loadedDict = await OpenDictionary.load(storagePath, dict1Path)

    t.truthy(loadedDict)
    t.deepEqual(loadedDict.lexicon(), ['cat', 'dog', 'poo', 'ran', 'run'])

    const result = loadedDict.lookup('cat')
    t.snapshot(result)
  })

  test('load - loads empty dictionary file', async (t) => {
    const loadedDict = await OpenDictionary.load(storagePath, emptyPath)

    t.truthy(loadedDict)
    t.deepEqual(loadedDict.lexicon(), [])

    const result = loadedDict.lookup('anything')
    t.deepEqual(result, [])
  })
}
