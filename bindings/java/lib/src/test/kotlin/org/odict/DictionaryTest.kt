package org.odict

import kotlin.test.Test
import kotlin.test.assertEquals

class DictionaryTest {
    @Test fun testLookup() {
        Dictionary.compile("../../examples/example2.xml")

        val dict = Dictionary("../../examples/example2.odict")
        val entry = dict.lookup("markdown")

        assertEquals("This <strong>is</strong> a <em>markdown</em> test", entry[0][0].etymologies?.get(0)?.senses?.get("v")?.definitions?.get(0)?.value);
    }

    @Test fun testLookupWithoutMarkdown() {
        Dictionary.compile("../../examples/example2.xml")

        val dict = Dictionary("../../examples/example2.odict")
        val entry = dict.lookup("markdown", markdownStrategy = "disable")

        assertEquals("This **is** a _markdown_ test", entry[0][0].etymologies?.get(0)?.senses?.get("v")?.definitions?.get(0)?.value);
    }

        @Test fun testLookupWithPlainText() {
        Dictionary.compile("../../examples/example2.xml")

        val dict = Dictionary("../../examples/example2.odict")
        val entry = dict.lookup("markdown", markdownStrategy = "text")

        assertEquals("This is a markdown test", entry[0][0].etymologies?.get(0)?.senses?.get("v")?.definitions?.get(0)?.value);
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
                "<dictionary><entry term=\"hello\"><ety><sense pos=\"v\"><definition value=\"hello world\" /></sense></ety></entry></dictionary>",
                "test.odict")
        
        val dict = Dictionary("test.odict")
        val entries = dict.lookup("hello")

        assertEquals("hello", entries[0][0].term)
        assertEquals("hello world", entries[0][0].etymologies?.get(0)?.senses?.get("v")?.definitions?.get(0)?.value)
    }
}
