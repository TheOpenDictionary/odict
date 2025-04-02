import os
import unittest
from theopendictionary import Dictionary

class TestTokenize(unittest.TestCase):
    def setUp(self):
        # Get the path to the test dictionary
        current_dir = os.path.dirname(os.path.abspath(__file__))
        self.dict_path = os.path.join(current_dir, "fixtures", "en_sample.odict")
        
        # Make sure the dictionary exists
        if not os.path.exists(self.dict_path):
            self.skipTest("Test dictionary not found")
        
        # Load the dictionary
        self.dictionary = Dictionary(self.dict_path)
    
    def test_tokenize(self):
        # Tokenize a sample text
        tokens = self.dictionary.tokenize("The quick brown fox jumps over the lazy dog")
        
        # Verify we got some tokens
        self.assertTrue(len(tokens) > 0)
        
        # Check token structure
        token = tokens[0]
        self.assertTrue(hasattr(token, 'lemma'))
        self.assertTrue(hasattr(token, 'entries'))
        
        # If there are entries, check their structure
        if len(token.entries) > 0:
            result = token.entries[0]
            self.assertTrue(hasattr(result, 'entry'))
            self.assertTrue(hasattr(result.entry, 'term'))

if __name__ == "__main__":
    unittest.main()
