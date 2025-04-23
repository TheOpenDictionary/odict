# OpenDictionary Pronunciation Feature

The pronunciation feature in OpenDictionary allows you to include phonetic representations and audio pronunciations for entries and examples in your dictionaries.

## Structure

Pronunciations in OpenDictionary consist of:

1. A **kind** attribute specifying the pronunciation system (IPA, Pinyin, etc.)
2. A **value** attribute containing the actual pronunciation
3. Optional **URL** elements pointing to audio files

## Supported Pronunciation Systems

The following pronunciation systems are built-in:

- **IPA** - International Phonetic Alphabet
- **Pinyin** - Standard romanization system for Mandarin Chinese
- **Hiragana** - Japanese syllabary
- **Romaji** - Romanization of Japanese
- **Katakana** - Japanese syllabary for foreign words
- **Yale** - Romanization system for Korean
- **Jyutping** - Romanization system for Cantonese
- **Bopomofo** - Phonetic notation for Chinese (Zhuyin)
- **Hepburn** - Romanization system for Japanese

You can also use custom pronunciation systems by specifying any string as the kind.

## Usage Examples

### Basic Entry-Level Pronunciation

```xml
<entry term="hello">
  <pronunciation kind="ipa" value="həˈləʊ">
    <url src="./audio/hello.mp3" type="audio/mpeg" />
  </pronunciation>
  <!-- ... rest of entry ... -->
</entry>
```

### Multiple Pronunciations (e.g., for different regional variants)

```xml
<entry term="tomato">
  <pronunciation kind="ipa" value="təˈmeɪtoʊ">
    <url src="./audio/tomato_us.mp3" type="audio/mpeg" description="US pronunciation" />
  </pronunciation>
  <pronunciation kind="ipa" value="təˈmɑːtəʊ">
    <url src="./audio/tomato_uk.mp3" type="audio/mpeg" description="UK pronunciation" />
  </pronunciation>
</entry>
```

### Non-Latin Script Languages

```xml
<entry term="你好">
  <pronunciation kind="pinyin" value="nǐ hǎo">
    <url src="./audio/nihao.mp3" type="audio/mpeg" />
  </pronunciation>
  <pronunciation kind="ipa" value="ni˨˩ xɑʊ̯˧˥" />
</entry>
```

### Example-Level Pronunciation

Attach pronunciations to specific examples:

```xml
<example value="Hello, how are you?">
  <pronunciation kind="ipa" value="həˈləʊ, haʊ ɑː juː?">
    <url src="./audio/hello_how_are_you.mp3" type="audio/mpeg" />
  </pronunciation>
</example>
```

### Using Custom Pronunciation Systems

You can use any string as the pronunciation kind:

```xml
<pronunciation kind="wadegiles" value="Pei-ching">
  <url src="./audio/beijing_wade.mp3" type="audio/mpeg" />
</pronunciation>
```

## Audio Files

The `<url>` element supports:

- `src` attribute (required): Path or URL to the audio file
- `type` attribute (optional): MIME type of the audio file
- `description` attribute (optional): Text description of the audio

You can include multiple audio files for a single pronunciation if needed.

## Best Practices

1. **Consistency**: Use the same pronunciation system throughout your dictionary
2. **Audio Quality**: Use high-quality audio recordings in compressed formats (mp3, ogg)
3. **Include Common Systems**: For major languages, include at least IPA along with any language-specific system
4. **Normalize Audio**: Ensure consistent volume levels across all audio files
5. **Use Relative Paths**: For audio files, use relative paths to maintain portability

See the [pronunciation_example.xml](../examples/pronunciation_example.xml) file for a comprehensive example of the pronunciation feature in action.