---
title: XML Schema Reference
description: Complete reference for the ODict XML (ODXML) schema.
---

## Element hierarchy

```text
dictionary
└── entry
    ├── ety
    │   ├── pronunciation
    │   │   └── url
    │   └── sense
    │       ├── group
    │       │   └── definition …
    │       ├── definition
    │       │   ├── example
    │       │   │   ├── translation …
    │       │   │   └── pronunciation …
    │       │   └── note
    │       │       └── example …
    │       ├── tag …
    │       ├── translation …
    │       └── form
    │           └── tag …
    └── media …
```

Repeatable child elements can appear in any order. Unless otherwise noted,
elements with a minimum of `0` can be omitted.

---

## Elements

### `<dictionary>`

#### Attributes

| Attribute | Type     | Required |
| --------- | -------- | -------- |
| `id`      | `string` | No       |
| `name`    | `string` | No       |

#### Child elements

| Element             | Min | Max       |
| ------------------- | --- | --------- |
| [`<entry>`](#entry) | 0   | unbounded |

---

### `<entry>`

#### Attributes

| Attribute | Type             | Required |
| --------- | ---------------- | -------- |
| `term`    | `string`         | Yes      |
| `see`     | entry term       | No       |
| `rank`    | unsigned integer | No       |

#### Child elements

| Element             | Min | Max       |
| ------------------- | --- | --------- |
| [`<ety>`](#ety)     | 0   | unbounded |
| [`<media>`](#media) | 0   | unbounded |

`see` identifies another entry by its `term`. When redirect following is
enabled, an entry without etymologies can use it to redirect lookup to the referenced
entry. `rank` stores source-specific frequency or importance data; ODict does
not prescribe whether larger or smaller numbers are more important.

---

### `<ety>`

#### Attributes

| Attribute     | Type     | Required |
| ------------- | -------- | -------- |
| `id`          | `string` | No       |
| `description` | `string` | No       |

#### Child elements

| Element                             | Min | Max       |
| ----------------------------------- | --- | --------- |
| [`<pronunciation>`](#pronunciation) | 0   | unbounded |
| [`<sense>`](#sense)                 | 0   | unbounded |

---

### `<sense>`

#### Attributes

| Attribute | Type     | Required |
| --------- | -------- | -------- |
| `pos`     | `string` | No       |
| `lemma`   | `string` | No       |

The `lemma` attribute identifies the headword that is the canonical form for this
sense. Its value should match the `term` of another `<entry>`.

Because `lemma` belongs to `<sense>`, a single entry can link different senses to
different headwords. For example, the verb sense of “saw” is a form of “see”,
while the noun sense is already its own lemma:

```xml
<entry term="saw">
  <ety>
    <sense pos="v" lemma="see">
      <definition value="The past tense of see" />
    </sense>
    <sense pos="n">
      <definition value="A tool with a toothed blade" />
    </sense>
  </ety>
</entry>

<entry term="see">
  <ety>
    <sense pos="v">
      <form kind="past-tense" term="saw" />
      <definition value="To perceive with the eyes" />
    </sense>
  </ety>
</entry>
```

ODict preserves the lemma reference on the sense for applications to inspect. It
does not redirect a lookup automatically. Use [`see` on `<entry>`](#entry) when
one entry should redirect to another, and use `<form>` on the lemma's sense to
describe the relationship in the other direction.

#### Child elements

| Element                         | Min | Max       |
| ------------------------------- | --- | --------- |
| [`<group>`](#group)             | 0   | unbounded |
| [`<definition>`](#definition)   | 0   | unbounded |
| [`<tag>`](#tag)                 | 0   | unbounded |
| [`<translation>`](#translation) | 0   | unbounded |
| [`<form>`](#form)               | 0   | unbounded |

---

### `<form>`

Forms describe terms related to a particular sense, such as inflections,
conjugations, plurals, comparatives, and superlatives.

#### Attributes

| Attribute | Type       | Required |
| --------- | ---------- | -------- |
| `term`    | entry term | Yes      |
| `kind`    | `string`   | No       |

#### Child elements

| Element         | Min | Max       |
| --------------- | --- | --------- |
| [`<tag>`](#tag) | 0   | unbounded |

ODict recognizes `conjugation`, `inflection`, `plural`, `singular`,
`comparative`, and `superlative` as standard `kind` values. Custom strings are
also accepted.

```xml
<sense pos="v">
  <form kind="conjugation" term="ran">
    <tag>past-tense</tag>
  </form>
  <form kind="conjugation" term="running">
    <tag>present-participle</tag>
  </form>
  <definition value="To move swiftly on foot" />
</sense>
```

---

### `<group>`

#### Attributes

| Attribute     | Type     | Required |
| ------------- | -------- | -------- |
| `id`          | `string` | No       |
| `description` | `string` | Yes      |

#### Child elements

| Element                       | Min | Max       |
| ----------------------------- | --- | --------- |
| [`<definition>`](#definition) | 0   | unbounded |

---

### `<definition>`

#### Attributes

| Attribute | Type     | Required |
| --------- | -------- | -------- |
| `id`      | `string` | No       |
| `value`   | `string` | Yes      |

#### Child elements

| Element                 | Min | Max       |
| ----------------------- | --- | --------- |
| [`<example>`](#example) | 0   | unbounded |
| [`<note>`](#note)       | 0   | unbounded |

---

### `<note>`

#### Attributes

| Attribute | Type     | Required |
| --------- | -------- | -------- |
| `id`      | `string` | No       |
| `value`   | `string` | Yes      |

#### Child elements

| Element                 | Min | Max       |
| ----------------------- | --- | --------- |
| [`<example>`](#example) | 0   | unbounded |

---

### `<example>`

#### Attributes

| Attribute | Type     | Required |
| --------- | -------- | -------- |
| `value`   | `string` | Yes      |

#### Child elements

| Element                             | Min | Max       |
| ----------------------------------- | --- | --------- |
| [`<translation>`](#translation)     | 0   | unbounded |
| [`<pronunciation>`](#pronunciation) | 0   | unbounded |

---

### `<translation>`

Translations can be attached directly to a sense or to an example.

#### Attributes

| Attribute | Type     | Required |
| --------- | -------- | -------- |
| `lang`    | `string` | Yes      |
| `value`   | `string` | Yes      |

Use a language code such as `en`, `fr`, or `zh-Hant` for `lang`.

---

### `<tag>`

A tag is a text-only child of `<sense>` or `<form>`. Tags are open-ended labels
for grammatical features, register, dialect, usage, or other source-specific
metadata.

```xml
<sense pos="n">
  <tag>informal</tag>
  <definition value="A person, especially a man" />
</sense>
```

---

### `<pronunciation>`

#### Attributes

| Attribute | Type     | Required |
| --------- | -------- | -------- |
| `kind`    | `string` | No       |
| `value`   | `string` | Yes      |

#### Child elements

| Element         | Min | Max       |
| --------------- | --- | --------- |
| [`<url>`](#url) | 0   | unbounded |

ODict recognizes `ipa`, `pinyin`, `hiragana`, `romaji`, `katakana`, `yale`,
`jyutping`, `bopomofo`, and `hepburn` as standard `kind` values. Custom strings
are also accepted.

---

### `<url>`

`<url>` attaches a media resource to a pronunciation.

#### Attributes

| Attribute     | Type     | Required |
| ------------- | -------- | -------- |
| `src`         | `string` | Yes      |
| `type`        | `string` | No       |
| `description` | `string` | No       |

---

### `<media>`

`<media>` attaches an image, audio file, video, or other resource directly to an
entry. It has the same attributes as [`<url>`](#url), but is an entry child
rather than a pronunciation child.

#### Attributes

| Attribute     | Type     | Required |
| ------------- | -------- | -------- |
| `src`         | `string` | Yes      |
| `type`        | `string` | No       |
| `description` | `string` | No       |

For both `<media>` and `<url>`, `src` accepts an `http://` or `https://` URL, or
a relative path beginning with `./` or `/`. The optional `type` value is a MIME
type.

```xml
<entry term="cat">
  <ety>
    <sense pos="n">
      <definition value="A small domesticated mammal" />
    </sense>
  </ety>
  <media
    src="./images/cat.jpg"
    type="image/jpeg"
    description="A domestic cat"
  />
</entry>
```

---

## Parts of speech

The `pos` attribute on `<sense>` accepts the following values. You can also pass any custom string, which will be treated as a custom part of speech.

### Universal

| Code       | Label                     |
| ---------- | ------------------------- |
| `art`      | article                   |
| `abv`      | abbreviation              |
| `adf`      | adfix                     |
| `adj`      | adjective                 |
| `phr_adj`  | adjective phrase          |
| `adv`      | adverb                    |
| `phr_adv`  | adverbial phrase          |
| `aff`      | affix                     |
| `aux`      | auxiliary                 |
| `aux_adj`  | auxiliary adjective       |
| `aux_v`    | auxiliary verb            |
| `chr`      | character                 |
| `cf`       | circumfix                 |
| `cls`      | classifier                |
| `conj`     | conjunction               |
| `conj_c`   | coordinating conjunction  |
| `contr`    | contraction               |
| `cop`      | copula                    |
| `ctr`      | counter                   |
| `det`      | determiner                |
| `expr`     | expression                |
| `inf`      | infix                     |
| `intf`     | interfix                  |
| `intj`     | interjection              |
| `vi`       | intransitive verb         |
| `name`     | name                      |
| `n`        | noun                      |
| `num`      | numeric                   |
| `part`     | particle                  |
| `phr`      | phrase                    |
| `postp`    | postposition              |
| `pref`     | prefix                    |
| `prep`     | preposition               |
| `phr_prep` | prepositional phrase      |
| `pron`     | pronoun                   |
| `propn`    | proper noun               |
| `prov`     | proverb                   |
| `punc`     | punctuation               |
| `conj_s`   | subordinating conjunction |
| `suff`     | suffix                    |
| `sym`      | symbol                    |
| `vt`       | transitive verb           |
| `un`       | unknown                   |
| `v`        | verb                      |

### Japanese-specific

| Code        | Label                                                                   |
| ----------- | ----------------------------------------------------------------------- |
| `adj_pn`    | pre-noun adjectival (rentaishi)                                         |
| `adj_kari`  | 'kari' adjective (archaic)                                              |
| `adj_ku`    | 'ku' adjective (archaic)                                                |
| `adj_nari`  | archaic/formal form of na-adjective                                     |
| `adj_na`    | adjectival nouns or quasi-adjectives (keiyodoshi)                       |
| `adj_shiku` | 'shiku' adjective (archaic)                                             |
| `adj_t`     | 'taru' adjective                                                        |
| `adj_ix`    | adjective (keiyoushi) - yoi/ii class                                    |
| `n_adv`     | adverbial noun (fukushitekimeishi)                                      |
| `adv_to`    | adverb taking the 'to' particle                                         |
| `adj_no`    | nouns which may take the genitive case particle 'no'                    |
| `n_pref`    | noun, used as a prefix                                                  |
| `n_suf`     | noun, used as a suffix                                                  |
| `nt`        | noun (temporal) (jisoumeishi)                                           |
| `adj_f`     | noun or verb acting prenominally                                        |
| `v5b`       | Godan verb with 'bu' ending                                             |
| `v5g`       | Godan verb with 'gu' ending                                             |
| `v5k`       | Godan verb with 'ku' ending                                             |
| `v5m`       | Godan verb with 'mu' ending                                             |
| `v5n`       | Godan verb with 'nu' ending                                             |
| `v5r`       | Godan verb with 'ru' ending                                             |
| `v5r_i`     | Godan verb with 'ru' ending (irregular verb)                            |
| `v5aru`     | Godan verb - -aru special class                                         |
| `v5k_s`     | Godan verb - Iku/Yuku special class                                     |
| `v5s`       | Godan verb with 'su' ending                                             |
| `v5t`       | Godan verb with 'tsu' ending                                            |
| `v5u`       | Godan verb with 'u' ending                                              |
| `v5uru`     | Godan verb - Uru old class verb (old form of Eru)                       |
| `v5u_s`     | Godan verb with 'u' ending (special class)                              |
| `v1`        | Ichidan verb                                                            |
| `v1s`       | Ichidan verb - kureru special class                                     |
| `vz`        | Ichidan verb - zuru verb (alternative form of -jiru verbs)              |
| `vk`        | Kuru verb - special class                                               |
| `v2b_s`     | Nidan verb (lower class) with 'bu' ending (archaic)                     |
| `v2b_k`     | Nidan verb (upper class) with 'bu' ending (archaic)                     |
| `v2d_s`     | Nidan verb (lower class) with 'dzu' ending (archaic)                    |
| `v2d_k`     | Nidan verb (upper class) with 'dzu' ending (archaic)                    |
| `v2g_s`     | Nidan verb (lower class) with 'gu' ending (archaic)                     |
| `v2g_k`     | Nidan verb (upper class) with 'gu' ending (archaic)                     |
| `v2h_s`     | Nidan verb (lower class) with 'hu/fu' ending (archaic)                  |
| `v2h_k`     | Nidan verb (upper class) with 'hu/fu' ending (archaic)                  |
| `v2k_s`     | Nidan verb (lower class) with 'ku' ending (archaic)                     |
| `v2k_k`     | Nidan verb (upper class) with 'ku' ending (archaic)                     |
| `v2m_s`     | Nidan verb (lower class) with 'mu' ending (archaic)                     |
| `v2m_k`     | Nidan verb (upper class) with 'mu' ending (archaic)                     |
| `v2n_s`     | Nidan verb (lower class) with 'nu' ending (archaic)                     |
| `v2r_s`     | Nidan verb (lower class) with 'ru' ending (archaic)                     |
| `v2r_k`     | Nidan verb (upper class) with 'ru' ending (archaic)                     |
| `v2s_s`     | Nidan verb (lower class) with 'su' ending (archaic)                     |
| `v2t_s`     | Nidan verb (lower class) with 'tsu' ending (archaic)                    |
| `v2t_k`     | Nidan verb (upper class) with 'tsu' ending (archaic)                    |
| `v2a_s`     | Nidan verb with 'u' ending (archaic)                                    |
| `v2w_s`     | Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic) |
| `v2y_s`     | Nidan verb (lower class) with 'yu' ending (archaic)                     |
| `v2y_k`     | Nidan verb (upper class) with 'yu' ending (archaic)                     |
| `v2z_s`     | Nidan verb (lower class) with 'zu' ending (archaic)                     |
| `vn`        | irregular nu verb                                                       |
| `vr`        | irregular ru verb, plain form ends with -ri                             |
| `vs_c`      | su verb - precursor to the modern suru                                  |
| `vs`        | noun or participle which takes the aux. verb suru                       |
| `vs_i`      | suru verb - included                                                    |
| `vs_s`      | suru verb - special class                                               |
| `v_unspec`  | verb unspecified                                                        |
| `v4b`       | Yodan verb with 'bu' ending (archaic)                                   |
| `v4g`       | Yodan verb with 'gu' ending (archaic)                                   |
| `v4h`       | Yodan verb with 'hu/fu' ending (archaic)                                |
| `v4k`       | Yodan verb with 'ku' ending (archaic)                                   |
| `v4m`       | Yodan verb with 'mu' ending (archaic)                                   |
| `v4n`       | Yodan verb with 'nu' ending (archaic)                                   |
| `v4r`       | Yodan verb with 'ru' ending (archaic)                                   |
| `v4s`       | Yodan verb with 'su' ending (archaic)                                   |
| `v4t`       | Yodan verb with 'tsu' ending (archaic)                                  |
