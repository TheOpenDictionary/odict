import { readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import test from 'ava'

import { compile, OpenDictionary } from '../index.js'

async function getDictionary(name: string) {
  return new OpenDictionary(
    compile(await readFile(join(fileURLToPath(new URL(import.meta.url)), `../../../examples/${name}.xml`), 'utf-8')),
  )
}

// Setup dictionaries for tests
let dict1: OpenDictionary
let dict2: OpenDictionary
let dict3: OpenDictionary

test.before(async () => {
  dict1 = await getDictionary('example1')
  dict2 = await getDictionary('example2')
  dict3 = await getDictionary('example3')
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
  // example1 has one entry with rank=100 (the "run" entry)
  t.is(dict1.minRank, 100)
})

test('rank - returns correct max_rank for dictionary with ranks', (t) => {
  // example1 has one entry with rank=100 (the "run" entry)
  t.is(dict1.maxRank, 100)
})

test('rank - returns null min_rank for dictionary without ranks', (t) => {
  // example2 has no rank attributes
  t.is(dict2.minRank, null)
})

test('rank - returns null max_rank for dictionary without ranks', (t) => {
  // example2 has no rank attributes
  t.is(dict2.maxRank, null)
})

test('rank - handles mixed entries with and without ranks', async (t) => {
  // Create a test dictionary with mixed rank entries
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

  const mixedDict = new OpenDictionary(compile(mixedXml))
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

test('should tokenize text case-sensitively by default', (t) => {
  const tokens = dict1.tokenize('DOG cat')

  t.is(tokens.length, 2)
  t.is(tokens[0].lemma, 'DOG')
  t.is(tokens[0].entries.length, 0) // "DOG" shouldn't match "dog"
  t.is(tokens[1].lemma, 'cat')
  t.is(tokens[1].entries[0].entry.term, 'cat')
})

test('should tokenize text case-insensitively when specified', (t) => {
  const tokens = dict1.tokenize('DOG cat', { insensitive: true })

  t.is(tokens.length, 2)
  t.is(tokens[0].lemma, 'DOG')
  t.is(tokens[0].entries.length, 1) // "DOG" should match "dog" with insensitivity
  t.is(tokens[0].entries[0].entry.term, 'dog')
  t.is(tokens[1].lemma, 'cat')
  t.is(tokens[1].entries[0].entry.term, 'cat')
})

test('should support follow=true for infinite following in tokenize', (t) => {
  const tokens = dict1.tokenize('ran', { follow: true })

  t.is(tokens.length, 1)
  t.is(tokens[0].lemma, 'ran')
  t.is(tokens[0].entries.length, 1)
  t.is(tokens[0].entries[0].entry.term, 'run')
  t.is(tokens[0].entries[0].directedFrom?.term, 'ran')
})

test('should support follow=false to disable following in tokenize', (t) => {
  const tokens = dict1.tokenize('ran', { follow: false })

  t.is(tokens.length, 1)
  t.is(tokens[0].lemma, 'ran')
  t.is(tokens[0].entries[0].entry.term, 'ran')
})

test('should support follow with specific number in tokenize', (t) => {
  const tokens = dict1.tokenize('ran', { follow: true })

  t.is(tokens.length, 1)
  t.is(tokens[0].lemma, 'ran')
  t.is(tokens[0].entries.length, 1)
  t.is(tokens[0].entries[0].entry.term, 'run')
  t.is(tokens[0].entries[0].directedFrom?.term, 'ran')
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
    const dict = new OpenDictionary('fake-alias')
    dict.lookup('dog')
  })

  t.true(!!error.message)
})

// Load tests - only run if OpenDictionary.load is available
if (typeof OpenDictionary.load === 'function') {
  const dict1Path = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example1.odict')
  const dict2Path = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example2.odict')
  const emptyPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/empty.odict')

  test.before('setup odict files', async () => {
    const dict1 = await getDictionary('example1')
    const dict2 = await getDictionary('example2')
    const empty = await getDictionary('empty')

    dict1.save(dict1Path)
    dict2.save(dict2Path)
    empty.save(emptyPath)
  })

  test('load - loads dictionary from local .odict file path', async (t) => {
    const dictPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example1.odict')

    const loadedDict = await OpenDictionary.load(dictPath)

    t.truthy(loadedDict)
    t.deepEqual(loadedDict.lexicon(), ['cat', 'dog', 'poo', 'ran', 'run'])

    // Verify it works the same as loading from compiled data
    const result = loadedDict.lookup('cat')
    t.snapshot(result)
  })

  test('load - loads dictionary without options', async (t) => {
    const dictPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example2.odict')

    const loadedDict = await OpenDictionary.load(dictPath)

    t.truthy(loadedDict)
    t.true(loadedDict.lexicon().length > 0)
  })

  test('load - throws error for non-existent file', async (t) => {
    const nonExistentPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/does-not-exist.odict')

    await t.throwsAsync(OpenDictionary.load(nonExistentPath))
  })

  test('load - throws error for invalid file format', async (t) => {
    const invalidPath = join(
      fileURLToPath(new URL(import.meta.url)),
      '../../../examples/example1.xml', // XML instead of .odict
    )

    await t.throwsAsync(OpenDictionary.load(invalidPath))
  })

  test('load - throws error for invalid dictionary name format', async (t) => {
    // Test invalid formats that would trigger download attempt
    const invalidFormats = [
      'invalid',
      'INVALID/EN', // uppercase
      'invalid/', // missing language
      '/english', // missing dictionary
      'invalid/en/extra', // too many parts
      'invalid@en', // wrong separator
      '123invalid/en', // numbers not allowed
    ]

    for (const format of invalidFormats) {
      await t.throwsAsync(OpenDictionary.load(format))
    }
  })

  test('load - loads empty dictionary file', async (t) => {
    const loadedDict = await OpenDictionary.load(emptyPath)

    t.truthy(loadedDict)
    t.deepEqual(loadedDict.lexicon(), [])

    const result = loadedDict.lookup('anything')
    t.deepEqual(result, [])
  })

  test('load - preserves dictionary functionality after loading', async (t) => {
    const loadedDict = await OpenDictionary.load(dict1Path)

    // Test all major functionality works
    t.truthy(loadedDict.lexicon())
    t.truthy(loadedDict.lookup('cat'))
    t.is(loadedDict.minRank, 100)
    t.is(loadedDict.maxRank, 100)

    // Test lookup with options
    const result = loadedDict.lookup('ran', { follow: true })
    t.is(result[0].entry.term, 'run')
    t.is(result[0].directedFrom?.term, 'ran')
  })

  test('load - loads different dictionary files correctly', async (t) => {
    const [loadedDict1, loadedDict2] = await Promise.all([
      OpenDictionary.load(dict1Path),
      OpenDictionary.load(dict2Path),
    ])

    // Verify they loaded different dictionaries
    const lexicon1 = loadedDict1.lexicon()
    const lexicon2 = loadedDict2.lexicon()

    t.notDeepEqual(lexicon1, lexicon2)
    t.not(loadedDict1.minRank, loadedDict2.minRank)
  })

  test('load - throws descriptive error for malformed .odict file', async (t) => {
    // Create a temporary file with invalid content
    const { writeFile, unlink } = await import('node:fs/promises')
    const { tmpdir } = await import('node:os')
    const tempPath = join(tmpdir(), 'malformed.odict')

    try {
      await writeFile(tempPath, 'invalid odict content')
      const error = await t.throwsAsync(OpenDictionary.load(tempPath))
      t.true(error.message.includes('The input does not have a valid ODict file signature'))
    } finally {
      try {
        await unlink(tempPath)
      } catch {}
    }
  })
}
