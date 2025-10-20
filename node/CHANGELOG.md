# Changelog





## [2.1.0](https://github.com/TheOpenDictionary/odict/compare/node/v2.0.0...node/v2.1.0) (2025-10-20)


### Features

* Allow passing remote load options to Node and Python ([#1315](https://github.com/TheOpenDictionary/odict/issues/1315)) ([80a8361](https://github.com/TheOpenDictionary/odict/commit/80a8361e1cff88bffb5bf36cb354ca04ad89a343))
* Remove backwards compatibility for legacy follow for Node and Python ([#1308](https://github.com/TheOpenDictionary/odict/issues/1308)) ([785fac5](https://github.com/TheOpenDictionary/odict/commit/785fac54001df45841fa18d0c6822e36a337b5ec))
* Remove backwards compatibility for legacy follow for Node and Python ([#1310](https://github.com/TheOpenDictionary/odict/issues/1310)) ([2efe45d](https://github.com/TheOpenDictionary/odict/commit/2efe45d44babc1357bd10650360b4edd671513dd))
* String interning ([#1313](https://github.com/TheOpenDictionary/odict/issues/1313)) ([a1ce402](https://github.com/TheOpenDictionary/odict/commit/a1ce4025950f840674e1e4e159b60311672febc2))

## [2.0.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.9.1...node/v2.0.0) (2025-10-17)


### ⚠ BREAKING CHANGES

* support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282))

### Features

* **lookup:** Update follow logic ([#1307](https://github.com/TheOpenDictionary/odict/issues/1307)) ([0aa9a53](https://github.com/TheOpenDictionary/odict/commit/0aa9a532def5b1f544f75f2e7ad3540e3f90cf55))
* Support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282)) ([326b325](https://github.com/TheOpenDictionary/odict/commit/326b325efd4a1ea1327ae2e36f55fe6c13663ca1))
* Use safe API for rkyv access ([dd3fd60](https://github.com/TheOpenDictionary/odict/commit/dd3fd60d1795538108ed0cb02281ccc792eec4f1))
* Use uv for python management ([9b1971c](https://github.com/TheOpenDictionary/odict/commit/9b1971c645310103364fb96e1efb3c01a64ba536))

## [1.9.1](https://github.com/TheOpenDictionary/odict/compare/node/v1.9.0...node/v1.9.1) (2025-08-16)


### Bug Fixes

* Update Node requirement ([b9a0a35](https://github.com/TheOpenDictionary/odict/commit/b9a0a3590ca621976a5025c274453a619c84e299))

## [1.9.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.8.0...node/v1.9.0) (2025-07-12)


### Features

* Add `min_rank` and `max_rank` methods for retrieving ranking bounds ([#1245](https://github.com/TheOpenDictionary/odict/issues/1245)) ([3cff5ee](https://github.com/TheOpenDictionary/odict/commit/3cff5ee302b5a2281c3ee8fdb75b1ee553760b94))
* Allow custom `follow` limit by changing from boolean to number ([#1246](https://github.com/TheOpenDictionary/odict/issues/1246)) ([d96187a](https://github.com/TheOpenDictionary/odict/commit/d96187a541220f2e934462a31af363f64786d623))

## [1.8.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.7.0...node/v1.8.0) (2025-05-14)


### Features

* Add Other(String) to PartOfSpeech enum to handle custom strings ([#1203](https://github.com/TheOpenDictionary/odict/issues/1203)) ([b7f258a](https://github.com/TheOpenDictionary/odict/commit/b7f258aa35a8bb29ccc6f369ae3316435b5f6c03))
* Replace HashMap with HashSet ([#1217](https://github.com/TheOpenDictionary/odict/issues/1217)) ([fdb5e11](https://github.com/TheOpenDictionary/odict/commit/fdb5e111ea84b179156486eacf4b78d843c12efb))

## [1.7.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.6.0...node/v1.7.0) (2025-05-10)


### Features

* Add `rank` attribute to Entry for storing word frequencies ([#1210](https://github.com/TheOpenDictionary/odict/issues/1210)) ([2b2439a](https://github.com/TheOpenDictionary/odict/commit/2b2439a4dcb82d2b2c247174eb22d4a90d2037d5))

## [1.6.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.5.0...node/v1.6.0) (2025-04-24)


### Features

* Add &lt;pronunciation&gt; tag ([#1197](https://github.com/TheOpenDictionary/odict/issues/1197)) ([026e2f2](https://github.com/TheOpenDictionary/odict/commit/026e2f292bf02c182684f8656b9eaa13533f1a72))
* Add `tags` fields to `sense` and `form`, as well as move `lemma` to `<sense>` ([#1192](https://github.com/TheOpenDictionary/odict/issues/1192)) ([f03d8c1](https://github.com/TheOpenDictionary/odict/commit/f03d8c122f96ec20f85ccff639962c9ffa44aee5))
* Add media URLs to `<entry>` nodes ([#1202](https://github.com/TheOpenDictionary/odict/issues/1202)) ([04c2307](https://github.com/TheOpenDictionary/odict/commit/04c2307b005ff57bbe460f7b1034f97c811b580f))
* Add translation node ([#1196](https://github.com/TheOpenDictionary/odict/issues/1196)) ([ef15aba](https://github.com/TheOpenDictionary/odict/commit/ef15abaf0749cf014315fb57ec63c50d2ae59e98))
* Move `translations` and `forms` to `sense` ([#1200](https://github.com/TheOpenDictionary/odict/issues/1200)) ([5c0e746](https://github.com/TheOpenDictionary/odict/commit/5c0e7466df84f5435cc53eba7fcc72813c11d28c))
* Remove wrapper components ([0908f01](https://github.com/TheOpenDictionary/odict/commit/0908f0128c1dd1b0749b756d757d8f3aa50e6c1c))
* Use `structural_convert` macro instead of custom From implementations ([#1199](https://github.com/TheOpenDictionary/odict/issues/1199)) ([392d624](https://github.com/TheOpenDictionary/odict/commit/392d624a4b956f0bc22d0529b4ccb0307807cdfd))


### Bug Fixes

* Remove duplicate Node versions from Changelog ([555301b](https://github.com/TheOpenDictionary/odict/commit/555301bb4ca51ef32193fb64fc486fcbf22f30f9))

## [1.5.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.4.0...node/v1.5.0) (2025-04-20)


### Features

* Add support for word forms and lemmas ([#1186](https://github.com/TheOpenDictionary/odict/issues/1186)) ([9e37a28](https://github.com/TheOpenDictionary/odict/commit/9e37a2834fda82bfaf558aeab9cc74fbced5a1d4))

## [1.4.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.3.0...node/v1.4.0) (2025-04-13)


### Features

* Add flag to support case insensitivity for lookups and tokenization ([#1179](https://github.com/TheOpenDictionary/odict/issues/1179)) ([a7e7baa](https://github.com/TheOpenDictionary/odict/commit/a7e7baac0f8d02e565a2d01acdc59c9bd1bc3242))

## [1.3.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.2.0...node/v1.3.0) (2025-04-04)


### Features

* **node:** Add support for TokenizeOptions ([e14fb17](https://github.com/TheOpenDictionary/odict/commit/e14fb17abcaa2f07bfabb482db11402cd2b41fbf))
* **tokenize:** Add additional metadata to Token model ([bd44701](https://github.com/TheOpenDictionary/odict/commit/bd44701bb3ef59fafac31a2b6582c729fd881f1e))


## [1.2.0](https://github.com/TheOpenDictionary/odict/compare/node/v1.1.1...node/v1.2.0) (2025-04-03)


### Features

* Add option to print entries as Markdown/HTML ([#1068](https://github.com/TheOpenDictionary/odict/issues/1068)) ([3422533](https://github.com/TheOpenDictionary/odict/commit/3422533514264dbe80e6ff4c6ac4e3c12f289ee8))
* Add stable Python binding 🎉 ([#1049](https://github.com/TheOpenDictionary/odict/issues/1049)) ([73c6e33](https://github.com/TheOpenDictionary/odict/commit/73c6e339b8614c6eb048de4ee7586dd5aa98803e))
* Add WASM support to Node package ([#1155](https://github.com/TheOpenDictionary/odict/issues/1155)) ([3456969](https://github.com/TheOpenDictionary/odict/commit/3456969422df2530693c196bafefa7cd92fb2f12))
* **ci:** Add workflow dispatch trigger for Node release ([24ce45b](https://github.com/TheOpenDictionary/odict/commit/24ce45b8678edcf92779031104c9b21614318bfa))
* **core:** Add new tokenization feature ([#1159](https://github.com/TheOpenDictionary/odict/issues/1159)) ([d9196c1](https://github.com/TheOpenDictionary/odict/commit/d9196c1aae4c275d3c326d5803f7baf65f7b5a89))
* Deployment changes ([#1002](https://github.com/TheOpenDictionary/odict/issues/1002)) ([981c223](https://github.com/TheOpenDictionary/odict/commit/981c2232fe8908cb9a0afd95f6c04e32a4c698ed))
* Remove universal darwin build ([c20d340](https://github.com/TheOpenDictionary/odict/commit/c20d340b5ec44a2c5dd84360dc9e5ff8bde8cba8))
* Rename Python library ([928343a](https://github.com/TheOpenDictionary/odict/commit/928343a7df53d64aa25d7e262f21f4aa0f09cc5e))
* Try updating CI ([c76c684](https://github.com/TheOpenDictionary/odict/commit/c76c684d427ef79df81e4c9a349dadba3a54339c))
* **v2:** Add Rust Node bindings ([#760](https://github.com/TheOpenDictionary/odict/issues/760)) ([aac5501](https://github.com/TheOpenDictionary/odict/commit/aac550181f6d144649ce9ad0ff823967b29668bf))


### Bug Fixes

* Bump node version ([221175a](https://github.com/TheOpenDictionary/odict/commit/221175a37d7275223b31011a208b375e57a36842))
* CI ([c63c3f9](https://github.com/TheOpenDictionary/odict/commit/c63c3f98e816aa626a74e600bdc10554ddf23e7f))
* **ci:** Remove charabia from Node package ([cf02d0c](https://github.com/TheOpenDictionary/odict/commit/cf02d0c6f7a6b9015c88e563e7d24a846428b145))
* **ci:** Test JS CI ([86adb85](https://github.com/TheOpenDictionary/odict/commit/86adb853bd772f94d191c38406ed9e3e7b78ba28))
* **ci:** Test JS CI ([c6230cd](https://github.com/TheOpenDictionary/odict/commit/c6230cde4137ff578839380ef8b18b34f0c226ac))
* Fix Node package version ([deda7b1](https://github.com/TheOpenDictionary/odict/commit/deda7b13ecfc386240682c2ee9d41b33791d7505))
* Fix Node package.json ([5203047](https://github.com/TheOpenDictionary/odict/commit/520304788d1d016c69c9643ed39b069db7844f14))
* **node:** Add repository field to Node package ([333c2b9](https://github.com/TheOpenDictionary/odict/commit/333c2b9ef3a225668d71f18b412ecb35eb63ca8a))
* **node:** Don't create new release ([8fb1a95](https://github.com/TheOpenDictionary/odict/commit/8fb1a95d4164ea9ed7d426342c260389bc08eed2))
* **node:** Fix prepublish ([6249f4a](https://github.com/TheOpenDictionary/odict/commit/6249f4af2e8c2ff627405bd21e41bd0eaac10a60))
* Update release please manifest ([3e633d5](https://github.com/TheOpenDictionary/odict/commit/3e633d58e7b56f3268f63b8eb46d4a514c229c31))


## [1.1.0](https://github.com/TheOpenDictionary/odict/compare/node-v1.0.0...node/v1.1.0) (2025-04-03)


### Features

* Add option to print entries as Markdown/HTML ([#1068](https://github.com/TheOpenDictionary/odict/issues/1068)) ([3422533](https://github.com/TheOpenDictionary/odict/commit/3422533514264dbe80e6ff4c6ac4e3c12f289ee8))
* Add WASM support to Node package ([#1155](https://github.com/TheOpenDictionary/odict/issues/1155)) ([3456969](https://github.com/TheOpenDictionary/odict/commit/3456969422df2530693c196bafefa7cd92fb2f12))
* **core:** Add new tokenization feature ([#1159](https://github.com/TheOpenDictionary/odict/issues/1159)) ([d9196c1](https://github.com/TheOpenDictionary/odict/commit/d9196c1aae4c275d3c326d5803f7baf65f7b5a89))

## [1.0.0](https://github.com/TheOpenDictionary/odict/compare/node-v1.0.0...node-v1.0.0) (2024-12-26)


### Features

* Add stable Python binding 🎉 ([#1049](https://github.com/TheOpenDictionary/odict/issues/1049)) ([73c6e33](https://github.com/TheOpenDictionary/odict/commit/73c6e339b8614c6eb048de4ee7586dd5aa98803e))
* **ci:** Add workflow dispatch trigger for Node release ([24ce45b](https://github.com/TheOpenDictionary/odict/commit/24ce45b8678edcf92779031104c9b21614318bfa))
* Deployment changes ([#1002](https://github.com/TheOpenDictionary/odict/issues/1002)) ([981c223](https://github.com/TheOpenDictionary/odict/commit/981c2232fe8908cb9a0afd95f6c04e32a4c698ed))
* Rename Python library ([928343a](https://github.com/TheOpenDictionary/odict/commit/928343a7df53d64aa25d7e262f21f4aa0f09cc5e))
* Try updating CI ([c76c684](https://github.com/TheOpenDictionary/odict/commit/c76c684d427ef79df81e4c9a349dadba3a54339c))
* **v2:** Add Rust Node bindings ([#760](https://github.com/TheOpenDictionary/odict/issues/760)) ([aac5501](https://github.com/TheOpenDictionary/odict/commit/aac550181f6d144649ce9ad0ff823967b29668bf))


### Bug Fixes

* **ci:** Remove charabia from Node package ([cf02d0c](https://github.com/TheOpenDictionary/odict/commit/cf02d0c6f7a6b9015c88e563e7d24a846428b145))
* **ci:** Test JS CI ([86adb85](https://github.com/TheOpenDictionary/odict/commit/86adb853bd772f94d191c38406ed9e3e7b78ba28))
* **ci:** Test JS CI ([c6230cd](https://github.com/TheOpenDictionary/odict/commit/c6230cde4137ff578839380ef8b18b34f0c226ac))
* **node:** Add repository field to Node package ([333c2b9](https://github.com/TheOpenDictionary/odict/commit/333c2b9ef3a225668d71f18b412ecb35eb63ca8a))
* **node:** Don't create new release ([8fb1a95](https://github.com/TheOpenDictionary/odict/commit/8fb1a95d4164ea9ed7d426342c260389bc08eed2))
* **node:** Fix prepublish ([6249f4a](https://github.com/TheOpenDictionary/odict/commit/6249f4af2e8c2ff627405bd21e41bd0eaac10a60))
