package org.odict

import kotlin.test.Test
import kotlin.test.assertEquals

class DictionaryTest {
    @Test fun testLookup() {
        Dictionary.compile("../../examples/example1.xml")

        val dict = Dictionary("../../examples/example1.odict")
        val entry = dict.lookup("run")

        assertEquals("run", entry[0][0].term)
        assertEquals(6, entry[0][0].etymologies?.get(0)?.usages?.get("v")?.groups?.get(0)?.definitions?.count())
    }

    @Test fun testLexicon() {
        Dictionary.compile("../../examples/example1.xml")

        val dict = Dictionary("../../examples/example1.odict")
        val lexicon = dict.lexicon()

        assertEquals(lexicon, listOf("cat", "dog", "poo", "ran", "run"))
    }

    @Test
    @Throws(Exception::class)
    fun testSearch() {
        Dictionary.compile("../../examples/example1.xml")

        val dict = Dictionary("../../examples/example1.odict")

        dict.index()

        val json = dict.search("run")

        assertEquals(2, json.count())
        assertEquals("ran", json[0].term)
        assertEquals("run", json[1].term)
    }

    @Test
    @Throws(Exception::class)
    fun testWrite() {
        Dictionary.write(
                "<dictionary><entry term=\"hello\"><ety><usage pos=\"v\"><definition value=\"hello world\" /></usage></ety></entry></dictionary>",
                "test.odict")
        
        val dict = Dictionary("test.odict")
        val entries = dict.lookup("hello")

        assertEquals("hello", entries[0][0].term)
        assertEquals("hello world", entries[0][0].etymologies?.get(0)?.usages?.get("v")?.definitions?.get(0)?.value)
    }
}
