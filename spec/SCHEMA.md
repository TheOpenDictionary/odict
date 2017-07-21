ODict XML Schema
================
The `.odict` file extension represents an odict XML file (referred to as a 'source file') that has been compiled
with the official ODict CLI. These XML files follow the standard XML format, however,
adhere to a very strict schema to ensure maximum clarity when defining dictionaries.
This schema is outlined below.

<a name="dictionary"></a> \<dictionary\>
----------------------------------------
Dictionary nodes occur at the base of all source files and will not compile without one.
ODict looks for these nodes by default when compiling. They take no attributes.

#### Children ####
- [\<entry\>](#entry)

<a name="entry"></a> \<entry\>
--------------------------------
Entries are the primary entry point to the dictionary and represent __unique terms__. They
are used as lookup keys internally by ODict, so it is important there are __no duplicate entries__.
There is a __required__ `term` attribute that __must__ be attached to every entry node
that represents the word being defined.

#### Attributes ####
- term

#### Children ####
- [\<ety\>](#etymology)


<a name="etymology"></a> \<ety\>
--------------------------------
Etymologies, shortened to 'ety' for simplicity, are the linguistic roots of a given word.
Wiktionary often separates word definitions by their etymologies, however, even single definitions
can be related to an etymology. Therefore, it is required each `<entry>` node contain at least
one etymology. Etymologies have an optional `description` attribute, to describe in further detail the
origins or root of a word.

#### Attributes ####
- description

#### Children ####
- [\<usage\>](#usage)

<a name="usage"></a> \<usage\>
------------------------------
Usages represent a group of definitions who share a single common trait, notable their part of speech. 

#### Attributes ###
- [pos](#parts-of-speech)

#### Children ####
- [\<group\>](#group)
- [\<definition\>](#definition)


<a name="group"></a> \<group\>
------------------------------
Usage groups are groups of definitions. They differ from `<usage>` tags as they all belong to the same
part of speech, but differ based on their meaning. For example, the verbal phrase to "pass out" might 
have the definitions:
 
 - to distribute
 - to hand out, especially to a large group of people
 
yet can also mean:

 - to faint
 - to lose consciousness 

It is important to distinguish between these verb definitions so the definition list does not become muddled.
While groups are not required, they are encouraged, especially for lengthy usage nodes.

#### Attributes ###
- description

#### Children ####
- [\<usage\>](#usage)

<a name="definition"></a> \<definition\>
----------------------------------------
Definitions are attribute-less, child-less nodes whose sole purpose is to contain a text definition, either
group by a `<group>` node or by usage. While it is possible to use HTML in definition nodes, using vanilla HTML will
cause the parser to treat all HTML tags as unrecognized XML children and ignore them when producing the output. Therefore,
you must encode all HTML entities in your XML __before__ compiling the file. Afterwards, you can decode these entities from
the CLI output using JavaScript or your favorite coding language.

<a name="parts-of-speech"></a> Parts of Speech
==============================================
Currently, the allowed parts of speech values are as follows:

- `verb`
- `noun`
- `adj`
- `pronoun`
- `adv` (adverb)
- `prep` (preposition)
- `conj` (conjunction)
- `intj` (interjection)
- `prefix`
- `suffix`
- `particle`