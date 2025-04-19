import tempfile
import unittest
import os
import uuid

from theopendictionary import Dictionary


class TestLemma(unittest.TestCase):
    def test_lemma(self):
        xml_content = """
        <dictionary>
          <entry term="running" lemma="run">
            <ety>
              <sense>
                <definition value="To move quickly on foot." />
              </sense>
            </ety>
          </entry>
          <entry term="ran" lemma="run">
            <ety>
              <sense>
                <definition value="Past tense of run." />
              </sense>
            </ety>
          </entry>
          <entry term="run">
            <forms>
              <form kind="past-tense">ran</form>
              <form kind="present-participle">running</form>
            </forms>
            <ety>
              <sense>
                <definition value="To move quickly on foot." />
              </sense>
            </ety>
          </entry>
        </dictionary>
        """

        # Create a temporary file
        temp_dir = tempfile.gettempdir()
        temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

        # Write XML content to an ODICT file
        Dictionary.write(xml_content, temp_file)

        try:
            # Create dictionary from the temporary ODICT file
            dictionary = Dictionary(temp_file)

            # Look up the entries
            running_results = dictionary.lookup("running")
            ran_results = dictionary.lookup("ran")

            # Check that we have one result for each
            self.assertEqual(len(running_results), 1)
            self.assertEqual(len(ran_results), 1)

            # Verify lemma references
            self.assertIsNotNone(running_results[0].entry.lemma)
            self.assertEqual(running_results[0].entry.lemma, "run")

            self.assertIsNotNone(ran_results[0].entry.lemma)
            self.assertEqual(ran_results[0].entry.lemma, "run")

        finally:
            # Clean up
            if os.path.exists(temp_file):
                os.remove(temp_file)


if __name__ == "__main__":
    unittest.main()
