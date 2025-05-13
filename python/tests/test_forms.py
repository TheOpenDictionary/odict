import tempfile
import unittest
import os
import uuid

from theopendictionary import Dictionary, EnumWrapper


class TestForms(unittest.TestCase):
    def test_forms(self):
        xml_content = """
        <dictionary>
          <entry term="run">
            <ety>
              <sense>
                <definition value="To move quickly on foot." />
                <form kind="superlative" term="ran" />
                <form kind="inflection" term="running" />
                <form term="runs" />
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

            # Look up the entry
            results = dictionary.lookup("run")

            # Check that we have one result
            self.assertEqual(len(results), 1)

            entry = results[0].entry

            # Access the first etymology
            etymology = entry.etymologies[0]

            # Get the first sense (they're stored in a dict by part of speech)
            sense = list(etymology.senses.values())[0]

            # Forms are now at the Sense level
            self.assertEqual(len(sense.forms), 3)

            # Forms are stored properly with terms and kinds
            self.assertEqual(sense.forms[0].term, "ran")
            self.assertIsNotNone(sense.forms[0].kind)
            self.assertEqual(sense.forms[0].kind.variant, "superlative")
            self.assertEqual(sense.forms[0].kind.value, "superlative")

            self.assertEqual(sense.forms[1].term, "running")
            self.assertIsNotNone(sense.forms[1].kind)
            self.assertEqual(sense.forms[1].kind.variant, "inflection")
            self.assertEqual(sense.forms[1].kind.value, "inflection")

            self.assertEqual(sense.forms[2].term, "runs")
            self.assertIsNone(sense.forms[2].kind)
        finally:
            # Clean up
            if os.path.exists(temp_file):
                os.remove(temp_file)


if __name__ == "__main__":
    unittest.main()
