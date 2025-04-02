import { readFileSync } from 'fs';
import { join } from 'path';
import { Dictionary } from '../index';

describe('tokenize', () => {
  let dict: Dictionary;

  beforeAll(() => {
    const buffer = readFileSync(join(__dirname, './fixtures/en_sample.odict'));
    dict = new Dictionary(buffer);
  });

  it('should tokenize text and find entries', () => {
    const tokens = dict.tokenize('The quick brown fox jumps over the lazy dog');
    
    // Verify we got some tokens
    expect(tokens.length).toBeGreaterThan(0);
    
    // Check token structure
    const token = tokens[0];
    expect(token).toHaveProperty('lemma');
    expect(token).toHaveProperty('entries');
    
    // If there are entries, check their structure
    if (token.entries.length > 0) {
      const result = token.entries[0];
      expect(result).toHaveProperty('entry');
      expect(result.entry).toHaveProperty('term');
    }
  });
});
