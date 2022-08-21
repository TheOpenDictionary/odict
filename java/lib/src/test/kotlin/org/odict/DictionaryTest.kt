package org.odict

import kotlin.test.Test
import kotlin.test.assertNotEquals
import kotlin.test.assertNotNull

class DictionaryTest {
    @Test fun testLookup() {
        Dictionary.compile("../../examples/example1.xml")

        val dict = Dictionary("../../examples/example1.odict")
        val entry: String? = dict.lookup("run")

        println("FUCK")
        println(entry)
        assertNotNull(null)

//        assertNotEquals(entry.toJSON(), "{}")
    }
}
