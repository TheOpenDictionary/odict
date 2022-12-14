const _ = require('lodash');

const stuff = `
adj-f	noun or verb acting prenominally	56
adj-i	adjective (keiyoushi)	1
adj-ix	adjective (keiyoushi) - yoi/ii class	7
adj-kari	'kari' adjective (archaic)	63
adj-ku	'ku' adjective (archaic)	64
adj-na	adjectival nouns or quasi-adjectives (keiyodoshi)	2
adj-nari	archaic/formal form of na-adjective	66
adj-no	nouns which may take the genitive case particle 'no'	3
adj-pn	pre-noun adjectival (rentaishi)	4
adj-shiku	'shiku' adjective (archaic)	65
adj-t	'taru' adjective	5
adv	adverb (fukushi)	6
adv-to	adverb taking the 'to' particle	8
aux	auxiliary	9
aux-adj	auxiliary adjective	10
aux-v	auxiliary verb	11
conj	conjunction	12
cop	copula	15
ctr	counter	51
exp	expressions (phrases, clauses, etc.)	13
int	interjection (kandoushi)	14
n	noun (common) (futsuumeishi)	17
n-adv	adverbial noun (fukushitekimeishi)	18
n-pr	proper noun	67
n-pref	noun, used as a prefix	20
n-suf	noun, used as a suffix	19
n-t	noun (temporal) (jisoumeishi)	21
num	numeric	24
pn	pronoun	61
pref	prefix	25
prt	particle	26
suf	suffix	27
unc	unclassified	98
v-unspec	verb unspecified	68
v1	Ichidan verb	28
v1-s	Ichidan verb - kureru special class	29
v2a-s	Nidan verb with 'u' ending (archaic)	59
v2b-k	Nidan verb (upper class) with 'bu' ending (archaic)	81
v2b-s	Nidan verb (lower class) with 'bu' ending (archaic)	93
v2d-k	Nidan verb (upper class) with 'dzu' ending (archaic)	79
v2d-s	Nidan verb (lower class) with 'dzu' ending (archaic)	90
v2g-k	Nidan verb (upper class) with 'gu' ending (archaic)	77
v2g-s	Nidan verb (lower class) with 'gu' ending (archaic)	86
v2h-k	Nidan verb (upper class) with 'hu/fu' ending (archaic)	80
v2h-s	Nidan verb (lower class) with 'hu/fu' ending (archaic)	92
v2k-k	Nidan verb (upper class) with 'ku' ending (archaic)	76
v2k-s	Nidan verb (lower class) with 'ku' ending (archaic)	85
v2m-k	Nidan verb (upper class) with 'mu' ending (archaic)	82
v2m-s	Nidan verb (lower class) with 'mu' ending (archaic)	94
v2n-s	Nidan verb (lower class) with 'nu' ending (archaic)	91
v2r-k	Nidan verb (upper class) with 'ru' ending (archaic)	84
v2r-s	Nidan verb (lower class) with 'ru' ending (archaic)	96
v2s-s	Nidan verb (lower class) with 'su' ending (archaic)	87
v2t-k	Nidan verb (upper class) with 'tsu' ending (archaic)	78
v2t-s	Nidan verb (lower class) with 'tsu' ending (archaic)	89
v2w-s	Nidan verb (lower class) with 'u' ending and 'we' conjugation (archaic)	97
v2y-k	Nidan verb (upper class) with 'yu' ending (archaic)	83
v2y-s	Nidan verb (lower class) with 'yu' ending (archaic)	95
v2z-s	Nidan verb (lower class) with 'zu' ending (archaic)	88
v4b	Yodan verb with 'bu' ending (archaic)	74
v4g	Yodan verb with 'gu' ending (archaic)	70
v4h	Yodan verb with 'hu/fu' ending (archaic)	60
v4k	Yodan verb with 'ku' ending (archaic)	69
v4m	Yodan verb with 'mu' ending (archaic)	75
v4n	Yodan verb with 'nu' ending (archaic)	73
v4r	Yodan verb with 'ru' ending (archaic)	53
v4s	Yodan verb with 'su' ending (archaic)	71
v4t	Yodan verb with 'tsu' ending (archaic)	72
v5aru	Godan verb - -aru special class	30
v5b	Godan verb with 'bu' ending	31
v5g	Godan verb with 'gu' ending	32
v5k	Godan verb with 'ku' ending	33
v5k-s	Godan verb - Iku/Yuku special class	34
v5m	Godan verb with 'mu' ending	35
v5n	Godan verb with 'nu' ending	36
v5r	Godan verb with 'ru' ending	37
v5r-i	Godan verb with 'ru' ending (irregular verb)	38
v5s	Godan verb with 'su' ending	39
v5t	Godan verb with 'tsu' ending	40
v5u	Godan verb with 'u' ending	41
v5u-s	Godan verb with 'u' ending (special class)	42
v5uru	Godan verb - Uru old class verb (old form of Eru)	43
vi	intransitive verb	44
vk	Kuru verb - special class	45
vn	irregular nu verb	52
vr	irregular ru verb, plain form ends with -ri	58
vs	noun or participle which takes the aux. verb suru	46
vs-c	su verb - precursor to the modern suru	62
vs-i	suru verb - included	48
vs-s	suru verb - special class	47
vt	transitive verb	50
vz	Ichidan verb - zuru verb (alternative form of -jiru verbs)
`;

let output = ""

stuff.split('\n').forEach(line => {
  const [key, value] = line.split('\t');

  if (value  ) {

    output += `${_.upperFirst(_.camelCase(value.replace(/(archaic|with|jisoumeishi|fukushitekimeishi|common|kandoushi|fukushi|keiyoushi)/g, '')))} PartOfSpeech = "${key}"\n`;
  }
  
})

console.log(output)