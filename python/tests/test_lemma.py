import tempfile
import unittest
import os
import uuid

from theopendictionary import Dictionary


class TestLemma(unittest.TestCase):
    def test_lemma(self):
        xml_content = """
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

            # Get the entries
            running_entry = running_results[0].entry
            ran_entry = ran_results[0].entry

            # Extract the first etymology
            running_etymology = running_entry.etymologies[0]
            ran_etymology = ran_entry.etymologies[0]

            # Get the first sense from each etymology (in Python it's an ordered list of key-value pairs)
            running_sense = list(running_etymology.senses.values())[0]
            ran_sense = list(ran_etymology.senses.values())[0]

            # Verify lemma references are on the sense objects
            self.assertIsNotNone(running_sense.lemma)
            self.assertEqual(running_sense.lemma, "run")

            self.assertIsNotNone(ran_sense.lemma)
            self.assertEqual(ran_sense.lemma, "run")

        finally:
            # Clean up
            if os.path.exists(temp_file):
                os.remove(temp_file)


if __name__ == "__main__":
    unittest.main()
