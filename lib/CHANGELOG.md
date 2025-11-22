# Changelog

## [3.2.0](https://github.com/TheOpenDictionary/odict/compare/lib/v3.1.1...lib/v3.2.0) (2025-11-22)


### Features

* **http:** Expose download methods on OpenDictionary ([99cd786](https://github.com/TheOpenDictionary/odict/commit/99cd786bd2aba9321670c890fe32f923165b3cbc))
* Preserve ordering of sense data ([#1339](https://github.com/TheOpenDictionary/odict/issues/1339)) ([777b8e9](https://github.com/TheOpenDictionary/odict/commit/777b8e96fbb35c0cdfab6f63fcd8f522c6c1cb3f))


### Bug Fixes

* Update dependencies ([6029b8d](https://github.com/TheOpenDictionary/odict/commit/6029b8d16d314747c180b19ce5385ce4ce78e128))

## [3.1.1](https://github.com/TheOpenDictionary/odict/compare/lib/v3.1.0...lib/v3.1.1) (2025-10-22)


### Bug Fixes

* Allow configuring config directory through LoadOptions ([#1323](https://github.com/TheOpenDictionary/odict/issues/1323)) ([3808745](https://github.com/TheOpenDictionary/odict/commit/3808745641f75018f71c29fdfcd8eb32ab1406f0))

## [3.1.0](https://github.com/TheOpenDictionary/odict/compare/lib/v3.0.0...lib/v3.1.0) (2025-10-20)


### Features

* Allow passing remote load options to Node and Python ([#1315](https://github.com/TheOpenDictionary/odict/issues/1315)) ([80a8361](https://github.com/TheOpenDictionary/odict/commit/80a8361e1cff88bffb5bf36cb354ca04ad89a343))
* Remove backwards compatibility for legacy follow for Node and Python ([#1308](https://github.com/TheOpenDictionary/odict/issues/1308)) ([785fac5](https://github.com/TheOpenDictionary/odict/commit/785fac54001df45841fa18d0c6822e36a337b5ec))
* Remove backwards compatibility for legacy follow for Node and Python ([#1310](https://github.com/TheOpenDictionary/odict/issues/1310)) ([2efe45d](https://github.com/TheOpenDictionary/odict/commit/2efe45d44babc1357bd10650360b4edd671513dd))
* String interning ([#1313](https://github.com/TheOpenDictionary/odict/issues/1313)) ([a1ce402](https://github.com/TheOpenDictionary/odict/commit/a1ce4025950f840674e1e4e159b60311672febc2))


### Bug Fixes

* Inline interning library ([5833965](https://github.com/TheOpenDictionary/odict/commit/583396596017cc2a979d45de3237885671c136e7))

## [3.0.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.9.1...lib/v3.0.0) (2025-10-17)


### ⚠ BREAKING CHANGES

* support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282))

### Features

* **lookup:** Update follow logic ([#1307](https://github.com/TheOpenDictionary/odict/issues/1307)) ([0aa9a53](https://github.com/TheOpenDictionary/odict/commit/0aa9a532def5b1f544f75f2e7ad3540e3f90cf55))
* Support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282)) ([326b325](https://github.com/TheOpenDictionary/odict/commit/326b325efd4a1ea1327ae2e36f55fe6c13663ca1))
* Use safe API for rkyv access ([dd3fd60](https://github.com/TheOpenDictionary/odict/commit/dd3fd60d1795538108ed0cb02281ccc792eec4f1))
* Use u8 repr for enums ([b8bca1d](https://github.com/TheOpenDictionary/odict/commit/b8bca1d8bb94a6cfa552e30e91bd3d1a02ebd9e9))
* Use uv for python management ([9b1971c](https://github.com/TheOpenDictionary/odict/commit/9b1971c645310103364fb96e1efb3c01a64ba536))

## [2.9.1](https://github.com/TheOpenDictionary/odict/compare/lib/v2.9.0...lib/v2.9.1) (2025-08-16)


### Bug Fixes

* Correct semver partial order implementation ([#1255](https://github.com/TheOpenDictionary/odict/issues/1255)) ([627e3ad](https://github.com/TheOpenDictionary/odict/commit/627e3ad45cf57e5e5e048bd1d727cef81c3e023a))


### Performance Improvements

* Decrease compress quality to 8 ([#1269](https://github.com/TheOpenDictionary/odict/issues/1269)) ([ccc5040](https://github.com/TheOpenDictionary/odict/commit/ccc504063b0289edb2410939141aa992c3adadd4)), closes [#1249](https://github.com/TheOpenDictionary/odict/issues/1249)

## [2.9.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.8.0...lib/v2.9.0) (2025-07-12)


### Features

* Add `min_rank` and `max_rank` methods for retrieving ranking bounds ([#1245](https://github.com/TheOpenDictionary/odict/issues/1245)) ([3cff5ee](https://github.com/TheOpenDictionary/odict/commit/3cff5ee302b5a2281c3ee8fdb75b1ee553760b94))
* Allow custom `follow` limit by changing from boolean to number ([#1246](https://github.com/TheOpenDictionary/odict/issues/1246)) ([d96187a](https://github.com/TheOpenDictionary/odict/commit/d96187a541220f2e934462a31af363f64786d623))
* Make mod semver public & implement partial order for SemanticVersion ([#1225](https://github.com/TheOpenDictionary/odict/issues/1225)) ([15b860a](https://github.com/TheOpenDictionary/odict/commit/15b860a06cfd638fb7e4e896cdc9760506322de6))


### Bug Fixes

* **deps:** Update rust crate quick-xml to 0.38.0 ([#1242](https://github.com/TheOpenDictionary/odict/issues/1242)) ([b55a728](https://github.com/TheOpenDictionary/odict/commit/b55a72826b94357a6d4781aae1cd75d97a28f7ee))

## [2.8.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.7.0...lib/v2.8.0) (2025-05-14)


### Features

* Add Niching policies to reduce disk space ([#1218](https://github.com/TheOpenDictionary/odict/issues/1218)) ([c33ac83](https://github.com/TheOpenDictionary/odict/commit/c33ac83972158813f76b5039e2aab55251dd83b8))
* Add Other(String) to PartOfSpeech enum to handle custom strings ([#1203](https://github.com/TheOpenDictionary/odict/issues/1203)) ([b7f258a](https://github.com/TheOpenDictionary/odict/commit/b7f258aa35a8bb29ccc6f369ae3316435b5f6c03))
* Replace HashMap with HashSet ([#1217](https://github.com/TheOpenDictionary/odict/issues/1217)) ([fdb5e11](https://github.com/TheOpenDictionary/odict/commit/fdb5e111ea84b179156486eacf4b78d843c12efb))


### Bug Fixes

* **deps:** Update rust crate tantivy to 0.24.0 ([#1176](https://github.com/TheOpenDictionary/odict/issues/1176)) ([111887f](https://github.com/TheOpenDictionary/odict/commit/111887f085d00a988f6227a6fd810a583500726b))

## [2.7.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.6.0...lib/v2.7.0) (2025-05-10)


### Features

* Add `rank` attribute to Entry for storing word frequencies ([#1210](https://github.com/TheOpenDictionary/odict/issues/1210)) ([2b2439a](https://github.com/TheOpenDictionary/odict/commit/2b2439a4dcb82d2b2c247174eb22d4a90d2037d5))

## [2.6.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.5.0...lib/v2.6.0) (2025-04-24)


### Features

* Add &lt;pronunciation&gt; tag ([#1197](https://github.com/TheOpenDictionary/odict/issues/1197)) ([026e2f2](https://github.com/TheOpenDictionary/odict/commit/026e2f292bf02c182684f8656b9eaa13533f1a72))
* Add `tags` fields to `sense` and `form`, as well as move `lemma` to `<sense>` ([#1192](https://github.com/TheOpenDictionary/odict/issues/1192)) ([f03d8c1](https://github.com/TheOpenDictionary/odict/commit/f03d8c122f96ec20f85ccff639962c9ffa44aee5))
* Add media URLs to `<entry>` nodes ([#1202](https://github.com/TheOpenDictionary/odict/issues/1202)) ([04c2307](https://github.com/TheOpenDictionary/odict/commit/04c2307b005ff57bbe460f7b1034f97c811b580f))
* Add translation node ([#1196](https://github.com/TheOpenDictionary/odict/issues/1196)) ([ef15aba](https://github.com/TheOpenDictionary/odict/commit/ef15abaf0749cf014315fb57ec63c50d2ae59e98))
* Add wrapper nodes to `example` and `note` types ([#1194](https://github.com/TheOpenDictionary/odict/issues/1194)) ([bde127c](https://github.com/TheOpenDictionary/odict/commit/bde127cab6a9d805de23c0ff695c504c510cc89a))
* Move `translations` and `forms` to `sense` ([#1200](https://github.com/TheOpenDictionary/odict/issues/1200)) ([5c0e746](https://github.com/TheOpenDictionary/odict/commit/5c0e7466df84f5435cc53eba7fcc72813c11d28c))
* Remove wrapper components ([0908f01](https://github.com/TheOpenDictionary/odict/commit/0908f0128c1dd1b0749b756d757d8f3aa50e6c1c))
* Use `structural_convert` macro instead of custom From implementations ([#1199](https://github.com/TheOpenDictionary/odict/issues/1199)) ([392d624](https://github.com/TheOpenDictionary/odict/commit/392d624a4b956f0bc22d0529b4ccb0307807cdfd))


### Bug Fixes

* **deps:** Update rust crate brotli to v8 ([#1189](https://github.com/TheOpenDictionary/odict/issues/1189)) ([a8d61e6](https://github.com/TheOpenDictionary/odict/commit/a8d61e697ad17fc7a69796a7ee0c27f5ee285944))

## [2.5.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.4.0...lib/v2.5.0) (2025-04-20)


### Features

* Add support for word forms and lemmas ([#1186](https://github.com/TheOpenDictionary/odict/issues/1186)) ([9e37a28](https://github.com/TheOpenDictionary/odict/commit/9e37a2834fda82bfaf558aeab9cc74fbced5a1d4))

## [2.4.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.3.0...lib/v2.4.0) (2025-04-13)


### Features

* Add flag to support case insensitivity for lookups and tokenization ([#1179](https://github.com/TheOpenDictionary/odict/issues/1179)) ([a7e7baa](https://github.com/TheOpenDictionary/odict/commit/a7e7baac0f8d02e565a2d01acdc59c9bd1bc3242))

## [2.3.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.2.0...lib/v2.3.0) (2025-04-04)


### Features

* **node:** Add support for TokenizeOptions ([e14fb17](https://github.com/TheOpenDictionary/odict/commit/e14fb17abcaa2f07bfabb482db11402cd2b41fbf))
* **tokenize:** Add additional metadata to Token model ([bd44701](https://github.com/TheOpenDictionary/odict/commit/bd44701bb3ef59fafac31a2b6582c729fd881f1e))


### Bug Fixes

* Fix tests ([a87323b](https://github.com/TheOpenDictionary/odict/commit/a87323b6ace6cfc4daafa2c3738da22a3e1ec796))
* **lookup:** Don't follow empty strings and disable follow by default ([73ddd89](https://github.com/TheOpenDictionary/odict/commit/73ddd89798cef3b98a670d8c9c5ad8c7816f0d83))

## [2.2.0](https://github.com/TheOpenDictionary/odict/compare/lib/v2.1.0...lib/v2.2.0) (2025-04-03)


### Features

* **serve:** Cache dictionaries ([#1170](https://github.com/TheOpenDictionary/odict/issues/1170)) ([907918b](https://github.com/TheOpenDictionary/odict/commit/907918ba12b34b44399bfa185a0dd5f8c8575ad0))

## [2.1.0](https://github.com/TheOpenDictionary/odict/compare/lib-v2.0.0...lib/v2.1.0) (2025-04-03)


### ⚠ BREAKING CHANGES

* **lib:** change compression from lz4 to brotli ([#1064](https://github.com/TheOpenDictionary/odict/issues/1064))

### Features

* Add option to print entries as Markdown/HTML ([#1068](https://github.com/TheOpenDictionary/odict/issues/1068)) ([3422533](https://github.com/TheOpenDictionary/odict/commit/3422533514264dbe80e6ff4c6ac4e3c12f289ee8))
* Add WASM support to Node package ([#1155](https://github.com/TheOpenDictionary/odict/issues/1155)) ([3456969](https://github.com/TheOpenDictionary/odict/commit/3456969422df2530693c196bafefa7cd92fb2f12))
* **core:** Add new tokenization feature ([#1159](https://github.com/TheOpenDictionary/odict/issues/1159)) ([d9196c1](https://github.com/TheOpenDictionary/odict/commit/d9196c1aae4c275d3c326d5803f7baf65f7b5a89))
* Expose Brotli compression options for `compile` ([#1067](https://github.com/TheOpenDictionary/odict/issues/1067)) ([0b282cd](https://github.com/TheOpenDictionary/odict/commit/0b282cde171ee3e6b1252c59fa9fc8f050e7c4b4))
* **lib:** Change compression from lz4 to brotli ([#1064](https://github.com/TheOpenDictionary/odict/issues/1064)) ([6dc0b9a](https://github.com/TheOpenDictionary/odict/commit/6dc0b9a05d9d7dce4aed1a29ba3b0f54532748af))


### Bug Fixes

* **deps:** Update rust crate dirs to v6 ([#1082](https://github.com/TheOpenDictionary/odict/issues/1082)) ([2b0c37d](https://github.com/TheOpenDictionary/odict/commit/2b0c37daf35a06211be5e2301a315c9262ba9a79))
* **deps:** Update rust crate pulldown-cmark to 0.13.0 ([#1109](https://github.com/TheOpenDictionary/odict/issues/1109)) ([80bb314](https://github.com/TheOpenDictionary/odict/commit/80bb314e1fdedb11d2fe59b1ccf446628c5a1dd9))
* **error:** Fix build failure when all default features are disabled ([215bd7b](https://github.com/TheOpenDictionary/odict/commit/215bd7becd62422bae64398b188f905bc87dd52d))

## [2.0.0](https://github.com/TheOpenDictionary/odict/compare/lib-v2.0.0...lib-v2.0.0) (2024-12-25)


### Features

* Add indexing ([#656](https://github.com/TheOpenDictionary/odict/issues/656)) ([a94db99](https://github.com/TheOpenDictionary/odict/commit/a94db9953c34df96bedff5c3ebde989a64d27ace))
* Add publish command and update Cargo.toml ([60af67d](https://github.com/TheOpenDictionary/odict/commit/60af67de1d8bd15046b3eb4a44ac35c86268a126))
* Add stable Python binding 🎉 ([#1049](https://github.com/TheOpenDictionary/odict/issues/1049)) ([73c6e33](https://github.com/TheOpenDictionary/odict/commit/73c6e339b8614c6eb048de4ee7586dd5aa98803e))
* **config:** Port `alias` logic and CLI command to Rust ([#641](https://github.com/TheOpenDictionary/odict/issues/641)) ([853bf43](https://github.com/TheOpenDictionary/odict/commit/853bf435ecf6808a8f7d0daa724802de9dac43f1))
* Deployment changes ([#1002](https://github.com/TheOpenDictionary/odict/issues/1002)) ([981c223](https://github.com/TheOpenDictionary/odict/commit/981c2232fe8908cb9a0afd95f6c04e32a4c698ed))
* Exclude files from crate ([f5992eb](https://github.com/TheOpenDictionary/odict/commit/f5992eb429370886cb7988b69546bcad48d59c20))
* Finalize search ([#693](https://github.com/TheOpenDictionary/odict/issues/693)) ([0e17c88](https://github.com/TheOpenDictionary/odict/commit/0e17c88142befd6c221a0008f30688a05151b865))
* Fix NAPI CI ([#1015](https://github.com/TheOpenDictionary/odict/issues/1015)) ([c688d43](https://github.com/TheOpenDictionary/odict/commit/c688d43ecb1059182ab53b2ab9042148f9dbf981))
* Reduce unnecessary Lazy declarations and remove once_cell ([#1016](https://github.com/TheOpenDictionary/odict/issues/1016)) ([6bcea66](https://github.com/TheOpenDictionary/odict/commit/6bcea668331fd191e967a1a1dabbd4dc9eeeb885))
* Replace just runners with mise tasks ([0275910](https://github.com/TheOpenDictionary/odict/commit/0275910feff1f100a464d5d95a92ebfef95d4e6f))
* Upgrade rkyv to 0.8.8 ([dbb16d9](https://github.com/TheOpenDictionary/odict/commit/dbb16d97f632076fa72d5976a46dbdd5236545e6))
* **v2:** Add `new` command ([#700](https://github.com/TheOpenDictionary/odict/issues/700)) ([6f65dc3](https://github.com/TheOpenDictionary/odict/commit/6f65dc371ae4b51600673b853353406ecaf92cb3))
* **v2:** Add info command ([#800](https://github.com/TheOpenDictionary/odict/issues/800)) ([8b73cc4](https://github.com/TheOpenDictionary/odict/commit/8b73cc4e687708abc90848740b827986391a2175))
* **v2:** Add pretty printing ([#701](https://github.com/TheOpenDictionary/odict/issues/701)) ([e24160f](https://github.com/TheOpenDictionary/odict/commit/e24160f4023b1be97b0d8cb98e03b82cecdedd8e))
* **v2:** Add Rust Node bindings ([#760](https://github.com/TheOpenDictionary/odict/issues/760)) ([aac5501](https://github.com/TheOpenDictionary/odict/commit/aac550181f6d144649ce9ad0ff823967b29668bf))
* **v2:** Add serve command ([#748](https://github.com/TheOpenDictionary/odict/issues/748)) ([ff10753](https://github.com/TheOpenDictionary/odict/commit/ff107533fcb25094230770b8c51697348caa6fc2))
* **v2:** Add SQL dumping ([#747](https://github.com/TheOpenDictionary/odict/issues/747)) ([caceb88](https://github.com/TheOpenDictionary/odict/commit/caceb883e527358a0f0e74221130af572c0f561a))
* **v2:** Implement basic dumping ([#654](https://github.com/TheOpenDictionary/odict/issues/654)) ([5e29764](https://github.com/TheOpenDictionary/odict/commit/5e29764048767752c56178df5e1ac1e9160894d0))
* **v2:** Make tokenizer customizable ([975e4b0](https://github.com/TheOpenDictionary/odict/commit/975e4b0881876b31e7a7d97c01f8178668867deb))


### Bug Fixes

* **ci:** Fix tests ([ce9c477](https://github.com/TheOpenDictionary/odict/commit/ce9c477432a0ffee7d6d32a938827bae10648da8))
* **ci:** Remove charabia from Node package ([cf02d0c](https://github.com/TheOpenDictionary/odict/commit/cf02d0c6f7a6b9015c88e563e7d24a846428b145))
* **deps:** Update module github.com/blevesearch/bleve_index_api to v1.1.6 ([17158be](https://github.com/TheOpenDictionary/odict/commit/17158bec25d2b5dbb2cfce611a3a41acbc8a477f))
* **deps:** Update module github.com/blevesearch/bleve/v2 to v2.4.0 ([911283c](https://github.com/TheOpenDictionary/odict/commit/911283cae2ced80f4438075dec618bf06c3b3483))
* **deps:** Update module github.com/bokwoon95/sqddl to v0.4.10 ([5afadcc](https://github.com/TheOpenDictionary/odict/commit/5afadccb084e096b2e8ef2035d9e273f78b5dacb))
* **deps:** Update module github.com/bokwoon95/sqddl to v0.4.11 ([649f4fc](https://github.com/TheOpenDictionary/odict/commit/649f4fc37ff3051b5d04242e94a7b45a21bc75cc))
* **deps:** Update module github.com/gkampitakis/go-snaps to v0.5.1 ([e0c776c](https://github.com/TheOpenDictionary/odict/commit/e0c776c1b0842314abdfc415141093c13fb50096))
* **deps:** Update module github.com/gkampitakis/go-snaps to v0.5.2 ([e25509a](https://github.com/TheOpenDictionary/odict/commit/e25509ae396ad6dcf86debe44a27fd2962d5339c))
* **deps:** Update module github.com/google/flatbuffers to v24 ([aa73d33](https://github.com/TheOpenDictionary/odict/commit/aa73d33d6685f6b15d4223943967c748d1bae8bd))
* **deps:** Update module github.com/google/flatbuffers to v24.3.7+incompatible ([2e9a949](https://github.com/TheOpenDictionary/odict/commit/2e9a949bc475bd11e294717b7e81ed4c48023138))
* **deps:** Update module github.com/schollz/progressbar/v3 to v3.14.2 ([44b3987](https://github.com/TheOpenDictionary/odict/commit/44b3987fdb45cc1dfbe4a69a60597e256fb7accd))
* **deps:** Update module github.com/stretchr/testify to v1.9.0 ([6a2232d](https://github.com/TheOpenDictionary/odict/commit/6a2232d2b060dcdc7d426d1bd4b1bc3c76ef16d9))
* **deps:** Update rust crate charabia to 0.8.10 ([848b2fc](https://github.com/TheOpenDictionary/odict/commit/848b2fc3076a249a19b9e561b336a6c23ab86846))
* **deps:** Update rust crate charabia to 0.8.8 ([ceb4b25](https://github.com/TheOpenDictionary/odict/commit/ceb4b25e4c9ff65a97d3f964ad1f6091309e9a19))
* **deps:** Update rust crate charabia to 0.8.9 ([eee9a23](https://github.com/TheOpenDictionary/odict/commit/eee9a232a5e0167b4cdfff1b4ecf6b36af04a1ce))
* **deps:** Update rust crate charabia to 0.9.0 ([e3c1c86](https://github.com/TheOpenDictionary/odict/commit/e3c1c86b9859c66a4f003acfeba168925bfec9b4))
* **deps:** Update rust crate lz4_flex to 0.11.3 ([c2be1b2](https://github.com/TheOpenDictionary/odict/commit/c2be1b2ef022895b63ce706c8147a9ae6957762c))
* **deps:** Update rust crate pulldown-cmark to 0.10.2 ([6303c50](https://github.com/TheOpenDictionary/odict/commit/6303c50f9fb4b1de0e0c7717bfe49fdb255de0af))
* **deps:** Update rust crate pulldown-cmark to 0.10.3 ([0311794](https://github.com/TheOpenDictionary/odict/commit/031179459880fc9a4a1e9ce146205ffb9b744ada))
* **deps:** Update rust crate pulldown-cmark to 0.11.0 ([d4e85f5](https://github.com/TheOpenDictionary/odict/commit/d4e85f54b2c973721ff99fb5c091489223927b35))
* **deps:** Update rust crate pulldown-cmark to 0.12.0 ([0885bf8](https://github.com/TheOpenDictionary/odict/commit/0885bf87c345103af6cb6138d220cba661b0f9ff))
* **deps:** Update rust crate quick-xml to 0.32.0 ([e43be29](https://github.com/TheOpenDictionary/odict/commit/e43be29f45db751e41bfa7f14f534f7091e4288f))
* **deps:** Update rust crate quick-xml to 0.33.0 ([8b1209f](https://github.com/TheOpenDictionary/odict/commit/8b1209f70e2bce817f1e274b4ff78b013a1b1fd1))
* **deps:** Update rust crate quick-xml to 0.34.0 ([7fbb066](https://github.com/TheOpenDictionary/odict/commit/7fbb0664f04fc38436a8d2c5f7269aedba183314))
* **deps:** Update rust crate quick-xml to 0.35.0 ([b29ed90](https://github.com/TheOpenDictionary/odict/commit/b29ed9056334014160ccdbd2448f25aeae91f38a))
* **deps:** Update rust crate quick-xml to 0.36.0 ([a1458d9](https://github.com/TheOpenDictionary/odict/commit/a1458d944430dccaf830bc6c7574709cf776b69d))
* **deps:** Update rust crate quick-xml to 0.37.0 ([c221e8e](https://github.com/TheOpenDictionary/odict/commit/c221e8ea829db0c693fd537bcb169fbf379224d7))
* **deps:** Update rust crate rayon to 1.10.0 ([1ef7881](https://github.com/TheOpenDictionary/odict/commit/1ef788104275cee64d9ce7a3358689f47c45725d))
* **deps:** Update rust crate regex to 1.10.4 ([4e021f2](https://github.com/TheOpenDictionary/odict/commit/4e021f20d5b7b33c14c5481e5f3615c62b061b33))
* **deps:** Update rust crate rkyv to 0.7.44 ([8d27070](https://github.com/TheOpenDictionary/odict/commit/8d27070c8d2af3d548c0fe227d4ba38d65ee99d8))
* **deps:** Update rust crate sea-query to 0.31.0 ([769b0fb](https://github.com/TheOpenDictionary/odict/commit/769b0fbe629d5d56186e79bb6d310b48fc4a928c))
* **deps:** Update rust crate sea-query to 0.32.0 ([fa18174](https://github.com/TheOpenDictionary/odict/commit/fa18174e3582b498bcdf7dd3c731cd6766e34d99))
* **deps:** Update rust crate serde to 1.0.197 ([0bad370](https://github.com/TheOpenDictionary/odict/commit/0bad37035c44e35576ea78816d8016934913c126))
* **deps:** Update rust crate serde to 1.0.198 ([2701027](https://github.com/TheOpenDictionary/odict/commit/2701027fe3ce7c7847f3e92a59b0b6092e73d941))
* **deps:** Update rust crate serde to 1.0.199 ([cca009c](https://github.com/TheOpenDictionary/odict/commit/cca009cf62b3a8a92f48c5172d222a4b3844a93a))
* **deps:** Update rust crate serde to 1.0.200 ([a192028](https://github.com/TheOpenDictionary/odict/commit/a19202869063703dc23e36ca9adbab5f04063d4e))
* **deps:** Update rust crate serde_json to 1.0.115 ([e578bc8](https://github.com/TheOpenDictionary/odict/commit/e578bc8abf479d884a0764075290494bfb39af8d))
* **deps:** Update rust crate serde_json to 1.0.116 ([3f4c11d](https://github.com/TheOpenDictionary/odict/commit/3f4c11dc1f771ed0c6bfb4f86d12a177da13347a))
* **deps:** Update rust crate uuid to 1.8.0 ([46b482f](https://github.com/TheOpenDictionary/odict/commit/46b482f29dd7a2287e0cf63e5f772d5a7ae2dba6))
