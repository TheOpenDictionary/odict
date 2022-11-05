package org.odict

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

class DictionaryTest {
    @Test fun testLookup() {
        Dictionary.compile("../../examples/example1.xml")

        val dict = Dictionary("../../examples/example1.odict")
        val entry = dict.lookup("run")

        // print(entry)

        // assertEquals("run", entry[0].term)
        // assertEquals(6, entry[0].etymologies?.get(0)?.usages?.get("v")?.groups?.get(0)?.definitions?.count())
    }

    @Test
    @Throws(Exception::class)
    fun testSearch() {
        Dictionary.compile("../../examples/example1.xml")

        val dict = Dictionary("../../examples/example1.odict")

        dict.index()

        val json = dict.search("run")

        assertEquals(1, json.count())
        // assertEquals("run", json[0][0].term)
    }

    @Test
    @Throws(Exception::class)
    fun testWrite() {
        Dictionary.write(
                "<dictionary><entry term=\"hello\"><ety><usage pos=\"v\"><definition>hello world</definition></usage></ety></entry></dictionary>",
                "test.odict")
        val dict = Dictionary("test.odict")
        val entries = dict.lookup("hello")

        // assertEquals("hello", entries[0][0].term)
        // assertEquals("hello world", entries[0][0].etymologies?.get(0)?.usages?.get("v")?.definitions?.get(0))
    }
}
