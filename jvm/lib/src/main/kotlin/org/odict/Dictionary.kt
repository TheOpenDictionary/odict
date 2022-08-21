package org.odict

import com.beust.klaxon.Klaxon
import java.io.IOException
import java.util.concurrent.TimeUnit
import kotlin.io.path.deleteIfExists
import kotlin.io.path.pathString
import kotlin.io.path.writeText

class Dictionary constructor(private val path: String) {

    fun lookup(vararg queries: String, split: Int = 0): List<Entry> {
        val json = execute("lookup", "-s", split.toString(), path, *queries)

        if (json != null && json.trim().isNotEmpty()) {
            return Klaxon().parseArray<Entry>(json) ?: ArrayList()
        }

        return ArrayList()
    }

    fun index() {
        execute("index", this.path)
    }

    fun search(vararg queries: String, index: Boolean = false): List<Entry> {
        var args = arrayOf("search", this.path, *queries)

        if (index) {
            args = arrayOf("search", "-i", this.path, *queries)
        }

        val output = execute(*args)

        if (output != null && output.trim().isNotEmpty()) {
            return Klaxon().parseArray<Entry>(output) ?: ArrayList()
        }

        return ArrayList()
    }

    companion object {
        fun compile(path: String) {
            this.execute("compile", path)
        }

        fun write(xml: String, path: String) {
            val tmp = kotlin.io.path.createTempFile()
            tmp.writeText(xml)
            this.execute("compile", "-o", path, tmp.pathString)
            tmp.deleteIfExists()
        }

        private fun execute(vararg args: String): String? {
            return try {
                val baseArgs = arrayOf(if (System.getenv("RUNTIME_ENV") == "test") "../../build/odict" else "odict", "--quiet")
                val proc = ProcessBuilder(*(baseArgs + args))
                        .redirectOutput(ProcessBuilder.Redirect.PIPE)
                        .start()

                proc.waitFor(3, TimeUnit.MINUTES)

                val err = proc.errorStream.bufferedReader().readText()

                if (err.isNotEmpty()) {
                    throw Throwable(err)
                }

                proc.inputStream.bufferedReader().readText()
            } catch(e: IOException) {
                e.printStackTrace()
                null
            }
        }
    }

    data class Entry(val id: String? = null, val term: String, val etymologies: List<Etymology>? = null)

    data class Etymology(val id: String? = null, val description: String? = null, val usages: Map<String, Usage>? = null)

    data class Usage(val id: String? = null, val description: String? = null, val definitions: List<String>? = null, val groups: List<Group>? = null)

    data class Group(val id: String? = null, val description: String? = null, val definitions: List<String>? = null)
}
