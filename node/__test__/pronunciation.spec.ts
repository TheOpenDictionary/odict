import test from 'ava'
import { compile, OpenDictionary, type Definition } from '../index'

test('Pronunciation support - should parse entries with pronunciations', async (t) => {
  const xml = `
    <dictionary>
      <entry term="你好">
        <ety>
          <pronunciation kind="pinyin" value="ni hao">
            <url src="./audio.mp3" />
          </pronunciation>
        </ety>
      </entry>
    </dictionary>
  `

  const compiled = compile(xml)
  const dict = new OpenDictionary(compiled)

  const results = dict.lookup('你好')
  t.is(results.length, 1)

  const entry = results[0].entry
  t.is(entry.etymologies.length, 1)
  t.truthy(entry.etymologies[0].pronunciations)
  t.is(entry.etymologies[0].pronunciations.length, 1)
  t.deepEqual(entry.etymologies[0].pronunciations[0].kind, {
    name: 'PronunciationKind',
    value: 'pinyin',
    variant: 'pinyin',
  })
  t.is(entry.etymologies[0].pronunciations[0].value, 'ni hao')
  t.is(entry.etymologies[0].pronunciations[0].media.length, 1)
  t.is(entry.etymologies[0].pronunciations[0].media[0].src, './audio.mp3')
})

test('Pronunciation support - should parse examples with pronunciations', async (t) => {
  const xml = `
    <dictionary>
      <entry term="example">
        <ety>
          <sense pos="n">
            <definition value="An example definition">
              <example value="An example sentence">
                <pronunciation kind="ipa" value="ɪɡˈzæmpl ˈsɛntəns">
                  <url src="./example.mp3" type="audio/mpeg" />
                </pronunciation>
              </example>
            </definition>
          </sense>
        </ety>
      </entry>
    </dictionary>
  `

  const compiled = compile(xml)
  const dict = new OpenDictionary(compiled)

  const results = dict.lookup('example')
  t.is(results.length, 1)

  const entry = results[0].entry
  const example = (entry.etymologies[0].senses['n'].definitions[0] as Definition).examples[0]

  t.truthy(example.pronunciations)
  t.is(example.pronunciations.length, 1)
  t.deepEqual(example.pronunciations[0].kind, {
    name: 'PronunciationKind',
    value: 'ipa',
    variant: 'ipa',
  })
  t.is(example.pronunciations[0].value, 'ɪɡˈzæmpl ˈsɛntəns')
  t.is(example.pronunciations[0].media.length, 1)
  t.is(example.pronunciations[0].media[0].src, './example.mp3')
  t.is(example.pronunciations[0].media[0].mimeType, 'audio/mpeg')
})

test('Pronunciation support - should handle multiple pronunciations for an entry', async (t) => {
  const xml = `
    <dictionary>
      <entry term="hello">
        <ety>
          <pronunciation kind="ipa" value="həˈləʊ">
            <url src="./hello-british.mp3" />
          </pronunciation>
          <pronunciation kind="ipa" value="hɛˈloʊ">
            <url src="./hello-american.mp3" />
          </pronunciation>
          <sense pos="adj">
            <definition value="A greeting" />
          </sense>
        </ety>
      </entry>
    </dictionary>
  `

  const compiled = compile(xml)
  const dict = new OpenDictionary(compiled)

  const results = dict.lookup('hello')
  t.is(results.length, 1)

  const entry = results[0].entry
  t.is(entry.etymologies.length, 1)
  t.is(entry.etymologies[0].pronunciations.length, 2)
  t.is(entry.etymologies[0].pronunciations[0].value, 'həˈləʊ')
  t.is(entry.etymologies[0].pronunciations[1].value, 'hɛˈloʊ')
})
