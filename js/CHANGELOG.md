# Changelog

## [0.8.1](https://github.com/TheOpenDictionary/odict/compare/js-v0.8.0...js-v0.8.1) (2023-09-23)


### Bug Fixes

* **js:** Add ready check for service ([#458](https://github.com/TheOpenDictionary/odict/issues/458)) ([ed984ba](https://github.com/TheOpenDictionary/odict/commit/ed984bad21af2c57f8e8c421a8ffe81c1b3fb8fe))
* **js:** Make notes optional ([ee72c66](https://github.com/TheOpenDictionary/odict/commit/ee72c6666cfc37f740946b801dc68190a46b22e4))

## [0.8.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.7.0...js-v0.8.0) (2023-09-17)


### Features

* **core:** add DefinitionNote type ([#450](https://github.com/TheOpenDictionary/odict/issues/450)) ([08bcac4](https://github.com/TheOpenDictionary/odict/commit/08bcac4b0710112166d6a43429fbb2f0c784c241))
* **js:** remove detectOpenHandles from test run ([2eed906](https://github.com/TheOpenDictionary/odict/commit/2eed906830a569a89b9f74d4a5058f71fde8dd10))

## [0.7.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.6.1...js-v0.7.0) (2023-06-28)


### Features

* add `serve` command ([#393](https://github.com/TheOpenDictionary/odict/issues/393)) ([cc7ef5c](https://github.com/TheOpenDictionary/odict/commit/cc7ef5c69811a4554e3b62bf492090de65ae86ea))
* add support for dictionary aliasing ([#391](https://github.com/TheOpenDictionary/odict/issues/391)) ([c6f1a04](https://github.com/TheOpenDictionary/odict/commit/c6f1a0495136817065007acc4414194f4e8445ad))

## [0.6.1](https://github.com/TheOpenDictionary/odict/compare/js-v0.6.0...js-v0.6.1) (2023-05-31)


### Bug Fixes

* **js:** fixed PartOfSpeech map types ([725bba4](https://github.com/TheOpenDictionary/odict/commit/725bba46a5f31580ed3207b7bea2e302680b49fb))

## [0.6.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.5.1...js-v0.6.0) (2023-05-31)


### Features

* **js:** add PartOfSpeech type ([8ee6e9a](https://github.com/TheOpenDictionary/odict/commit/8ee6e9a119445807af7e5fe4017e54e2fde0474a))

## [0.5.1](https://github.com/TheOpenDictionary/odict/compare/js-v0.5.0...js-v0.5.1) (2023-05-29)


### Bug Fixes

* attempt to fix tests ([eda588a](https://github.com/TheOpenDictionary/odict/commit/eda588ae0e6b9c971df3bee4c2d598d9a95a22e8))
* **js:** fix test ([0487c39](https://github.com/TheOpenDictionary/odict/commit/0487c39c173fdd52c35c5f959ddc4c2d23cd40af))
* **js:** restart process if there are unexpected exits ([388176c](https://github.com/TheOpenDictionary/odict/commit/388176c60145ba1ba02ca4dc3a54c8fb3a688b68))

## [0.5.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.4.1...js-v0.5.0) (2023-05-28)


### Features

* use IPC library for inter-process communication ([#344](https://github.com/TheOpenDictionary/odict/issues/344)) ([fc23307](https://github.com/TheOpenDictionary/odict/commit/fc2330771dcdbfe6358fa09f15f1cdf86bd4b9ab))

## [0.4.1](https://github.com/TheOpenDictionary/odict/compare/js-v0.4.0...js-v0.4.1) (2023-05-28)


### Bug Fixes

* **js:** remove log statements ([810e6f7](https://github.com/TheOpenDictionary/odict/commit/810e6f7b0a772cc0d58797f02ff4c962a9e98bcc))

## [0.4.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.3.2...js-v0.4.0) (2023-05-28)


### Features

* **js:** add part of speech enum to js package ([0fdd9bc](https://github.com/TheOpenDictionary/odict/commit/0fdd9bc073a20769a6bb80939290a8981d526920))


### Bug Fixes

* **deps:** update dependency flatbuffers to v23.5.26 ([60b6baf](https://github.com/TheOpenDictionary/odict/commit/60b6baf51caaf8d835a5bc8d2c1c052926224364))

## [0.3.2](https://github.com/TheOpenDictionary/odict/compare/js-v0.3.1...js-v0.3.2) (2023-05-20)


### Bug Fixes

* **js:** bug fixes with service implementation ([#325](https://github.com/TheOpenDictionary/odict/issues/325)) ([e76cb1c](https://github.com/TheOpenDictionary/odict/commit/e76cb1c1ba1724528d7113c924d2b582494eb11b))

## [0.3.1](https://github.com/TheOpenDictionary/odict/compare/js-v0.3.0...js-v0.3.1) (2023-05-19)


### Bug Fixes

* **js:** compile FlatBuffer files ([1b2f9a2](https://github.com/TheOpenDictionary/odict/commit/1b2f9a25def93b954ada35d31db0dc4367fc4d4d))
* **js:** remove service upon exit/disconnect ([42669ce](https://github.com/TheOpenDictionary/odict/commit/42669ce4e7478d5acd69342dac70d9cebcbc4c4a))

## [0.3.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.2.0...js-v0.3.0) (2023-05-19)


### Features

* **js:** communicate to CLI via long-running process ([#316](https://github.com/TheOpenDictionary/odict/issues/316)) ([1547d40](https://github.com/TheOpenDictionary/odict/commit/1547d409af52763dac16ab7cb3e11b7cb89609e2))
* modularize Justfiles ([#187](https://github.com/TheOpenDictionary/odict/issues/187)) ([5b02037](https://github.com/TheOpenDictionary/odict/commit/5b0203772975518cb2428b36a7dbb7c050179937))


### Bug Fixes

* **dump:** remove unnecessary ODict dependencies ([a93c14c](https://github.com/TheOpenDictionary/odict/commit/a93c14cd108764c86ad2fa3b4c8fc721aaace0c7))
* remove commented code and print statement ([777a668](https://github.com/TheOpenDictionary/odict/commit/777a6686179bc0b5cdd294b2bcc9cf7e81364587))

## [0.2.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.1.1...js-v0.2.0) (2023-04-04)


### Features

* add lexicon() method to all SDKs ([#181](https://github.com/TheOpenDictionary/odict/issues/181)) ([bc6450f](https://github.com/TheOpenDictionary/odict/commit/bc6450fdbc2e9848e1376a6433aa6b759219ff1f))

## [0.1.1](https://github.com/TheOpenDictionary/odict/compare/js-v0.1.0...js-v0.1.1) (2023-02-13)


### Bug Fixes

* **js:** update TypeScript types ([dde8af5](https://github.com/TheOpenDictionary/odict/commit/dde8af56457e44ac1bee808f02ba18a640bb86ac))

## [0.1.0](https://github.com/TheOpenDictionary/odict/compare/js-v0.0.5...js-v0.1.0) (2023-02-01)


### Features

* add support for pretty-printing entries ([#62](https://github.com/TheOpenDictionary/odict/issues/62)) ([24d82bc](https://github.com/TheOpenDictionary/odict/commit/24d82bcf44b896347f4c9e9210cfe72bea82d978))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* **js:** added repository link ([a1c77e4](https://github.com/TheOpenDictionary/odict/commit/a1c77e48a1387afdbafdade58009627950095b45))


### Bug Fixes

* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* JavaScript fixes ([6fbd152](https://github.com/TheOpenDictionary/odict/commit/6fbd1528f63a55f6b2aded8f39b3ab155f62bb80))
* JavaScript fixes ([16ce875](https://github.com/TheOpenDictionary/odict/commit/16ce87517509c08c7081d5d24f0e82fcd3631793))
* **js:** disable term splitting by default ([f739ef0](https://github.com/TheOpenDictionary/odict/commit/f739ef0bc8ef4709b3fea52cc8b00486c547366d))
* **js:** fix prepublish ([73955ed](https://github.com/TheOpenDictionary/odict/commit/73955edf1cd85730661465c6d1859abc1a495db3))
* **js:** update Node configuration ([b0f3850](https://github.com/TheOpenDictionary/odict/commit/b0f385060cc45460d8ca38c699b00ad0cac9aa9e))
* **js:** update types ([0670089](https://github.com/TheOpenDictionary/odict/commit/0670089449ccc1601a8e6a08c41c8faffd981814))

## [0.0.5](https://github.com/TheOpenDictionary/odict/compare/theopendictionary-v0.0.4...theopendictionary-v0.0.5) (2023-01-05)


### Bug Fixes

* **js:** update types ([0670089](https://github.com/TheOpenDictionary/odict/commit/0670089449ccc1601a8e6a08c41c8faffd981814))

## [0.0.4](https://github.com/TheOpenDictionary/odict/compare/theopendictionary-v0.0.3...theopendictionary-v0.0.4) (2022-12-11)


### Features

* add support for pretty-printing entries ([#62](https://github.com/TheOpenDictionary/odict/issues/62)) ([24d82bc](https://github.com/TheOpenDictionary/odict/commit/24d82bcf44b896347f4c9e9210cfe72bea82d978))

## [0.0.3](https://github.com/TheOpenDictionary/odict/compare/theopendictionary-v0.0.2...theopendictionary-v0.0.3) (2022-11-12)


### Bug Fixes

* **js:** disable term splitting by default ([f739ef0](https://github.com/TheOpenDictionary/odict/commit/f739ef0bc8ef4709b3fea52cc8b00486c547366d))
* **js:** update Node configuration ([b0f3850](https://github.com/TheOpenDictionary/odict/commit/b0f385060cc45460d8ca38c699b00ad0cac9aa9e))

## [0.0.2](https://github.com/TheOpenDictionary/odict/compare/theopendictionary-v0.0.1...theopendictionary-v0.0.2) (2022-11-06)


### Features

* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* **js:** added repository link ([a1c77e4](https://github.com/TheOpenDictionary/odict/commit/a1c77e48a1387afdbafdade58009627950095b45))


### Bug Fixes

* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* JavaScript fixes ([6fbd152](https://github.com/TheOpenDictionary/odict/commit/6fbd1528f63a55f6b2aded8f39b3ab155f62bb80))
* JavaScript fixes ([16ce875](https://github.com/TheOpenDictionary/odict/commit/16ce87517509c08c7081d5d24f0e82fcd3631793))
* **js:** fix prepublish ([73955ed](https://github.com/TheOpenDictionary/odict/commit/73955edf1cd85730661465c6d1859abc1a495db3))
* **js:** update Node configuration ([b0f3850](https://github.com/TheOpenDictionary/odict/commit/b0f385060cc45460d8ca38c699b00ad0cac9aa9e))
