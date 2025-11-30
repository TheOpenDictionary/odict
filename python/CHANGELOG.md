# Changelog

## [2.2.1](https://github.com/TheOpenDictionary/odict/compare/python/v2.2.0...python/v2.2.1) (2025-11-30)


### Bug Fixes

* Add automatic retrying if remote downloads are corrupted ([#1355](https://github.com/TheOpenDictionary/odict/issues/1355)) ([333f6ca](https://github.com/TheOpenDictionary/odict/commit/333f6cacfe2c1f966ea2b0528c5de48b99ed4fc5))

## [2.2.0](https://github.com/TheOpenDictionary/odict/compare/python/v2.1.1...python/v2.2.0) (2025-11-22)


### Features

* Preserve ordering of sense data ([#1339](https://github.com/TheOpenDictionary/odict/issues/1339)) ([777b8e9](https://github.com/TheOpenDictionary/odict/commit/777b8e96fbb35c0cdfab6f63fcd8f522c6c1cb3f))

## [2.1.1](https://github.com/TheOpenDictionary/odict/compare/python/v2.1.0...python/v2.1.1) (2025-10-22)


### Bug Fixes

* Allow configuring config directory through LoadOptions ([#1323](https://github.com/TheOpenDictionary/odict/issues/1323)) ([3808745](https://github.com/TheOpenDictionary/odict/commit/3808745641f75018f71c29fdfcd8eb32ab1406f0))
* Remove unused merge crate from python and node ([81d6685](https://github.com/TheOpenDictionary/odict/commit/81d668530caa068660c76dc5235241335d37ff98))

## [2.1.0](https://github.com/TheOpenDictionary/odict/compare/python/v2.0.0...python/v2.1.0) (2025-10-20)


### Features

* Allow passing remote load options to Node and Python ([#1315](https://github.com/TheOpenDictionary/odict/issues/1315)) ([80a8361](https://github.com/TheOpenDictionary/odict/commit/80a8361e1cff88bffb5bf36cb354ca04ad89a343))
* Remove backwards compatibility for legacy follow for Node and Python ([#1308](https://github.com/TheOpenDictionary/odict/issues/1308)) ([785fac5](https://github.com/TheOpenDictionary/odict/commit/785fac54001df45841fa18d0c6822e36a337b5ec))
* Remove backwards compatibility for legacy follow for Node and Python ([#1310](https://github.com/TheOpenDictionary/odict/issues/1310)) ([2efe45d](https://github.com/TheOpenDictionary/odict/commit/2efe45d44babc1357bd10650360b4edd671513dd))

## [2.0.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.8.0...python/v2.0.0) (2025-10-17)


### ⚠ BREAKING CHANGES

* support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282))

### Features

* **lookup:** Update follow logic ([#1307](https://github.com/TheOpenDictionary/odict/issues/1307)) ([0aa9a53](https://github.com/TheOpenDictionary/odict/commit/0aa9a532def5b1f544f75f2e7ad3540e3f90cf55))
* Support for remote dictionaries ([#1282](https://github.com/TheOpenDictionary/odict/issues/1282)) ([326b325](https://github.com/TheOpenDictionary/odict/commit/326b325efd4a1ea1327ae2e36f55fe6c13663ca1))
* Use uv for python management ([9b1971c](https://github.com/TheOpenDictionary/odict/commit/9b1971c645310103364fb96e1efb3c01a64ba536))

## [1.8.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.7.0...python/v1.8.0) (2025-07-12)


### Features

* Add `min_rank` and `max_rank` methods for retrieving ranking bounds ([#1245](https://github.com/TheOpenDictionary/odict/issues/1245)) ([3cff5ee](https://github.com/TheOpenDictionary/odict/commit/3cff5ee302b5a2281c3ee8fdb75b1ee553760b94))
* Allow custom `follow` limit by changing from boolean to number ([#1246](https://github.com/TheOpenDictionary/odict/issues/1246)) ([d96187a](https://github.com/TheOpenDictionary/odict/commit/d96187a541220f2e934462a31af363f64786d623))

## [1.7.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.6.0...python/v1.7.0) (2025-05-14)


### Features

* Add Other(String) to PartOfSpeech enum to handle custom strings ([#1203](https://github.com/TheOpenDictionary/odict/issues/1203)) ([b7f258a](https://github.com/TheOpenDictionary/odict/commit/b7f258aa35a8bb29ccc6f369ae3316435b5f6c03))
* Replace HashMap with HashSet ([#1217](https://github.com/TheOpenDictionary/odict/issues/1217)) ([fdb5e11](https://github.com/TheOpenDictionary/odict/commit/fdb5e111ea84b179156486eacf4b78d843c12efb))


### Bug Fixes

* **deps:** Update rust crate pyo3 to 0.25.0 ([#1221](https://github.com/TheOpenDictionary/odict/issues/1221)) ([8105fde](https://github.com/TheOpenDictionary/odict/commit/8105fde943c4c4d0fad74c9e00c8cc01d70f199a))

## [1.6.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.5.0...python/v1.6.0) (2025-05-10)


### Features

* Add `rank` attribute to Entry for storing word frequencies ([#1210](https://github.com/TheOpenDictionary/odict/issues/1210)) ([2b2439a](https://github.com/TheOpenDictionary/odict/commit/2b2439a4dcb82d2b2c247174eb22d4a90d2037d5))

## [1.5.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.4.0...python/v1.5.0) (2025-04-24)


### Features

* Add &lt;pronunciation&gt; tag ([#1197](https://github.com/TheOpenDictionary/odict/issues/1197)) ([026e2f2](https://github.com/TheOpenDictionary/odict/commit/026e2f292bf02c182684f8656b9eaa13533f1a72))
* Add `tags` fields to `sense` and `form`, as well as move `lemma` to `<sense>` ([#1192](https://github.com/TheOpenDictionary/odict/issues/1192)) ([f03d8c1](https://github.com/TheOpenDictionary/odict/commit/f03d8c122f96ec20f85ccff639962c9ffa44aee5))
* Add media URLs to `<entry>` nodes ([#1202](https://github.com/TheOpenDictionary/odict/issues/1202)) ([04c2307](https://github.com/TheOpenDictionary/odict/commit/04c2307b005ff57bbe460f7b1034f97c811b580f))
* Add translation node ([#1196](https://github.com/TheOpenDictionary/odict/issues/1196)) ([ef15aba](https://github.com/TheOpenDictionary/odict/commit/ef15abaf0749cf014315fb57ec63c50d2ae59e98))
* Move `translations` and `forms` to `sense` ([#1200](https://github.com/TheOpenDictionary/odict/issues/1200)) ([5c0e746](https://github.com/TheOpenDictionary/odict/commit/5c0e7466df84f5435cc53eba7fcc72813c11d28c))
* Remove wrapper components ([0908f01](https://github.com/TheOpenDictionary/odict/commit/0908f0128c1dd1b0749b756d757d8f3aa50e6c1c))
* Use `structural_convert` macro instead of custom From implementations ([#1199](https://github.com/TheOpenDictionary/odict/issues/1199)) ([392d624](https://github.com/TheOpenDictionary/odict/commit/392d624a4b956f0bc22d0529b4ccb0307807cdfd))

## [1.4.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.3.0...python/v1.4.0) (2025-04-20)


### Features

* Add support for word forms and lemmas ([#1186](https://github.com/TheOpenDictionary/odict/issues/1186)) ([9e37a28](https://github.com/TheOpenDictionary/odict/commit/9e37a2834fda82bfaf558aeab9cc74fbced5a1d4))

## [1.3.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.2.0...python/v1.3.0) (2025-04-13)


### Features

* Add flag to support case insensitivity for lookups and tokenization ([#1179](https://github.com/TheOpenDictionary/odict/issues/1179)) ([a7e7baa](https://github.com/TheOpenDictionary/odict/commit/a7e7baac0f8d02e565a2d01acdc59c9bd1bc3242))

## [1.2.0](https://github.com/TheOpenDictionary/odict/compare/python/v1.1.1...python/v1.2.0) (2025-04-04)


### Features

* **tokenize:** Add additional metadata to Token model ([bd44701](https://github.com/TheOpenDictionary/odict/commit/bd44701bb3ef59fafac31a2b6582c729fd881f1e))

## [1.1.1](https://github.com/TheOpenDictionary/odict/compare/python/v1.1.0...python/v1.1.1) (2025-04-03)


### Bug Fixes

* Update Python version ([f9ec497](https://github.com/TheOpenDictionary/odict/commit/f9ec4972f3906185863dd9cdac5d02306292c483))

## [1.0.0](https://github.com/TheOpenDictionary/odict/compare/python-v1.0.0...python/v1.0.0) (2025-04-03)


### Features

* Add option to print entries as Markdown/HTML ([#1068](https://github.com/TheOpenDictionary/odict/issues/1068)) ([3422533](https://github.com/TheOpenDictionary/odict/commit/3422533514264dbe80e6ff4c6ac4e3c12f289ee8))
* **core:** Add new tokenization feature ([#1159](https://github.com/TheOpenDictionary/odict/issues/1159)) ([d9196c1](https://github.com/TheOpenDictionary/odict/commit/d9196c1aae4c275d3c326d5803f7baf65f7b5a89))


### Bug Fixes

* **deps:** Update rust crate pyo3 to 0.24.0 ([#1141](https://github.com/TheOpenDictionary/odict/issues/1141)) ([3fb50bd](https://github.com/TheOpenDictionary/odict/commit/3fb50bd371fae1163e2f0acdb2c68e4692555d94))

## [1.0.0](https://github.com/TheOpenDictionary/odict/compare/python-v1.0.0...python-v1.0.0) (2024-12-25)


### Features

* Add indexing ([#656](https://github.com/TheOpenDictionary/odict/issues/656)) ([a94db99](https://github.com/TheOpenDictionary/odict/commit/a94db9953c34df96bedff5c3ebde989a64d27ace))
* Add stable Python binding 🎉 ([#1049](https://github.com/TheOpenDictionary/odict/issues/1049)) ([73c6e33](https://github.com/TheOpenDictionary/odict/commit/73c6e339b8614c6eb048de4ee7586dd5aa98803e))
* Rename Python library ([928343a](https://github.com/TheOpenDictionary/odict/commit/928343a7df53d64aa25d7e262f21f4aa0f09cc5e))
* Swap asdf with mise ([1451356](https://github.com/TheOpenDictionary/odict/commit/145135680138d5438e98d1f1d61a9b82edba9c7c))
* Update Python package version ([1980388](https://github.com/TheOpenDictionary/odict/commit/19803884381c9f8e6483e35d73f93351529950e1))
