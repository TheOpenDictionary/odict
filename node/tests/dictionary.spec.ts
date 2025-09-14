import { readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { beforeAll, describe, expect, it } from 'vitest'

import { compile, OpenDictionary } from '../index.js'

async function getDictionary(name: string) {
  return new OpenDictionary(
    compile(await readFile(join(fileURLToPath(new URL(import.meta.url)), `../../../examples/${name}.xml`), 'utf-8')),
  )
}

describe('Dictionary', () => {
  expect.addSnapshotSerializer({
    test: (t) => typeof t.value === 'string' && !('variant' in t),
    serialize: (t) => `"${t.value}"`,
  })

  let dict1: OpenDictionary
  let dict2: OpenDictionary
  let dict3: OpenDictionary

  beforeAll(async () => {
    dict1 = await getDictionary('example1')
    dict2 = await getDictionary('example2')
    dict3 = await getDictionary('example3')
  })

  describe('lookup', () => {
    it('looks up terms properly', async () => {
      const result = dict1.lookup('cat')
      expect(result).toMatchSnapshot()
    })

    it("doesn't split unless specified", async () => {
      const result = dict1.lookup('catdog')
      expect(result.length).toBe(0)
    })

    it('follows terms properly', async () => {
      const result = dict1.lookup('ran', { follow: 1 })
      expect(result[0].entry.term).toBe('run')
      expect(result[0].directedFrom?.term).toBe('ran')
    })

    it('can split terms', async () => {
      const result = dict1.lookup('catdog', { split: 3 })
      expect(result).toMatchSnapshot()
    })

    it('is case-sensitive by default', async () => {
      const result = dict1.lookup('CAT')
      expect(result.length).toBe(0)
    })

    it('can perform case-insensitive lookup', async () => {
      const result = dict1.lookup('CAT', { insensitive: true })
      expect(result.length).toBe(1)
      expect(result[0].entry.term).toBe('cat')
    })

    it('works with mixed case', async () => {
      const result = dict1.lookup(['DoG', 'cAT'], { insensitive: true })
      expect(result.length).toBe(2)
      expect(result[0].entry.term).toBe('dog')
      expect(result[1].entry.term).toBe('cat')
    })

    it('combines case-insensitivity with follow option', async () => {
      const result = dict1.lookup('RaN', { follow: 1, insensitive: true })
      expect(result[0].entry.term).toBe('run')
      expect(result[0].directedFrom?.term).toBe('ran')
    })

    it('supports follow=true for infinite following', async () => {
      const result = dict1.lookup('ran', { follow: true })
      expect(result[0].entry.term).toBe('run')
      expect(result[0].directedFrom?.term).toBe('ran')
    })

    it('supports follow=false to disable following', async () => {
      const result = dict1.lookup('ran', { follow: false })
      expect(result[0].entry.term).toBe('ran')
    })

    it('supports follow with specific number', async () => {
      const result = dict1.lookup('ran', { follow: 5 })
      expect(result[0].entry.term).toBe('run')
      expect(result[0].directedFrom?.term).toBe('ran')
    })
  })

  it('can return the lexicon', async () => {
    const result = dict1.lexicon()
    expect(result).toStrictEqual(['cat', 'dog', 'poo', 'ran', 'run'])
  })

  describe('rank', () => {
    it('returns correct min_rank for dictionary with ranks', () => {
      // example1 has one entry with rank=100 (the "run" entry)
      expect(dict1.minRank).toBe(100)
    })

    it('returns correct max_rank for dictionary with ranks', () => {
      // example1 has one entry with rank=100 (the "run" entry)
      expect(dict1.maxRank).toBe(100)
    })

    it('returns null min_rank for dictionary without ranks', () => {
      // example2 has no rank attributes
      expect(dict2.minRank).toBe(null)
    })

    it('returns null max_rank for dictionary without ranks', () => {
      // example2 has no rank attributes
      expect(dict2.maxRank).toBe(null)
    })

    it('handles mixed entries with and without ranks', async () => {
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
      expect(mixedDict.minRank).toBe(10)
      expect(mixedDict.maxRank).toBe(100)
    })
  })

  it('should tokenize text and find entries', () => {
    const tokens = dict3.tokenize('你好！你是谁？')

    expect(tokens).toMatchSnapshot()
    expect(tokens.length).toBeGreaterThan(0)
    expect(tokens[0].lemma).toBe('你好')
    expect(tokens[0].entries[0].entry.term).toBe('你')
    expect(tokens[0].entries[1].entry.term).toBe('好')
  })

  it('should tokenize text case-sensitively by default', () => {
    const tokens = dict1.tokenize('DOG cat')

    expect(tokens.length).toBe(2)
    expect(tokens[0].lemma).toBe('DOG')
    expect(tokens[0].entries.length).toBe(0) // "DOG" shouldn't match "dog"
    expect(tokens[1].lemma).toBe('cat')
    expect(tokens[1].entries[0].entry.term).toBe('cat')
  })

  it('should tokenize text case-insensitively when specified', () => {
    const tokens = dict1.tokenize('DOG cat', { insensitive: true })

    expect(tokens.length).toBe(2)
    expect(tokens[0].lemma).toBe('DOG')
    expect(tokens[0].entries.length).toBe(1) // "DOG" should match "dog" with insensitivity
    expect(tokens[0].entries[0].entry.term).toBe('dog')
    expect(tokens[1].lemma).toBe('cat')
    expect(tokens[1].entries[0].entry.term).toBe('cat')
  })

  it('should support follow=true for infinite following in tokenize', () => {
    const tokens = dict1.tokenize('ran', { follow: true })

    expect(tokens.length).toBe(1)
    expect(tokens[0].lemma).toBe('ran')
    expect(tokens[0].entries.length).toBe(1)
    expect(tokens[0].entries[0].entry.term).toBe('run')
    expect(tokens[0].entries[0].directedFrom?.term).toBe('ran')
  })

  it('should support follow=false to disable following in tokenize', () => {
    const tokens = dict1.tokenize('ran', { follow: false })

    expect(tokens.length).toBe(1)
    expect(tokens[0].lemma).toBe('ran')
    expect(tokens[0].entries[0].entry.term).toBe('ran')
  })

  it('should support follow with specific number in tokenize', () => {
    const tokens = dict1.tokenize('ran', { follow: 5 })

    expect(tokens.length).toBe(1)
    expect(tokens[0].lemma).toBe('ran')
    expect(tokens[0].entries.length).toBe(1)
    expect(tokens[0].entries[0].entry.term).toBe('run')
    expect(tokens[0].entries[0].directedFrom?.term).toBe('ran')
  })

  it.skipIf(process.env.NAPI_RS_FORCE_WASI)('can index and search a dictionary', async () => {
    dict1.index()

    const results = dict1.search('run')

    expect(results).toMatchSnapshot()
  })

  it('throws errors inside JavaScript', async () => {
    try {
      // @ts-expect-error
      const dict = new OpenDictionary('fake-alias')
      dict.lookup('dog')
    } catch (e) {
      expect((e as Error).message).toContain('Failed to create reference from Buffer')
    }
  })

  describe.skipIf(typeof OpenDictionary.load !== 'function')('load', () => {
    const dict1Path = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example1.odict')

    const dict2Path = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example2.odict')

    const emptyPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/empty.odict')

    beforeAll(async () => {
      const dict1 = await getDictionary('example1')
      const dict2 = await getDictionary('example2')
      const empty = await getDictionary('empty')

      dict1.save(dict1Path)
      dict2.save(dict2Path)
      empty.save(emptyPath)
    })

    it('loads dictionary from local .odict file path', async () => {
      const dictPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example1.odict')

      const loadedDict = await OpenDictionary.load(dictPath)

      expect(loadedDict).toBeDefined()

      expect(loadedDict.lexicon()).toStrictEqual(['cat', 'dog', 'poo', 'ran', 'run'])

      // Verify it works the same as loading from compiled data
      const result = loadedDict.lookup('cat')
      expect(result).toMatchSnapshot()
    })

    it('loads dictionary without options', async () => {
      const dictPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/example2.odict')

      const loadedDict = await OpenDictionary.load(dictPath)

      expect(loadedDict).toBeDefined()
      expect(loadedDict.lexicon().length).toBeGreaterThan(0)
    })

    it('throws error for non-existent file', async () => {
      const nonExistentPath = join(fileURLToPath(new URL(import.meta.url)), '../../../examples/does-not-exist.odict')

      await expect(OpenDictionary.load(nonExistentPath)).rejects.toThrow()
    })

    it('throws error for invalid file format', async () => {
      const invalidPath = join(
        fileURLToPath(new URL(import.meta.url)),
        '../../../examples/example1.xml', // XML instead of .odict
      )

      await expect(OpenDictionary.load(invalidPath)).rejects.toThrow()
    })

    it('throws error for invalid dictionary name format', async () => {
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
        await expect(OpenDictionary.load(format)).rejects.toThrow()
      }
    })

    describe.skipIf(process.env.NO_NETWORK)('network tests', () => {
      it('handles download failure', { timeout: 30_000 }, async () => {
        const validFormat = 'wiktionary/some-fake-dict'
        await expect(OpenDictionary.load(validFormat)).rejects.toThrow(/E_HTTP_404/)
      })

      it('handles download success', { timeout: 30_000 }, async () => {
        const validFormat = 'wiktionary/jpn'
        expect(await OpenDictionary.load(validFormat)).toBeDefined()
      })
    })

    it('loads empty dictionary file', async () => {
      const loadedDict = await OpenDictionary.load(emptyPath)

      expect(loadedDict).toBeDefined()
      expect(loadedDict.lexicon()).toStrictEqual([])

      const result = loadedDict.lookup('anything')
      expect(result).toStrictEqual([])
    })

    it('preserves dictionary functionality after loading', async () => {
      const loadedDict = await OpenDictionary.load(dict1Path)

      // Test all major functionality works
      expect(loadedDict.lexicon()).toBeDefined()
      expect(loadedDict.lookup('cat')).toBeDefined()
      expect(loadedDict.minRank).toBe(100)
      expect(loadedDict.maxRank).toBe(100)

      // Test lookup with options
      const result = loadedDict.lookup('ran', { follow: 1 })
      expect(result[0].entry.term).toBe('run')
      expect(result[0].directedFrom?.term).toBe('ran')
    })

    it('loads different dictionary files correctly', async () => {
      const [loadedDict1, loadedDict2] = await Promise.all([
        OpenDictionary.load(dict1Path),
        OpenDictionary.load(dict2Path),
      ])

      // Verify they loaded different dictionaries
      const lexicon1 = loadedDict1.lexicon()
      const lexicon2 = loadedDict2.lexicon()

      expect(lexicon1).not.toStrictEqual(lexicon2)
      expect(loadedDict1.minRank).not.toBe(loadedDict2.minRank)
    })

    it('throws descriptive error for malformed .odict file', async () => {
      // Create a temporary file with invalid content
      const { writeFile, unlink } = await import('node:fs/promises')
      const { tmpdir } = await import('node:os')
      const tempPath = join(tmpdir(), 'malformed.odict')

      try {
        await writeFile(tempPath, 'invalid odict content')
        await expect(OpenDictionary.load(tempPath)).rejects.toThrow(
          'The input does not have a valid ODict file signature',
        )
      } finally {
        try {
          await unlink(tempPath)
        } catch {}
      }
    })
  })
})
