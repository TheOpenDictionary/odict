package org.odict

import com.squareup.moshi.JsonAdapter
import com.squareup.moshi.Moshi
import com.squareup.moshi.Types
import com.squareup.moshi.kotlin.reflect.KotlinJsonAdapterFactory
import java.io.IOException
import java.util.concurrent.TimeUnit
import kotlin.io.path.deleteIfExists
import kotlin.io.path.pathString
import kotlin.io.path.writeText

class Dictionary constructor(private val path: String) {

    fun lookup(vararg queries: String, follow: Boolean = false, split: Int = 0): List<List<Entry>> {
        var args = arrayOf("lookup", "-f", "json")
        
        if (follow) {
          args += "-F"
        }

        args += arrayOf("-s", split.toString(), path)

        val resultJson = execute(*args, *queries)
        
        return resultJson?.let { lookupAdapter.fromJson(it) } ?: emptyList()
    }

    fun index() {
        execute("index", this.path)
    }

    fun lexicon(): List<String> {
        val lexicon = execute("lexicon", this.path)
        return lexicon?.trim()?.split("\n") ?: emptyList()
    }

    fun search(vararg queries: String, index: Boolean = false): List<Entry> {
        var args = arrayOf("search", this.path, *queries)

        if (index) {
            args = arrayOf("search", "-i", this.path, *queries)
        }

        val output = execute(*args)
        return output?.let { searchAdapter.fromJson(it) } ?: emptyList()
    }

    companion object {
        private val moshi = Moshi.Builder().addLast(KotlinJsonAdapterFactory()).build()

        private val innerLookupType = Types.newParameterizedType(MutableList::class.java, Entry::class.java)
        private val outerLookupType = Types.newParameterizedType(MutableList::class.java, innerLookupType)
        private val lookupAdapter: JsonAdapter<List<List<Entry>>> = moshi.adapter(outerLookupType)

        private val searchType = Types.newParameterizedType(List::class.java, Entry::class.java)
        private val searchAdapter = moshi.adapter<List<Entry>>(searchType)

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
                val baseArgs = arrayOf(if (System.getenv("RUNTIME_ENV") == "test") "../../bin/odict" else "odict", "--quiet")
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

    data class Description(val id: String? = null, val value: String? = null, val examples: List<String>? = null)

    data class Usage(val id: String? = null, val description: String? = null, val definitions: List<Description>? = null, val groups: List<Group>? = null)

    data class Group(val id: String? = null, val description: String? = null, val definitions: List<Description>? = null)
}
