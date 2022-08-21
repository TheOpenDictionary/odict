package org.odict

import java.io.IOException
import java.util.concurrent.TimeUnit

class Dictionary constructor(path: String) {

    fun lookup(vararg queries: String, split: Int = 0): String? {
        return Dictionary.execute("lookup", "-s", split.toString(), *queries)
    }

    companion object {
        fun compile(path: String) {
            this.execute("compile", path)
        }

        private fun execute(vararg args: String): String? {
            return try {
                val baseArgs = arrayOf("odict", "--quiet")
                val proc = ProcessBuilder(*(baseArgs + args))
                        .redirectOutput(ProcessBuilder.Redirect.PIPE)
                        .redirectError(ProcessBuilder.Redirect.PIPE)
                        .start()

                proc.waitFor(3, TimeUnit.MINUTES)


                println( proc.inputStream.bufferedReader().readText())
                proc.inputStream.bufferedReader().readText()
            } catch(e: IOException) {
                e.printStackTrace()
                null
            }
        }
    }
}
