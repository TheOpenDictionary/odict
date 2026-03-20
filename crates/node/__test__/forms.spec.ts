import test from 'ava'
import { writeFile } from 'node:fs/promises'
import { join } from 'node:path'
import { tmpdir } from 'node:os'
import { randomUUID } from 'node:crypto'

import { compile, OpenDictionary } from '../index.js'

test('Form support - should handle entries with forms', async (t) => {
  const xmlContent = `
    <dictionary>
      <entry term="run">
        <ety>
          <sense>
            <definition value="To move quickly on foot." />
            <form kind="inflection" term="ran" />
            <form kind="superlative" term="running" />
            <form term="runs" />
          </sense>
        </ety>
      </entry>
    </dictionary>
  `

  // Create a temporary file
  const tempFile = join(tmpdir(), `${randomUUID()}.xml`)
  await writeFile(tempFile, xmlContent, 'utf-8')

  // Compile and load the dictionary
  const compiled = compile(xmlContent)
  const dict = new OpenDictionary(compiled)

  // Look up the entry
  const results = dict.lookup('run')

  // Check that we have one result
  t.is(results.length, 1)

  const entry = results[0].entry

  // Get the etymology and sense
  const etymology = entry.etymologies[0]
  const sense = Object.values(etymology.senses)[0]

  // Check the forms in the sense
  t.is(sense.forms.length, 3)

  // Forms are stored properly with terms and kinds
  t.is(sense.forms[0].term, 'ran')
  t.deepEqual(sense.forms[0].kind, {
    name: 'FormKind',
    value: 'inflection',
    variant: 'inflection',
  })

  t.is(sense.forms[1].term, 'running')
  t.deepEqual(sense.forms[1].kind, {
    name: 'FormKind',
    value: 'superlative',
    variant: 'superlative',
  })

  t.is(sense.forms[2].term, 'runs')
  t.is(sense.forms[2].kind, undefined) // Optional kind is null when not specified
})

test('Lemma support - should handle entries with lemma references', async (t) => {
  const xmlContent = `
    <dictionary>
      <entry term="running">
        <ety>
          <sense lemma="run">
            <definition value="To move quickly on foot." />
          </sense>
        </ety>
      </entry>
      <entry term="ran">
        <ety>
          <sense lemma="run">
            <definition value="Past tense of run." />
          </sense>
        </ety>
      </entry>
      <entry term="run">
        <form kind="past-tense" term="ran" />
        <form kind="present-participle" term="running" />
        <ety>
          <sense>
            <definition value="To move quickly on foot." />
          </sense>
        </ety>
      </entry>
    </dictionary>
  `

  // Create a temporary file
  const tempFile = join(tmpdir(), `${randomUUID()}.xml`)
  await writeFile(tempFile, xmlContent, 'utf-8')

  // Compile and load the dictionary
  const compiled = compile(xmlContent)
  const dict = new OpenDictionary(compiled)

  // Look up the entries
  const runningResults = dict.lookup('running')
  const ranResults = dict.lookup('ran')

  // Check that we have one result for each
  t.is(runningResults.length, 1)
  t.is(ranResults.length, 1)

  // Get the entries
  const runningEntry = runningResults[0].entry
  const ranEntry = ranResults[0].entry

  // Extract the first etymology
  const runningEtymology = runningEntry.etymologies[0]
  const ranEtymology = ranEntry.etymologies[0]

  // Get the senses (they're in an object keyed by part of speech)
  // The default part of speech is 'n' (noun) from the XML
  const runningSense = Object.values(runningEtymology.senses)[0]
  const ranSense = Object.values(ranEtymology.senses)[0]

  // Verify lemma references are on the sense objects
  t.truthy(runningSense.lemma)
  t.is(runningSense.lemma, 'run')

  t.truthy(ranSense.lemma)
  t.is(ranSense.lemma, 'run')
})
