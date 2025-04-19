import tempfile
import unittest
import os
import uuid

from theopendictionary import Dictionary, FormKind


class TestForms(unittest.TestCase):
    def test_forms(self):
        xml_content = """
        <dictionary>
          <entry term="run">
            <forms>
              <form kind="superlative">ran</form>
              <form kind="inflection">running</form>
              <form>runs</form>
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

            # Look up the entry
            results = dictionary.lookup("run")

            # Check that we have one result
            self.assertEqual(len(results), 1)

            entry = results[0].entry

            # Check the forms
            self.assertEqual(len(entry.forms), 3)

            # Forms are stored properly with terms and kinds
            self.assertEqual(entry.forms[0].term, "ran")
            self.assertEqual(entry.forms[0].kind, FormKind.Superlative)

            self.assertEqual(entry.forms[1].term, "running")
            self.assertEqual(entry.forms[1].kind, FormKind.Inflection)

            self.assertEqual(entry.forms[2].term, "runs")
            self.assertIsNone(entry.forms[2].kind)
        finally:
            # Clean up
            if os.path.exists(temp_file):
                os.remove(temp_file)


if __name__ == "__main__":
    unittest.main()
