package org.odict

import com.beust.klaxon.JsonArray
import com.beust.klaxon.Klaxon
import java.io.IOException
import java.util.concurrent.TimeUnit
import kotlin.io.path.deleteIfExists
import kotlin.io.path.pathString
import kotlin.io.path.writeText

private fun getJson(): String {
    return "[\n" +
            " [\n" +
            "  {\n" +
            "   \"term\": \"run\",\n" +
            "   \"etymologies\": [\n" +
            "    {\n" +
            "     \"description\": \"Latin root\",\n" +
            "     \"usages\": {\n" +
            "      \"n\": {\n" +
            "       \"pos\": \"n\",\n" +
            "       \"definitions\": [\n" +
            "        \"\",\n" +
            "        \"\",\n" +
            "        \"\",\n" +
            "        \"\",\n" +
            "        \"\",\n" +
            "        \"\"\n" +
            "       ],\n" +
            "       \"groups\": []\n" +
            "      },\n" +
            "      \"v\": {\n" +
            "       \"pos\": \"v\",\n" +
            "       \"definitions\": [],\n" +
            "       \"groups\": [\n" +
            "        {\n" +
            "         \"description\": \"A number of verb usages\",\n" +
            "         \"definitions\": [\n" +
            "          \"\\n            \\n            \\n          \",\n" +
            "          \"\",\n" +
            "          \"\",\n" +
            "          \"\",\n" +
            "          \"\",\n" +
            "          \"\"\n" +
            "         ]\n" +
            "        }\n" +
            "       ]\n" +
            "      }\n" +
            "     }\n" +
            "    }\n" +
            "   ]\n" +
            "  }\n" +
            " ]\n" +
            "]"
}

class Dictionary constructor(private val path: String) {

    fun lookup(vararg queries: String, split: Int = 0): List<Entry> {
        val json = execute("lookup", "-s", split.toString(), path, *queries)

        // print(json)

        if (json != null && json.trim().isNotEmpty()) {
            val myThing = Klaxon().parseArray<JsonArray<JsonArray<Entry>>>("""[[{
		"term": "run",
		"etymologies": [{
			"description": "Latin root",
			"usages": {
				"n": {
					"pos": "n",
					"definitions": [
						"",
						"",
						"",
						"",
						"",
						""
					],
					"groups": []
				},
				"v": {
					"pos": "v",
					"definitions": [],
					"groups": [{
						"description": "A number of verb usages",
						"definitions": [
							"\n            \n            \n          ",
							"",
							"",
							"",
							"",
							""
						]
					}]
				}
			}
		}]
	}]]""")
            print(myThing)
            // val myThing = Klaxon().parseArray<List<Entry>>(json) ?: ArrayList()
            // print(myThing)
            // return "hello"
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

    data class Usage(val id: String? = null, val description: String? = null, val definitions: List<String>? = null, val groups: List<Group>? = null)

    data class Group(val id: String? = null, val description: String? = null, val definitions: List<String>? = null)
}
