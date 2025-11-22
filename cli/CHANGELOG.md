# Changelog

## [3.2.0](https://github.com/TheOpenDictionary/odict/compare/cli/v3.1.1...cli/v3.2.0) (2025-11-22)


### Features

* Preserve ordering of sense data ([#1339](https://github.com/TheOpenDictionary/odict/issues/1339)) ([777b8e9](https://github.com/TheOpenDictionary/odict/commit/777b8e96fbb35c0cdfab6f63fcd8f522c6c1cb3f))


### Bug Fixes

* Update dependencies ([6029b8d](https://github.com/TheOpenDictionary/odict/commit/6029b8d16d314747c180b19ce5385ce4ce78e128))

## [3.1.1](https://github.com/TheOpenDictionary/odict/compare/cli/v3.1.0...cli/v3.1.1) (2025-10-22)


### Bug Fixes

* Allow configuring config directory through LoadOptions ([#1323](https://github.com/TheOpenDictionary/odict/issues/1323)) ([3808745](https://github.com/TheOpenDictionary/odict/commit/3808745641f75018f71c29fdfcd8eb32ab1406f0))

## [3.1.0](https://github.com/TheOpenDictionary/odict/compare/cli/v3.0.0...cli/v3.1.0) (2025-10-20)


### Miscellaneous Chores

* **cli:** Synchronize odict versions

## [3.0.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.9.1...cli/v3.0.0) (2025-10-17)


### ⚠ BREAKING CHANGES

* support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282))

### Features

* **lookup:** Update follow logic ([#1307](https://github.com/TheOpenDictionary/odict/issues/1307)) ([0aa9a53](https://github.com/TheOpenDictionary/odict/commit/0aa9a532def5b1f544f75f2e7ad3540e3f90cf55))
* Support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282)) ([326b325](https://github.com/TheOpenDictionary/odict/commit/326b325efd4a1ea1327ae2e36f55fe6c13663ca1))
* Use safe API for rkyv access ([dd3fd60](https://github.com/TheOpenDictionary/odict/commit/dd3fd60d1795538108ed0cb02281ccc792eec4f1))
* Use u8 repr for enums ([b8bca1d](https://github.com/TheOpenDictionary/odict/commit/b8bca1d8bb94a6cfa552e30e91bd3d1a02ebd9e9))
* Use uv for python management ([9b1971c](https://github.com/TheOpenDictionary/odict/commit/9b1971c645310103364fb96e1efb3c01a64ba536))

## [2.9.1](https://github.com/TheOpenDictionary/odict/compare/cli/v2.9.0...cli/v2.9.1) (2025-08-16)


### Miscellaneous Chores

* **cli:** Synchronize odict versions

## [2.9.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.8.0...cli/v2.9.0) (2025-07-12)


### Features

* Add `min_rank` and `max_rank` methods for retrieving ranking bounds ([#1245](https://github.com/TheOpenDictionary/odict/issues/1245)) ([3cff5ee](https://github.com/TheOpenDictionary/odict/commit/3cff5ee302b5a2281c3ee8fdb75b1ee553760b94))
* Allow custom `follow` limit by changing from boolean to number ([#1246](https://github.com/TheOpenDictionary/odict/issues/1246)) ([d96187a](https://github.com/TheOpenDictionary/odict/commit/d96187a541220f2e934462a31af363f64786d623))


### Bug Fixes

* **deps:** Update rust crate lru to 0.16.0 ([#1241](https://github.com/TheOpenDictionary/odict/issues/1241)) ([ba9d9a9](https://github.com/TheOpenDictionary/odict/commit/ba9d9a97cef809a3445e4ba89b90024e4520bd44))

## [2.8.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.7.0...cli/v2.8.0) (2025-05-14)


### Features

* Add Niching policies to reduce disk space ([#1218](https://github.com/TheOpenDictionary/odict/issues/1218)) ([c33ac83](https://github.com/TheOpenDictionary/odict/commit/c33ac83972158813f76b5039e2aab55251dd83b8))
* Add Other(String) to PartOfSpeech enum to handle custom strings ([#1203](https://github.com/TheOpenDictionary/odict/issues/1203)) ([b7f258a](https://github.com/TheOpenDictionary/odict/commit/b7f258aa35a8bb29ccc6f369ae3316435b5f6c03))
* Replace HashMap with HashSet ([#1217](https://github.com/TheOpenDictionary/odict/issues/1217)) ([fdb5e11](https://github.com/TheOpenDictionary/odict/commit/fdb5e111ea84b179156486eacf4b78d843c12efb))

## [2.7.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.6.0...cli/v2.7.0) (2025-05-10)


### Miscellaneous Chores

* **cli:** Synchronize odict versions

## [2.6.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.4.1...cli/v2.6.0) (2025-04-24)


### Miscellaneous Chores

* **cli:** Synchronize odict versions

## [2.4.1](https://github.com/TheOpenDictionary/odict/compare/cli/v2.4.0...cli/v2.4.1) (2025-04-20)

## [2.4.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.3.0...cli/v2.4.0) (2025-04-13)


### Features

* Add flag to support case insensitivity for lookups and tokenization ([#1179](https://github.com/TheOpenDictionary/odict/issues/1179)) ([a7e7baa](https://github.com/TheOpenDictionary/odict/commit/a7e7baac0f8d02e565a2d01acdc59c9bd1bc3242))

## [2.3.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.2.0...cli/v2.3.0) (2025-04-04)


### Features

* **cli:** Add spinner to compile command ([3909dad](https://github.com/TheOpenDictionary/odict/commit/3909dad800ed9bd7cedf18437e239a9a35dc60f3))

## [2.2.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.1.0...cli/v2.2.0) (2025-04-03)


### Features

* **serve:** Cache dictionaries ([#1170](https://github.com/TheOpenDictionary/odict/issues/1170)) ([907918b](https://github.com/TheOpenDictionary/odict/commit/907918ba12b34b44399bfa185a0dd5f8c8575ad0))

## [2.1.0](https://github.com/TheOpenDictionary/odict/compare/cli/v2.0.0...cli/v2.1.0) (2025-04-03)


### Features

* Add option to print entries as Markdown/HTML ([#1068](https://github.com/TheOpenDictionary/odict/issues/1068)) ([3422533](https://github.com/TheOpenDictionary/odict/commit/3422533514264dbe80e6ff4c6ac4e3c12f289ee8))
* Add WASM support to Node package ([#1155](https://github.com/TheOpenDictionary/odict/issues/1155)) ([3456969](https://github.com/TheOpenDictionary/odict/commit/3456969422df2530693c196bafefa7cd92fb2f12))
* **core:** Add new tokenization feature ([#1159](https://github.com/TheOpenDictionary/odict/issues/1159)) ([d9196c1](https://github.com/TheOpenDictionary/odict/commit/d9196c1aae4c275d3c326d5803f7baf65f7b5a89))
* Expose Brotli compression options for `compile` ([#1067](https://github.com/TheOpenDictionary/odict/issues/1067)) ([0b282cd](https://github.com/TheOpenDictionary/odict/commit/0b282cde171ee3e6b1252c59fa9fc8f050e7c4b4))


### Bug Fixes

* **deps:** Update rust crate derive_more to v2 ([#1100](https://github.com/TheOpenDictionary/odict/issues/1100)) ([959cb1a](https://github.com/TheOpenDictionary/odict/commit/959cb1af01db57e7fef781fccdd16261046a710d))
* **deps:** Update rust crate pulldown-cmark to 0.13.0 ([#1109](https://github.com/TheOpenDictionary/odict/issues/1109)) ([80bb314](https://github.com/TheOpenDictionary/odict/commit/80bb314e1fdedb11d2fe59b1ccf446628c5a1dd9))
* Fix CLI description ([52e92f7](https://github.com/TheOpenDictionary/odict/commit/52e92f7a45e620ef586217f48b1f6d93ee9104d8))

## [2.0.0](https://github.com/TheOpenDictionary/odict/compare/cli-v2.0.0...cli/v2.0.0) (2024-12-26)


### Features

* Add indexing ([#656](https://github.com/TheOpenDictionary/odict/issues/656)) ([a94db99](https://github.com/TheOpenDictionary/odict/commit/a94db9953c34df96bedff5c3ebde989a64d27ace))
* Add stable Python binding 🎉 ([#1049](https://github.com/TheOpenDictionary/odict/issues/1049)) ([73c6e33](https://github.com/TheOpenDictionary/odict/commit/73c6e339b8614c6eb048de4ee7586dd5aa98803e))
* Change binary name to odict ([99d8436](https://github.com/TheOpenDictionary/odict/commit/99d8436539c4962f4559848b18371d77c3b381b1))
* **config:** Port `alias` logic and CLI command to Rust ([#641](https://github.com/TheOpenDictionary/odict/issues/641)) ([853bf43](https://github.com/TheOpenDictionary/odict/commit/853bf435ecf6808a8f7d0daa724802de9dac43f1))
* Deployment changes ([#1002](https://github.com/TheOpenDictionary/odict/issues/1002)) ([981c223](https://github.com/TheOpenDictionary/odict/commit/981c2232fe8908cb9a0afd95f6c04e32a4c698ed))
* Finalize search ([#693](https://github.com/TheOpenDictionary/odict/issues/693)) ([0e17c88](https://github.com/TheOpenDictionary/odict/commit/0e17c88142befd6c221a0008f30688a05151b865))
* Fix NAPI CI ([#1015](https://github.com/TheOpenDictionary/odict/issues/1015)) ([c688d43](https://github.com/TheOpenDictionary/odict/commit/c688d43ecb1059182ab53b2ab9042148f9dbf981))
* Reduce unnecessary Lazy declarations and remove once_cell ([#1016](https://github.com/TheOpenDictionary/odict/issues/1016)) ([6bcea66](https://github.com/TheOpenDictionary/odict/commit/6bcea668331fd191e967a1a1dabbd4dc9eeeb885))
* Replace just runners with mise tasks ([0275910](https://github.com/TheOpenDictionary/odict/commit/0275910feff1f100a464d5d95a92ebfef95d4e6f))
* Update CLI package name ([6c0ad7a](https://github.com/TheOpenDictionary/odict/commit/6c0ad7a1948ad523fea36b88fc6423dce4a4975f))
* **v2:** Add `new` command ([#700](https://github.com/TheOpenDictionary/odict/issues/700)) ([6f65dc3](https://github.com/TheOpenDictionary/odict/commit/6f65dc371ae4b51600673b853353406ecaf92cb3))
* **v2:** Add info command ([#800](https://github.com/TheOpenDictionary/odict/issues/800)) ([8b73cc4](https://github.com/TheOpenDictionary/odict/commit/8b73cc4e687708abc90848740b827986391a2175))
* **v2:** Add pretty printing ([#701](https://github.com/TheOpenDictionary/odict/issues/701)) ([e24160f](https://github.com/TheOpenDictionary/odict/commit/e24160f4023b1be97b0d8cb98e03b82cecdedd8e))
* **v2:** Add Rust Node bindings ([#760](https://github.com/TheOpenDictionary/odict/issues/760)) ([aac5501](https://github.com/TheOpenDictionary/odict/commit/aac550181f6d144649ce9ad0ff823967b29668bf))
* **v2:** Add serve command ([#748](https://github.com/TheOpenDictionary/odict/issues/748)) ([ff10753](https://github.com/TheOpenDictionary/odict/commit/ff107533fcb25094230770b8c51697348caa6fc2))
* **v2:** Add SQL dumping ([#747](https://github.com/TheOpenDictionary/odict/issues/747)) ([caceb88](https://github.com/TheOpenDictionary/odict/commit/caceb883e527358a0f0e74221130af572c0f561a))
* **v2:** Implement basic dumping ([#654](https://github.com/TheOpenDictionary/odict/issues/654)) ([5e29764](https://github.com/TheOpenDictionary/odict/commit/5e29764048767752c56178df5e1ac1e9160894d0))


### Bug Fixes

* **deps:** Update module github.com/google/flatbuffers to v24 ([aa73d33](https://github.com/TheOpenDictionary/odict/commit/aa73d33d6685f6b15d4223943967c748d1bae8bd))
* **deps:** Update module github.com/google/flatbuffers to v24.3.7+incompatible ([2e9a949](https://github.com/TheOpenDictionary/odict/commit/2e9a949bc475bd11e294717b7e81ed4c48023138))
* **deps:** Update rust crate clap to 4.5.4 ([e8f91a3](https://github.com/TheOpenDictionary/odict/commit/e8f91a3bc00743e73e8ca1efbf21640c42935d61))
* **deps:** Update rust crate pulldown-cmark to 0.10.2 ([6303c50](https://github.com/TheOpenDictionary/odict/commit/6303c50f9fb4b1de0e0c7717bfe49fdb255de0af))
* **deps:** Update rust crate pulldown-cmark to 0.10.3 ([0311794](https://github.com/TheOpenDictionary/odict/commit/031179459880fc9a4a1e9ce146205ffb9b744ada))
* **deps:** Update rust crate pulldown-cmark to 0.11.0 ([d4e85f5](https://github.com/TheOpenDictionary/odict/commit/d4e85f54b2c973721ff99fb5c091489223927b35))
* **deps:** Update rust crate pulldown-cmark to 0.12.0 ([0885bf8](https://github.com/TheOpenDictionary/odict/commit/0885bf87c345103af6cb6138d220cba661b0f9ff))
* **deps:** Update rust crate serde to 1.0.198 ([2701027](https://github.com/TheOpenDictionary/odict/commit/2701027fe3ce7c7847f3e92a59b0b6092e73d941))
* **deps:** Update rust crate serde to 1.0.199 ([cca009c](https://github.com/TheOpenDictionary/odict/commit/cca009cf62b3a8a92f48c5172d222a4b3844a93a))
* **deps:** Update rust crate serde to 1.0.200 ([a192028](https://github.com/TheOpenDictionary/odict/commit/a19202869063703dc23e36ca9adbab5f04063d4e))
* Fix dist ([2402b73](https://github.com/TheOpenDictionary/odict/commit/2402b73b43c1f8f202a8317253c115e84faa5953))
