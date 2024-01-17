# Changelog

## [1.22.1](https://github.com/TheOpenDictionary/odict/compare/v1.22.0...v1.22.1) (2024-01-17)


### Bug Fixes

* **build:** Except wasm from GoReleaser config ([68df180](https://github.com/TheOpenDictionary/odict/commit/68df1802d3de47f02b7cb47fcbbe503d82a00880))
* **ci:** Fix merge conflict ([50f8bf8](https://github.com/TheOpenDictionary/odict/commit/50f8bf880bd27cc85e336101e85b7706d19dd6d5))
* **deps:** Update module github.com/blevesearch/bleve_index_api to v1.1.5 ([f2c7869](https://github.com/TheOpenDictionary/odict/commit/f2c7869f55621f3e093976f53384fc9a1919dde4))
* **deps:** Update module github.com/imdario/mergo to v1 ([#388](https://github.com/TheOpenDictionary/odict/issues/388)) ([de31bb5](https://github.com/TheOpenDictionary/odict/commit/de31bb52c3719fb687eb61f4e9d50455f992560e))

## [1.22.0](https://github.com/TheOpenDictionary/odict/compare/v1.21.0...v1.22.0) (2024-01-15)


### Features

* Add basic WASM support ([#394](https://github.com/TheOpenDictionary/odict/issues/394)) ([8df4e85](https://github.com/TheOpenDictionary/odict/commit/8df4e851c175dc5677d9580a29e9d38927f796a6))


### Bug Fixes

* **read:** Improve incompatibility error message ([39a7103](https://github.com/TheOpenDictionary/odict/commit/39a7103bbeb902dc380dd4963799556bd86ba85a))
* **sql:** Fix types.md to convert to string on sql dump ([#520](https://github.com/TheOpenDictionary/odict/issues/520)) ([f018e00](https://github.com/TheOpenDictionary/odict/commit/f018e00bcbb36a529adfefe57be9405cf593d9b2))
* **sql:** Important SQL fixes ([82538a0](https://github.com/TheOpenDictionary/odict/commit/82538a0e84ae3e5e0b91b1c3e15e063b56e09cbd))
* **wasm:** Change wasm package to pre-release ([d8540d8](https://github.com/TheOpenDictionary/odict/commit/d8540d848aefce9e6663989d3098edca15fbba22))
* **wasm:** Manually set version ([19d5691](https://github.com/TheOpenDictionary/odict/commit/19d56915d5e9b5e2b2dd9713b1cb5d34fd559374))

## [1.21.0](https://github.com/TheOpenDictionary/odict/compare/v1.20.0...v1.21.0) (2024-01-10)


### Features

* Add XSD generation ([#494](https://github.com/TheOpenDictionary/odict/issues/494)) ([bfe8ad4](https://github.com/TheOpenDictionary/odict/commit/bfe8ad47f1817c5d1a0733df421229207b7f6833))
* **cli:** Respect markdown setting when pretty printing ([7291d28](https://github.com/TheOpenDictionary/odict/commit/7291d280b85e68528053e29a6c1971d1f3c40a28))


### Bug Fixes

* **deps:** Update dependency com.google.guava:guava to v32.1.3-jre ([a49f0a6](https://github.com/TheOpenDictionary/odict/commit/a49f0a6157fd9ab7edaff713957de1a3d97ee9b1))
* **deps:** Update dependency com.google.guava:guava to v33 ([de5f639](https://github.com/TheOpenDictionary/odict/commit/de5f6392d096c0f96354f1965d5e11d41fffaaf8))
* **deps:** Update github.com/gomarkdown/markdown digest to 1d6d208 ([33630d3](https://github.com/TheOpenDictionary/odict/commit/33630d3a9017b695c3ac4f5ce1c3f487d3cd84f9))
* **deps:** Update github.com/gomarkdown/markdown digest to a660076 ([96df6cb](https://github.com/TheOpenDictionary/odict/commit/96df6cbc38d9280b5f7da48973d3a2f8cf39fd67))
* **deps:** Update module github.com/blevesearch/bleve_index_api to v1.1.0 ([e016381](https://github.com/TheOpenDictionary/odict/commit/e0163812b45daa535b42a5b7552b2af91cd71d53))
* **deps:** Update module github.com/blevesearch/bleve_index_api to v1.1.2 ([c9e0fd9](https://github.com/TheOpenDictionary/odict/commit/c9e0fd9c8e27b8fcea5166e79dff01055cae492a))
* **deps:** Update module github.com/blevesearch/bleve_index_api to v1.1.3 ([c355ce8](https://github.com/TheOpenDictionary/odict/commit/c355ce8b68744f91931ad7559b6e0078bf763633))
* **deps:** Update module github.com/blevesearch/bleve_index_api to v1.1.4 ([0e79b82](https://github.com/TheOpenDictionary/odict/commit/0e79b8252e4d0e113dfef53af8b7c908ae170d42))
* **deps:** Update module github.com/bokwoon95/sq to v0.4.4 ([ecfd7b1](https://github.com/TheOpenDictionary/odict/commit/ecfd7b15af0efa6633d335e83c37bed344f1739d))
* **deps:** Update module github.com/bokwoon95/sq to v0.4.5 ([f0c0321](https://github.com/TheOpenDictionary/odict/commit/f0c03217210c1869697ccef28edf4afe23a1578f))
* **deps:** Update module github.com/bokwoon95/sqddl to v0.4.9 ([4bc88a5](https://github.com/TheOpenDictionary/odict/commit/4bc88a5a7495a86674886b53e4face5b3b519eee))
* **deps:** Update module github.com/fatih/color to v1.16.0 ([b4c745d](https://github.com/TheOpenDictionary/odict/commit/b4c745db538da6cab3e2ab64c50b38d24aa93ba7))
* **deps:** Update module github.com/google/uuid to v1.4.0 ([9ebf6ce](https://github.com/TheOpenDictionary/odict/commit/9ebf6cedfe5d6abd1e649ea2345e584d2b7adb18))
* **deps:** Update module github.com/google/uuid to v1.5.0 ([186d585](https://github.com/TheOpenDictionary/odict/commit/186d58543030e56182bf2b8359140280bbec0b4d))
* **deps:** Update module github.com/samber/lo to v1.39.0 ([365b6cd](https://github.com/TheOpenDictionary/odict/commit/365b6cda03793f6920e24c5ac0ae1a678d1252eb))
* **deps:** Update module github.com/schollz/progressbar/v3 to v3.14.0 ([c727887](https://github.com/TheOpenDictionary/odict/commit/c7278877104f76c390e9e3773405625b23bf0e06))
* **deps:** Update module github.com/schollz/progressbar/v3 to v3.14.1 ([5e1346e](https://github.com/TheOpenDictionary/odict/commit/5e1346e084aa111fb868ec46ccf0292669ca5c74))
* **deps:** Update module github.com/urfave/cli/v2 to v2.26.0 ([4451fbb](https://github.com/TheOpenDictionary/odict/commit/4451fbbb61722c4d3c100ce106ba00e8af0c84a5))
* **deps:** Update module github.com/urfave/cli/v2 to v2.27.1 ([d8e0648](https://github.com/TheOpenDictionary/odict/commit/d8e0648b196bb2786d5335746abe558cf7ffd462))
* Fix XSD generation ([#495](https://github.com/TheOpenDictionary/odict/issues/495)) ([8dd4218](https://github.com/TheOpenDictionary/odict/commit/8dd42182cd9730eb7294d8f68468fd1f9b49a2cf))
* **pos:** Fix mismatch between buffer strings and POS tags ([#555](https://github.com/TheOpenDictionary/odict/issues/555)) ([fe190f1](https://github.com/TheOpenDictionary/odict/commit/fe190f1c983202b68f8a9cc1bc59ab5ead312eef))
* **README:** Update badge ([53dacbc](https://github.com/TheOpenDictionary/odict/commit/53dacbcb995f7136351a67d1444a72481ff7cc8e))
* **test:** Add test cleanup ([eaf2ded](https://github.com/TheOpenDictionary/odict/commit/eaf2ded8c5631512af4f0f7abde3a33fb3c44a95))
* Update lockfiles ([056ce84](https://github.com/TheOpenDictionary/odict/commit/056ce84877575fc92ff142c562be35e2fc513cc5))

## [1.20.0](https://github.com/TheOpenDictionary/odict/compare/v1.19.3...v1.20.0) (2023-09-28)


### Features

* Add option to output Markdown as plain-text ([#470](https://github.com/TheOpenDictionary/odict/issues/470)) ([71ca0d5](https://github.com/TheOpenDictionary/odict/commit/71ca0d5c6f22c660fdd51e098a66e2e1f4acf47c))


### Bug Fixes

* **build:** Update build-all command ([5686f95](https://github.com/TheOpenDictionary/odict/commit/5686f9599aa48c1b60d6f5fff1e883e82f97aeaa))

## [1.19.3](https://github.com/TheOpenDictionary/odict/compare/v1.19.2...v1.19.3) (2023-09-27)


### Bug Fixes

* **build:** Disable CGO ([90530f5](https://github.com/TheOpenDictionary/odict/commit/90530f5d75cea0f8e0bc945cdeeb60f39344bc59))

## [1.19.2](https://github.com/TheOpenDictionary/odict/compare/v1.19.1...v1.19.2) (2023-09-26)


### Bug Fixes

* Fix Goreleaser config ([127b253](https://github.com/TheOpenDictionary/odict/commit/127b2531f7f0df0ec6d3ebad200858172a120f30))

## [1.19.1](https://github.com/TheOpenDictionary/odict/compare/v1.19.0...v1.19.1) (2023-09-26)


### Bug Fixes

* **ci:** Fix GoReleaser? ([203031e](https://github.com/TheOpenDictionary/odict/commit/203031e5188af350cdeb3278577aec85c54618be))
* **ci:** Use GoReleaser action ([28852e3](https://github.com/TheOpenDictionary/odict/commit/28852e30316e2e124253a4ca6e85a8560b50ddcd))
* **goreleaser:** Add formula to Formula directory ([6fe51c9](https://github.com/TheOpenDictionary/odict/commit/6fe51c9926eb4803c1ea955aefacab13bab5d916))
* **goreleaser:** Don't include two binaries in archive ([ad7ec2c](https://github.com/TheOpenDictionary/odict/commit/ad7ec2c1ac84e8d15631dfec71ce13986ae93e6e))

## [1.19.0](https://github.com/TheOpenDictionary/odict/compare/v1.18.1...v1.19.0) (2023-09-26)


### Features

* Add Markdown support to description and value attributes ([#462](https://github.com/TheOpenDictionary/odict/issues/462)) ([5a2111f](https://github.com/TheOpenDictionary/odict/commit/5a2111f6aa2f4817028e2fa9c8fec0314569f360))
* Prebuild binaries ([#463](https://github.com/TheOpenDictionary/odict/issues/463)) ([123bfd7](https://github.com/TheOpenDictionary/odict/commit/123bfd7102be53438f74b7eb9a8542ebb2e2137b))


### Bug Fixes

* **cli:** Only show completed text if command did not error ([ada10e0](https://github.com/TheOpenDictionary/odict/commit/ada10e010d0d34c5c9a670fd57e053f25efe93ec))

## [1.18.1](https://github.com/TheOpenDictionary/odict/compare/v1.18.0...v1.18.1) (2023-09-23)


### Bug Fixes

* **ci:** Update release-please setup ([a372e48](https://github.com/TheOpenDictionary/odict/commit/a372e4826f043f5fa6abfa90162fc1f9aa665009))
* **deps:** Disable renovate upgrades of GH actions ([6a02948](https://github.com/TheOpenDictionary/odict/commit/6a02948a106b1129d22090219d4cea149dfa3def))
* **deps:** Update module github.com/bokwoon95/sq to v0.4.2 ([05af8cd](https://github.com/TheOpenDictionary/odict/commit/05af8cdf4c176af5a107142df5f83ea2ec4c8e70))
* **deps:** Update module github.com/bokwoon95/sq to v0.4.3 ([382f35d](https://github.com/TheOpenDictionary/odict/commit/382f35dbfa3655acfb05171a20f08e455058b224))
* **js:** Add ready check for service ([#458](https://github.com/TheOpenDictionary/odict/issues/458)) ([ed984ba](https://github.com/TheOpenDictionary/odict/commit/ed984bad21af2c57f8e8c421a8ffe81c1b3fb8fe))
* **js:** Make notes optional ([ee72c66](https://github.com/TheOpenDictionary/odict/commit/ee72c6666cfc37f740946b801dc68190a46b22e4))
* **print:** Add line break after etymologies ([e2cb252](https://github.com/TheOpenDictionary/odict/commit/e2cb2527d99827536fff3ef8e37128aa3f063d64))

## [1.18.0](https://github.com/TheOpenDictionary/odict/compare/v1.17.1...v1.18.0) (2023-09-18)


### Features

* **merge:** simplify merging logic ([#455](https://github.com/TheOpenDictionary/odict/issues/455)) ([2e0b218](https://github.com/TheOpenDictionary/odict/commit/2e0b2186543d065272a3996c3edef8421c427280))


### Bug Fixes

* **print:** add line break after printing notes ([329e788](https://github.com/TheOpenDictionary/odict/commit/329e788a71e094a9ebaa0af9386bfa739eb899ac))

## [1.17.1](https://github.com/TheOpenDictionary/odict/compare/v1.17.0...v1.17.1) (2023-09-17)


### Bug Fixes

* **cli:** remove ODict dependencies from go.mod ([aaca7d4](https://github.com/TheOpenDictionary/odict/commit/aaca7d4de126f72e2ab4100cedccd493ff6eff15))

## [1.17.0](https://github.com/TheOpenDictionary/odict/compare/v1.16.1...v1.17.0) (2023-09-17)


### Features

* **core:** add DefinitionNote type ([#450](https://github.com/TheOpenDictionary/odict/issues/450)) ([08bcac4](https://github.com/TheOpenDictionary/odict/commit/08bcac4b0710112166d6a43429fbb2f0c784c241))
* **js:** remove detectOpenHandles from test run ([2eed906](https://github.com/TheOpenDictionary/odict/commit/2eed906830a569a89b9f74d4a5058f71fde8dd10))


### Bug Fixes

* **deps:** update dependency com.google.guava:guava to v32.1.1-jre ([57633a4](https://github.com/TheOpenDictionary/odict/commit/57633a471cbc92a7c5c64ca497e53c5f8db78b8b))
* **deps:** update dependency com.google.guava:guava to v32.1.2-jre ([ee29572](https://github.com/TheOpenDictionary/odict/commit/ee2957265f48d7041a01dc238e555ad6aa6d27c8))
* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.10 ([4f8082d](https://github.com/TheOpenDictionary/odict/commit/4f8082d8a48933fc9a1dd8693eea57763e3ee70b))
* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.9 ([3e50eec](https://github.com/TheOpenDictionary/odict/commit/3e50eec8268e25a5f122c58e1802c6a97e70136b))
* **deps:** update module github.com/bokwoon95/sq to v0.4.0 ([5c7add6](https://github.com/TheOpenDictionary/odict/commit/5c7add66de99e0e0b8074fb16470a2e2db15f940))
* **deps:** update module github.com/bokwoon95/sq to v0.4.1 ([24ab925](https://github.com/TheOpenDictionary/odict/commit/24ab92554cced324a31eac266778c43b05caa87a))
* **deps:** update module github.com/bokwoon95/sqddl to v0.4.6 ([7694773](https://github.com/TheOpenDictionary/odict/commit/76947734f3a704d9970e40662e492ce0b36ac904))
* **deps:** update module github.com/google/uuid to v1.3.1 ([985f897](https://github.com/TheOpenDictionary/odict/commit/985f897345e924636976f7897039aa19a8153958))
* remove local dependencies ([de4805a](https://github.com/TheOpenDictionary/odict/commit/de4805a008d706326c3be317311c5092cd4ee43f))

## [1.16.1](https://github.com/TheOpenDictionary/odict/compare/v1.16.0...v1.16.1) (2023-06-29)


### Bug Fixes

* **deps:** update dependency com.google.guava:guava to v32.1.0-jre ([a72fdcf](https://github.com/TheOpenDictionary/odict/commit/a72fdcfb6752319e7560f61db484c7ed4190502f))
* fix ODict imports ([e31dcdd](https://github.com/TheOpenDictionary/odict/commit/e31dcddc11006e6b2895f2516c592d660101cf83))
* sync workspace ([a399eae](https://github.com/TheOpenDictionary/odict/commit/a399eaee8f8d12ee537ed68a87487c12c3387237))

## [1.16.0](https://github.com/TheOpenDictionary/odict/compare/v1.15.0...v1.16.0) (2023-06-28)


### Features

* add `serve` command ([#393](https://github.com/TheOpenDictionary/odict/issues/393)) ([cc7ef5c](https://github.com/TheOpenDictionary/odict/commit/cc7ef5c69811a4554e3b62bf492090de65ae86ea))
* add support for dictionary aliasing ([#391](https://github.com/TheOpenDictionary/odict/issues/391)) ([c6f1a04](https://github.com/TheOpenDictionary/odict/commit/c6f1a0495136817065007acc4414194f4e8445ad))

## [1.15.0](https://github.com/TheOpenDictionary/odict/compare/v1.14.1...v1.15.0) (2023-06-19)


### Features

* **docs:** update README ([7680cf5](https://github.com/TheOpenDictionary/odict/commit/7680cf5b6205b4210bd18c439025ef71238fd81c))
* **jvm:** add build script ([3eed477](https://github.com/TheOpenDictionary/odict/commit/3eed477faa5616a2ba4394297defe7ca8c10246c))


### Bug Fixes

* **deps:** update dependency com.google.guava:guava to v32.0.1-jre ([b1c9213](https://github.com/TheOpenDictionary/odict/commit/b1c921392dfa1d9df9803d3e8963bd5762af8e8a))
* **deps:** update module github.com/bokwoon95/sqddl to v0.4.4 ([a77e513](https://github.com/TheOpenDictionary/odict/commit/a77e51315bf34cad15e8e7e031a290c846f99eca))
* **deps:** update module github.com/bokwoon95/sqddl to v0.4.5 ([ffdab01](https://github.com/TheOpenDictionary/odict/commit/ffdab016f8bad669fdfa08ec14219627c5c8297c))
* **deps:** update module github.com/urfave/cli/v2 to v2.25.6 ([7c62e86](https://github.com/TheOpenDictionary/odict/commit/7c62e864b6fb61fee81cb819bbf5dd507add49af))
* **deps:** update module github.com/urfave/cli/v2 to v2.25.7 ([124f3be](https://github.com/TheOpenDictionary/odict/commit/124f3be491554d99c10c2facfe74f7e66310901d))
* **write:** fix default dictionary IDs ([#383](https://github.com/TheOpenDictionary/odict/issues/383)) ([9207e5a](https://github.com/TheOpenDictionary/odict/commit/9207e5a4bb05fc52c9ff8ed6f0c6e931c6dc150d))

## [1.14.1](https://github.com/TheOpenDictionary/odict/compare/v1.14.0...v1.14.1) (2023-05-31)


### Bug Fixes

* **js:** fixed PartOfSpeech map types ([725bba4](https://github.com/TheOpenDictionary/odict/commit/725bba46a5f31580ed3207b7bea2e302680b49fb))

## [1.14.0](https://github.com/TheOpenDictionary/odict/compare/v1.13.2...v1.14.0) (2023-05-31)


### Features

* **js:** add PartOfSpeech type ([8ee6e9a](https://github.com/TheOpenDictionary/odict/commit/8ee6e9a119445807af7e5fe4017e54e2fde0474a))
* **just:** update deps rule ([e22e47d](https://github.com/TheOpenDictionary/odict/commit/e22e47d19a36d011bd9d677192a8334f656925eb))


### Bug Fixes

* **cli:** fix issue where format option was ignored ([47fcf5a](https://github.com/TheOpenDictionary/odict/commit/47fcf5a745ce76cb81ca6735dc46721e52bb8d73))
* **deps:** update module github.com/stretchr/testify to v1.8.4 ([ffb1f44](https://github.com/TheOpenDictionary/odict/commit/ffb1f441ee06601330049a7dea89467946d7b92e))

## [1.13.2](https://github.com/TheOpenDictionary/odict/compare/v1.13.1...v1.13.2) (2023-05-29)


### Bug Fixes

* **deps:** update module github.com/bokwoon95/sq to v0.3.5 ([c424360](https://github.com/TheOpenDictionary/odict/commit/c424360ed8bd9dfb2989a542a45b0c560b4be204))
* **deps:** update module github.com/urfave/cli/v2 to v2.25.5 ([640fac8](https://github.com/TheOpenDictionary/odict/commit/640fac815821be72bdcfba7e70055228e7c9b05c))
* **js:** fix broken compilation ([741c236](https://github.com/TheOpenDictionary/odict/commit/741c236d537cb1eaacc728ae4bcddad5664fee42))

## [1.13.1](https://github.com/TheOpenDictionary/odict/compare/v1.13.0...v1.13.1) (2023-05-29)


### Bug Fixes

* attempt to fix tests ([eda588a](https://github.com/TheOpenDictionary/odict/commit/eda588ae0e6b9c971df3bee4c2d598d9a95a22e8))
* **js:** fix test ([0487c39](https://github.com/TheOpenDictionary/odict/commit/0487c39c173fdd52c35c5f959ddc4c2d23cd40af))
* **js:** restart process if there are unexpected exits ([388176c](https://github.com/TheOpenDictionary/odict/commit/388176c60145ba1ba02ca4dc3a54c8fb3a688b68))

## [1.13.0](https://github.com/TheOpenDictionary/odict/compare/v1.12.1...v1.13.0) (2023-05-28)


### Features

* use IPC library for inter-process communication ([#344](https://github.com/TheOpenDictionary/odict/issues/344)) ([fc23307](https://github.com/TheOpenDictionary/odict/commit/fc2330771dcdbfe6358fa09f15f1cdf86bd4b9ab))


### Bug Fixes

* **deps:** update module github.com/urfave/cli/v2 to v2.25.4 ([d026d4e](https://github.com/TheOpenDictionary/odict/commit/d026d4e06ae6250980b88745c5bcd04b53591547))

## [1.12.1](https://github.com/TheOpenDictionary/odict/compare/v1.12.0...v1.12.1) (2023-05-28)


### Bug Fixes

* **js:** remove log statements ([810e6f7](https://github.com/TheOpenDictionary/odict/commit/810e6f7b0a772cc0d58797f02ff4c962a9e98bcc))

## [1.12.0](https://github.com/TheOpenDictionary/odict/compare/v1.11.2...v1.12.0) (2023-05-28)


### Features

* **js:** add part of speech enum to js package ([0fdd9bc](https://github.com/TheOpenDictionary/odict/commit/0fdd9bc073a20769a6bb80939290a8981d526920))
* make goimports call more specific ([4a1a0db](https://github.com/TheOpenDictionary/odict/commit/4a1a0dbd1d005683dbb2f63abd733ec26b0064c5))


### Bug Fixes

* **deps:** update dependency com.google.guava:guava to v32 ([6c7b5ad](https://github.com/TheOpenDictionary/odict/commit/6c7b5ad3a638cade5ec7cfa9a0c7f8d12cb817d5))
* **deps:** update dependency flatbuffers to v23.5.26 ([60b6baf](https://github.com/TheOpenDictionary/odict/commit/60b6baf51caaf8d835a5bc8d2c1c052926224364))
* **deps:** update module github.com/bokwoon95/sq to v0.3.0 ([fec4115](https://github.com/TheOpenDictionary/odict/commit/fec4115d7150de48a74a1b10ceb1028588ccba31))
* **deps:** update module github.com/bokwoon95/sq to v0.3.3 ([cb3b29d](https://github.com/TheOpenDictionary/odict/commit/cb3b29d9068b6b7419e895afe8fe001c75449a07))
* **deps:** update module github.com/bokwoon95/sq to v0.3.4 ([cc1479a](https://github.com/TheOpenDictionary/odict/commit/cc1479a31a3d48df89f21557c606beec51b0d7cc))
* **deps:** update module github.com/google/flatbuffers to v23.5.26+incompatible ([15b2ec6](https://github.com/TheOpenDictionary/odict/commit/15b2ec69deab6e7da97a331607f785fb11d69785))
* **deps:** update module github.com/imdario/mergo to v0.3.16 ([c8bf550](https://github.com/TheOpenDictionary/odict/commit/c8bf5501d0802f1b0e1e26de6541faf649f0454a))

## [1.11.2](https://github.com/TheOpenDictionary/odict/compare/v1.11.1...v1.11.2) (2023-05-20)


### Bug Fixes

* **js:** bug fixes with service implementation ([#325](https://github.com/TheOpenDictionary/odict/issues/325)) ([e76cb1c](https://github.com/TheOpenDictionary/odict/commit/e76cb1c1ba1724528d7113c924d2b582494eb11b))

## [1.11.1](https://github.com/TheOpenDictionary/odict/compare/v1.11.0...v1.11.1) (2023-05-19)


### Bug Fixes

* **js:** compile FlatBuffer files ([1b2f9a2](https://github.com/TheOpenDictionary/odict/commit/1b2f9a25def93b954ada35d31db0dc4367fc4d4d))
* **js:** remove service upon exit/disconnect ([42669ce](https://github.com/TheOpenDictionary/odict/commit/42669ce4e7478d5acd69342dac70d9cebcbc4c4a))

## [1.11.0](https://github.com/TheOpenDictionary/odict/compare/v1.10.0...v1.11.0) (2023-05-19)


### Features

* **js:** communicate to CLI via long-running process ([#316](https://github.com/TheOpenDictionary/odict/issues/316)) ([1547d40](https://github.com/TheOpenDictionary/odict/commit/1547d409af52763dac16ab7cb3e11b7cb89609e2))
* modularize Justfiles ([#187](https://github.com/TheOpenDictionary/odict/issues/187)) ([5b02037](https://github.com/TheOpenDictionary/odict/commit/5b0203772975518cb2428b36a7dbb7c050179937))
* use new flatc asdf plugin ([#219](https://github.com/TheOpenDictionary/odict/issues/219)) ([ff7d32e](https://github.com/TheOpenDictionary/odict/commit/ff7d32ec66f3f8cc618c999957d5671c4a1b4e26))


### Bug Fixes

* **cli:** remove ODict dependencies from go.mod ([48bb08c](https://github.com/TheOpenDictionary/odict/commit/48bb08cd856c9e9d2ad5d85c550332b7af3eaebc))
* **deps:** update dependency com.squareup.moshi:moshi to v1.15.0 ([b1ac27f](https://github.com/TheOpenDictionary/odict/commit/b1ac27f508a8966419308d0eec64f07fd082b11b))
* **deps:** update dependency com.squareup.moshi:moshi-kotlin to v1.15.0 ([d402118](https://github.com/TheOpenDictionary/odict/commit/d402118f14ba5f2b5cf2b1d18521ec30225555e7))
* **deps:** update dependency com.squareup.moshi:moshi-kotlin-codegen to v1.15.0 ([c2c69b5](https://github.com/TheOpenDictionary/odict/commit/c2c69b5c82167a40b767af5c7bce32b43359cf2e))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 00c7142 ([0d97001](https://github.com/TheOpenDictionary/odict/commit/0d970019eb7341b119f99ced9fc9abf036dbe1b1))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 10225b4 ([28bcbe0](https://github.com/TheOpenDictionary/odict/commit/28bcbe05e9873a2d3d42edf04519ad8aaa66560d))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 1ad3de2 ([7ca5f14](https://github.com/TheOpenDictionary/odict/commit/7ca5f14e8ecbd68d4f584bba8b7f383fd8d0a819))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 1e2f5ca ([198da61](https://github.com/TheOpenDictionary/odict/commit/198da6144127c26efadb854203a7b9705420ecec))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 34f1918 ([34f301b](https://github.com/TheOpenDictionary/odict/commit/34f301b047fa09ff9e2068eaf3a1275349b2cd56))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 34f301b ([c2a794b](https://github.com/TheOpenDictionary/odict/commit/c2a794bf5bfc6e4e81a144ef3c47545df871e37a))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 3728308 ([a861425](https://github.com/TheOpenDictionary/odict/commit/a861425d731ee726430c2eef6290c5fb390e4929))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 46dd9c6 ([aa0c16c](https://github.com/TheOpenDictionary/odict/commit/aa0c16c2055842b81e99c64d3afbac5534a54882))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 56631f8 ([34f1918](https://github.com/TheOpenDictionary/odict/commit/34f191815d299f69aea01c0abdd275eb06b117b5))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 591c113 ([69a42a8](https://github.com/TheOpenDictionary/odict/commit/69a42a85d6796da050c965f6f8774f2745d4290c))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 649a122 ([e6c8843](https://github.com/TheOpenDictionary/odict/commit/e6c8843f5cfd5925b82c8ccfcbd7e3252c6a5923))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 694535f ([e401c65](https://github.com/TheOpenDictionary/odict/commit/e401c65dc19e7ec3ec1190521b88e67f6ad372f6))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 69a42a8 ([cec338e](https://github.com/TheOpenDictionary/odict/commit/cec338e59cf53952358ec7c205037196d8e7f638))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 797c93b ([ddbb069](https://github.com/TheOpenDictionary/odict/commit/ddbb069bf0aea07b2a3a012f5480d140e69aef8c))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 7ca5f14 ([591c113](https://github.com/TheOpenDictionary/odict/commit/591c113af1bb678cf65f34c4b64ad5af51e7a47b))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 7d653ed ([c606b2f](https://github.com/TheOpenDictionary/odict/commit/c606b2f5586d13580dc487d59ec46b0197074027))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 8822574 ([d560c35](https://github.com/TheOpenDictionary/odict/commit/d560c35519e9ff352e289d56320514a3606a9efd))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 8997a90 ([d5b3bad](https://github.com/TheOpenDictionary/odict/commit/d5b3bad1b80dedba571f3609eb7f04e10691652c))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 8aa5eb3 ([8fff55e](https://github.com/TheOpenDictionary/odict/commit/8fff55e7a29d65727a2cd7569b0b3629db61ab57))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 8fff55e ([649a122](https://github.com/TheOpenDictionary/odict/commit/649a12230b86d57f0329be55b49e94a5e21a1107))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 90bb672 ([9dfd521](https://github.com/TheOpenDictionary/odict/commit/9dfd5210eec55abcbd80a7fda9b0dc248efd5463))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 95914aa ([694535f](https://github.com/TheOpenDictionary/odict/commit/694535fb0f5cd66be7c9867e2af0bd0230142b2b))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 9c7d6d2 ([3728308](https://github.com/TheOpenDictionary/odict/commit/372830813e29b4ccc70639f810dcfef6255a2a0b))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to 9dfd521 ([2092a48](https://github.com/TheOpenDictionary/odict/commit/2092a4833f2d4130fe049a41de8db57ca3125562))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to a69237e ([797c93b](https://github.com/TheOpenDictionary/odict/commit/797c93b61d4e931417d2a871cfd0c3513dc4b7a6))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to aa0c16c ([a69237e](https://github.com/TheOpenDictionary/odict/commit/a69237e20273822a0464bb78074a514556f42844))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to c2a794b ([1ad3de2](https://github.com/TheOpenDictionary/odict/commit/1ad3de2eb81a142475cd6c750ce9d2c5a4c2760c))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to ccad2a9 ([95914aa](https://github.com/TheOpenDictionary/odict/commit/95914aa340e73e811d92fc0763c34d81eaa34394))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to cec338e ([8822574](https://github.com/TheOpenDictionary/odict/commit/88225741600ce524ec0550949cc939532d6ac7b8))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to d560c35 ([a31983a](https://github.com/TheOpenDictionary/odict/commit/a31983ad30c3270f35ae95d81e301f055f9f1c7f))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to d5b3bad ([7d653ed](https://github.com/TheOpenDictionary/odict/commit/7d653ed4d97157ba4344d8dad36f03c6c921cca2))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to ddbb069 ([e9e8699](https://github.com/TheOpenDictionary/odict/commit/e9e86992814f848be9f923e17ebc83bf1beb30de))
* **deps:** update github.com/theopendictionary/odict/lib/core digest to e9e8699 ([ccad2a9](https://github.com/TheOpenDictionary/odict/commit/ccad2a987b65752df98a4bf00f8f6d44b4c03cc3))
* **deps:** update github.com/theopendictionary/odict/lib/test digest to e9e84bb ([1e2f5ca](https://github.com/TheOpenDictionary/odict/commit/1e2f5ca9b992d4e3025a300de29a8ebbe4d64139))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 029caad ([0a62e63](https://github.com/TheOpenDictionary/odict/commit/0a62e63d142665567dcf63222e164ed9f8a16fae))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 0ae6ba7 ([87abc85](https://github.com/TheOpenDictionary/odict/commit/87abc854edba929467bb607dba792df080d178b7))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 37aaab1 ([774155c](https://github.com/TheOpenDictionary/odict/commit/774155c434dedc2a75e502f20312e6e5210cce5b))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 4007012 ([f57b945](https://github.com/TheOpenDictionary/odict/commit/f57b945f35d45fc818f9510c8236bf6c898ab4b3))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 4b605b5 ([ae6714d](https://github.com/TheOpenDictionary/odict/commit/ae6714d3ec3e4bffc47803654c32248225c59bce))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 5e8c480 ([007c161](https://github.com/TheOpenDictionary/odict/commit/007c161e7220b0a0b5aa016846aa2b8f6bf9d366))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 6410897 ([cd5468b](https://github.com/TheOpenDictionary/odict/commit/cd5468b06b265836f5da7198cd96696dec322433))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 8202945 ([6410897](https://github.com/TheOpenDictionary/odict/commit/64108978fdee555fd9fae052361115a18d2b42c3))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 82c261b ([85a3960](https://github.com/TheOpenDictionary/odict/commit/85a3960cde196d6adc18d627e96864b79933bab6))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 85a3960 ([e87b377](https://github.com/TheOpenDictionary/odict/commit/e87b3779ab473f7295e4622463f85e41ac319103))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 87abc85 ([029caad](https://github.com/TheOpenDictionary/odict/commit/029caad2a29e50c7596ea2f38fb61aafcae39c48))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 9255a7f ([f55aa92](https://github.com/TheOpenDictionary/odict/commit/f55aa92ff6434853eee3d725cca204f366108f1c))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 9b681a2 ([8202945](https://github.com/TheOpenDictionary/odict/commit/8202945d9338ba2426f63546191462bae61490dc))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to 9c40380 ([eb9852b](https://github.com/TheOpenDictionary/odict/commit/eb9852ba65abf344c3d103de06b4f3a010cb8e27))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to ae23994 ([37aaab1](https://github.com/TheOpenDictionary/odict/commit/37aaab13961bdc47f2b46935e3ef32a614d81d79))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to ae6714d ([fb1ba4e](https://github.com/TheOpenDictionary/odict/commit/fb1ba4e0d0d6332ae27d2719e3bebd1e257dc36b))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to b1ac27f ([4407a79](https://github.com/TheOpenDictionary/odict/commit/4407a7997dceaedc9567380fa84704b426a16219))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to b6c8d91 ([ae23994](https://github.com/TheOpenDictionary/odict/commit/ae23994fecdc7c62756a40beb7204d3df5ca8248))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to c86b8e3 ([4b605b5](https://github.com/TheOpenDictionary/odict/commit/4b605b552a2c18344ff7488e9d2e24e691ce4c9d))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to cd5468b ([d7ede00](https://github.com/TheOpenDictionary/odict/commit/d7ede002b409f56f2d9d9a40107295d30b130595))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to e87b377 ([9b681a2](https://github.com/TheOpenDictionary/odict/commit/9b681a26d19931a80b073e5f4f3560fb05ac6099))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to eb9852b ([1882c5d](https://github.com/TheOpenDictionary/odict/commit/1882c5d6d6b51526730bfcd35a134ec068705b45))
* **deps:** update github.com/theopendictionary/odict/lib/utils digest to fb1ba4e ([7ca5437](https://github.com/TheOpenDictionary/odict/commit/7ca543712ea11fb36d3f490572d64d9dadb085a9))
* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.8 ([00c7142](https://github.com/TheOpenDictionary/odict/commit/00c71422d60af27d0f388fd595430acd0239bd2f))
* **deps:** update module github.com/bokwoon95/sq to v0.2.9 ([90bb672](https://github.com/TheOpenDictionary/odict/commit/90bb6728f881755c7f96d041371b114a19204cd4))
* **deps:** update module github.com/bokwoon95/sqddl to v0.4.2 ([e2923a2](https://github.com/TheOpenDictionary/odict/commit/e2923a2254f7e7ad2d01c0f7759faececc085456))
* **deps:** update module github.com/bokwoon95/sqddl to v0.4.3 ([91909c5](https://github.com/TheOpenDictionary/odict/commit/91909c5031a11aca9cdd6f9cf4673ba8be50cd9a))
* **deps:** update module github.com/google/flatbuffers to v23.5.9+incompatible ([4007012](https://github.com/TheOpenDictionary/odict/commit/4007012c064ec88f79f90452a40caa425b5fa918))
* **deps:** update module github.com/stretchr/testify to v1.8.3 ([7c0fb56](https://github.com/TheOpenDictionary/odict/commit/7c0fb566bca656fdbec0e6e3b0a2ad1ec999d263))
* **deps:** update module github.com/urfave/cli/v2 to v2.25.2 ([9c7d6d2](https://github.com/TheOpenDictionary/odict/commit/9c7d6d2a7a0dc054dbffce5c3e2fb8390788683c))
* **deps:** update module github.com/urfave/cli/v2 to v2.25.3 ([3862651](https://github.com/TheOpenDictionary/odict/commit/386265132971035f1d8114aa8dd9277f95582099))
* **dump:** remove unnecessary ODict dependencies ([a93c14c](https://github.com/TheOpenDictionary/odict/commit/a93c14cd108764c86ad2fa3b4c8fc721aaace0c7))
* prevent upgrades of ODict packages ([bcef3cb](https://github.com/TheOpenDictionary/odict/commit/bcef3cb6c28b2ae06c2ec01a3ffd56ef5ee394ba))
* **python:** remove log statement ([cf78da9](https://github.com/TheOpenDictionary/odict/commit/cf78da92ca0e0a8cf164f5ca976ac18975f98f88))
* remove commented code and print statement ([777a668](https://github.com/TheOpenDictionary/odict/commit/777a6686179bc0b5cdd294b2bcc9cf7e81364587))
* **search:** remove ODict dependencies ([178a6a7](https://github.com/TheOpenDictionary/odict/commit/178a6a7c5d97b1ac530f03984571cc178f4142c5))

## [1.10.0](https://github.com/TheOpenDictionary/odict/compare/v1.9.1...v1.10.0) (2023-04-04)


### Features

* add lexicon() method to all SDKs ([#181](https://github.com/TheOpenDictionary/odict/issues/181)) ([bc6450f](https://github.com/TheOpenDictionary/odict/commit/bc6450fdbc2e9848e1376a6433aa6b759219ff1f))
* support SQL output format ([#122](https://github.com/TheOpenDictionary/odict/issues/122)) ([7dd99c5](https://github.com/TheOpenDictionary/odict/commit/7dd99c53120e35a7b2d3f824530670cdf545ce54))


### Bug Fixes

* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.7 ([a1324ba](https://github.com/TheOpenDictionary/odict/commit/a1324bad86ba82bdfc4f2e30d0776eccab34a1b1))
* **deps:** update module github.com/bokwoon95/sq to v0.2.7 ([bc63ba2](https://github.com/TheOpenDictionary/odict/commit/bc63ba23d20a729f03bd6f7e991c8ead7c41da7d))
* **deps:** update module github.com/bokwoon95/sq to v0.2.8 ([5309a68](https://github.com/TheOpenDictionary/odict/commit/5309a68c5ecb9f982acade77f72fe51abdab616a))
* **deps:** update module github.com/bokwoon95/sqddl to v0.3.13 ([4f378dc](https://github.com/TheOpenDictionary/odict/commit/4f378dc8e0629715b6b05f174d6481ca718902f5))
* **deps:** update module github.com/bokwoon95/sqddl to v0.4.1 ([c81b6ff](https://github.com/TheOpenDictionary/odict/commit/c81b6ff5f1f9d595984fa7ae4b9cfa811b4e2833))
* **deps:** update module github.com/fatih/color to v1.15.0 ([d13ad4d](https://github.com/TheOpenDictionary/odict/commit/d13ad4d85050ef95025a84e3d2d26cbb1f0e6906))
* **deps:** update module github.com/google/flatbuffers to v23.3.3+incompatible ([36c0e00](https://github.com/TheOpenDictionary/odict/commit/36c0e0080785c5d60ebeea79b1031af0b0bf0072))
* **deps:** update module github.com/imdario/mergo to v0.3.14 ([5bc1850](https://github.com/TheOpenDictionary/odict/commit/5bc1850d95b6df7519342dcbc8e562b3c6d2f475))
* **deps:** update module github.com/imdario/mergo to v0.3.15 ([7559455](https://github.com/TheOpenDictionary/odict/commit/755945541db95a1b8ce2e4f84576de6ae537e758))
* **deps:** update module github.com/schollz/progressbar/v3 to v3.13.1 ([68ea5e8](https://github.com/TheOpenDictionary/odict/commit/68ea5e83cf31ea4f9cbaebd95b72e4236cfd4d26))
* **deps:** update module github.com/stretchr/testify to v1.8.2 ([d9ae95a](https://github.com/TheOpenDictionary/odict/commit/d9ae95a22175f58c37a9ce60faf1f97ba832b393))
* **deps:** update module github.com/urfave/cli/v2 to v2.24.4 ([fde31cb](https://github.com/TheOpenDictionary/odict/commit/fde31cbcea8827a20e812bfb5a57b1c07ea3fcfc))
* **deps:** update module github.com/urfave/cli/v2 to v2.25.0 ([5b5b684](https://github.com/TheOpenDictionary/odict/commit/5b5b684115c8f323e8fc4a1e2b8143ee1159ac50))
* **deps:** update module github.com/urfave/cli/v2 to v2.25.1 ([9287596](https://github.com/TheOpenDictionary/odict/commit/92875964844119bc9d66a72e9d047c5f2375eec2))

## [1.9.1](https://github.com/TheOpenDictionary/odict/compare/v1.9.0...v1.9.1) (2023-02-13)


### Bug Fixes

* **deps:** update module github.com/urfave/cli/v2 to v2.24.3 ([e8bd9bf](https://github.com/TheOpenDictionary/odict/commit/e8bd9bfa9c35e78db0c6c0abd24b359381d2f76c))
* **go:** always show description for groups ([97e8a62](https://github.com/TheOpenDictionary/odict/commit/97e8a62153a392189bdc57ae57a0bf7837790f60))
* **js:** update TypeScript types ([dde8af5](https://github.com/TheOpenDictionary/odict/commit/dde8af56457e44ac1bee808f02ba18a640bb86ac))
* **lookup:** fix follow flag for fallback lookups ([fb0748d](https://github.com/TheOpenDictionary/odict/commit/fb0748d47aeee4e4ff5c13899a36e5742b008af2))
* remove component for cli ([f3618e2](https://github.com/TheOpenDictionary/odict/commit/f3618e2daac35d3cac6ecd741f91b2f375e0c150))

## [1.9.0](https://github.com/TheOpenDictionary/odict/compare/cli-v1.8.5...cli-v1.9.0) (2023-02-01)


### Features

* 1.5.0 refactor ([#30](https://github.com/TheOpenDictionary/odict/issues/30)) ([193519f](https://github.com/TheOpenDictionary/odict/commit/193519f62e014c496b975c22aeca32a0b9222e84))
* add example sentences to schema ([#32](https://github.com/TheOpenDictionary/odict/issues/32)) ([298e71f](https://github.com/TheOpenDictionary/odict/commit/298e71f92f68f2d9c2d09131a69373a116a3a6da))
* add LICENSE file ([3fba965](https://github.com/TheOpenDictionary/odict/commit/3fba965f77977474fc4a0e32c6b21272f7bbd147))
* add pronunciation attribute ([#92](https://github.com/TheOpenDictionary/odict/issues/92)) ([89bbe01](https://github.com/TheOpenDictionary/odict/commit/89bbe019c77e5e1bdb70261c88052f99067568b1))
* add support for entry aliasing ([#61](https://github.com/TheOpenDictionary/odict/issues/61)) ([077d947](https://github.com/TheOpenDictionary/odict/commit/077d94735a8d81f747015fc36045ff265ed1b736))
* add support for Japanese-specific POS tags ([#66](https://github.com/TheOpenDictionary/odict/issues/66)) ([2dd7171](https://github.com/TheOpenDictionary/odict/commit/2dd717101323d11481c9bd87e153f25feef3569f))
* add support for pretty-printing entries ([#62](https://github.com/TheOpenDictionary/odict/issues/62)) ([24d82bc](https://github.com/TheOpenDictionary/odict/commit/24d82bcf44b896347f4c9e9210cfe72bea82d978))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* added "merge" and "dump" commands ([#10](https://github.com/TheOpenDictionary/odict/issues/10)) ([ca9423c](https://github.com/TheOpenDictionary/odict/commit/ca9423caf24be490f0be00201c336732cc313e40))
* added basic e2e tests and updated godoc ([#11](https://github.com/TheOpenDictionary/odict/issues/11)) ([80ca45c](https://github.com/TheOpenDictionary/odict/commit/80ca45c3af148fdc7b68f3851ee91614b84081ea))
* added CI pipeline ([45c31a8](https://github.com/TheOpenDictionary/odict/commit/45c31a8d4faa0cc320c4f4af40173c6c3499e682))
* added force indexing to Java and added ODICT_INDEX_DIR env variable ([62b2d68](https://github.com/TheOpenDictionary/odict/commit/62b2d6876ce8a34d2f6d0114308e06a3b4bd34e4))
* added in-repo docs ([#22](https://github.com/TheOpenDictionary/odict/issues/22)) ([b64550d](https://github.com/TheOpenDictionary/odict/commit/b64550d3c37902bde3b22e0c690174218e7994d1))
* added index option ([bf9a6b7](https://github.com/TheOpenDictionary/odict/commit/bf9a6b743008bbf843a72c9185e1c52c73aaccbf))
* added Java library + moved to Bazel monorepo ([#13](https://github.com/TheOpenDictionary/odict/issues/13)) ([fdb8304](https://github.com/TheOpenDictionary/odict/commit/fdb83040bab3414ad2d02daa76681dba731371f2))
* added lookup command ([22d2549](https://github.com/TheOpenDictionary/odict/commit/22d25495ab7618cf80d31d2a5cb380f80139d0ff))
* added Python extension ([#14](https://github.com/TheOpenDictionary/odict/issues/14)) ([5235b64](https://github.com/TheOpenDictionary/odict/commit/5235b64a0d2ca82a42662162034d9dbff5ccab81))
* attempt to setup Release Please ([6640e94](https://github.com/TheOpenDictionary/odict/commit/6640e94cc480d12cec04f3a95659d67230813880))
* **ci:** add component names to release-please ([7bba8ff](https://github.com/TheOpenDictionary/odict/commit/7bba8ffed043cb99c460e3cd7e0ec1c070a7a877))
* **cli:** add lexicon command ([#58](https://github.com/TheOpenDictionary/odict/issues/58)) ([cb40db1](https://github.com/TheOpenDictionary/odict/commit/cb40db116865e79fc5faf0538755c494bc789462))
* **cli:** added -i flag to search command ([c08aebc](https://github.com/TheOpenDictionary/odict/commit/c08aebc06474fcd9e56c8075193d21695a9dc035))
* created FUNDING.yml ([aa70c6f](https://github.com/TheOpenDictionary/odict/commit/aa70c6fe006af7379a8da64c262aa33b95dc336a))
* expand POS tags ([1a12dcc](https://github.com/TheOpenDictionary/odict/commit/1a12dccab0c29da959909b36480992ea98a35e6a))
* extended Java and Python bindings to hold base64 dictionary in-memory to prevent constant I/O ([#15](https://github.com/TheOpenDictionary/odict/issues/15)) ([9c5c251](https://github.com/TheOpenDictionary/odict/commit/9c5c251e48a46a375807f8b476374ed209b4c316))
* fix pretty-printing and disable case insensitivity ([aef0f41](https://github.com/TheOpenDictionary/odict/commit/aef0f41564afb99b1f692b5299c96425d7e302fc))
* **go:** add `split` command ([#31](https://github.com/TheOpenDictionary/odict/issues/31)) ([71814fc](https://github.com/TheOpenDictionary/odict/commit/71814fce27fda86fddb19b9907097e8dc530fba3))
* **java:** added toJSON method to Entry ([d566821](https://github.com/TheOpenDictionary/odict/commit/d566821f311a33676aa7a0dc46835036d2d63025))
* **java:** updated Java implementation to use more efficient Bleve search ([#19](https://github.com/TheOpenDictionary/odict/issues/19)) ([23d465e](https://github.com/TheOpenDictionary/odict/commit/23d465ef6a79f3d0eb6320a413c8284271c79f42))
* **js:** added repository link ([a1c77e4](https://github.com/TheOpenDictionary/odict/commit/a1c77e48a1387afdbafdade58009627950095b45))
* **jvm:** replaced Klaxon in favor of Moshi in `lookup()` function ([#44](https://github.com/TheOpenDictionary/odict/issues/44)) ([d960814](https://github.com/TheOpenDictionary/odict/commit/d960814cfe3ea06285af1dfe135e4e9799461eaa))
* Ported entire codebase to Go for stability + scalability ([#8](https://github.com/TheOpenDictionary/odict/issues/8)) ([ebc652f](https://github.com/TheOpenDictionary/odict/commit/ebc652f19d0403e32c0c3c1ecd742b125abd2584))
* ported Java lookups to native code to increase performance ([#17](https://github.com/TheOpenDictionary/odict/issues/17)) ([086a063](https://github.com/TheOpenDictionary/odict/commit/086a063be133f5a0aee9f67523342d06c612eb08))
* removed docs directory ([d6bdd8a](https://github.com/TheOpenDictionary/odict/commit/d6bdd8ab24e50217cf13c98e690fbaeed50bd204))
* **renovate:** enable automerge and updateLockFiles ([a1d96ad](https://github.com/TheOpenDictionary/odict/commit/a1d96ad8350e564fd7f9174d46cf52085ce76a86))
* setting up Maven deployment ([#24](https://github.com/TheOpenDictionary/odict/issues/24)) ([5984beb](https://github.com/TheOpenDictionary/odict/commit/5984beb01f43525d2681151b244e96788f879cc7))
* updated code to export C functions ([bb8163f](https://github.com/TheOpenDictionary/odict/commit/bb8163fed2b60610bf3d7807729b8f1a2bdfad04))


### Bug Fixes

* [BREAKING] updated content length size to be 8 bytes instead of 4 ([639dbab](https://github.com/TheOpenDictionary/odict/commit/639dbab4feac15c4f69049bfb9b7bcfecaf68b47))
* added back library target ([069903e](https://github.com/TheOpenDictionary/odict/commit/069903ef96e63dbc15275cad09e534a7fa3978b2))
* added bridge archive as data dep ([f5d44c9](https://github.com/TheOpenDictionary/odict/commit/f5d44c9faba58846ae51043327faf15f41ec40f0))
* attempt to fix various memory leaks ([#16](https://github.com/TheOpenDictionary/odict/issues/16)) ([bb33fa3](https://github.com/TheOpenDictionary/odict/commit/bb33fa3b5917c0eca798d3c577d8fc617acc83b2))
* **bazel:** referenced patch by repo name ([6d2c80c](https://github.com/TheOpenDictionary/odict/commit/6d2c80caba2d16501792776ab7defaa4d3ac65d6))
* **bazel:** updated to use workspace name ([8252ac4](https://github.com/TheOpenDictionary/odict/commit/8252ac45b4794bf799664efe0665f0f60245245a))
* **bridge:** set lookup to use exact search ([2c783e2](https://github.com/TheOpenDictionary/odict/commit/2c783e2a4d99788300249699a5d5149527f6ccc4))
* **build:** updated dependencies ([f6e4fe6](https://github.com/TheOpenDictionary/odict/commit/f6e4fe6e96223c4a3a489e24b0da5c11f2c31f69))
* changed vellum dependency to not rely on git ([4f52d28](https://github.com/TheOpenDictionary/odict/commit/4f52d28378e29bb016aa7c5e0e9d23f32eadcba7))
* changed vellum git URL ([eb5ce8f](https://github.com/TheOpenDictionary/odict/commit/eb5ce8fd4235371820f56d9857536c722dddc610))
* **ci:** fixed actions ([62c0064](https://github.com/TheOpenDictionary/odict/commit/62c00640d194230b13c438131598a36e229ef9b1))
* **ci:** update release-please config ([eceac85](https://github.com/TheOpenDictionary/odict/commit/eceac85af3e236aaa042dd61ade102601e48564c))
* **ci:** updated release-please to only publish for released packages ([7e2e083](https://github.com/TheOpenDictionary/odict/commit/7e2e0831cc4b171047aea100c78eff77e82f1c03))
* **cli:** write lexicon to stdout ([461725c](https://github.com/TheOpenDictionary/odict/commit/461725cb920d5ba497fa4492eba3d6146cc84ce0))
* consolidate jvm_rules_external ([030a1d9](https://github.com/TheOpenDictionary/odict/commit/030a1d99037e01d49cfd991165d01559b2dfc061))
* corrected module name ([6eca61d](https://github.com/TheOpenDictionary/odict/commit/6eca61d3b12d68496ee0d1072d571db8285ab0ad))
* corrected Python dll path ([60c80fd](https://github.com/TheOpenDictionary/odict/commit/60c80fdae1c6e3d826e7e6a608e4458cca067b46))
* corrected searching feature ([0df307e](https://github.com/TheOpenDictionary/odict/commit/0df307ebe05e9db080a7c336eec964acc407b3b1))
* **deps:** update dependency com.google.guava:guava to v31.1-jre ([61f9ece](https://github.com/TheOpenDictionary/odict/commit/61f9eceed4dc3b962c5209919c6e0d2818e3ce74))
* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.5 ([#36](https://github.com/TheOpenDictionary/odict/issues/36)) ([3d4f20d](https://github.com/TheOpenDictionary/odict/commit/3d4f20daf7833ecd0d6315c9e92dd052baed0262))
* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.6 ([bec0d8c](https://github.com/TheOpenDictionary/odict/commit/bec0d8cdf326f13c2e69c8ce3dacd99e8b3625c6))
* **deps:** update module github.com/fatih/color to v1.14.0 ([1384993](https://github.com/TheOpenDictionary/odict/commit/138499397cbd743e2a753fefd11bc003ee7b8426))
* **deps:** update module github.com/fatih/color to v1.14.1 ([ccc9ffe](https://github.com/TheOpenDictionary/odict/commit/ccc9ffe1592cb8f85d4e566f66a762704e04ea34))
* **deps:** update module github.com/google/flatbuffers to v22.12.6+incompatible ([6287eb4](https://github.com/TheOpenDictionary/odict/commit/6287eb4f37846250e4a516edd710d59278516a64))
* **deps:** update module github.com/google/flatbuffers to v23 ([#85](https://github.com/TheOpenDictionary/odict/issues/85)) ([73f38fd](https://github.com/TheOpenDictionary/odict/commit/73f38fda3f9787408630e7ba4a2b06b08b964d8a))
* **deps:** update module github.com/schollz/progressbar/v3 to v3.12.2 ([3962131](https://github.com/TheOpenDictionary/odict/commit/39621314bced6b6ee86eaef1026c287a2ca1752d))
* **deps:** update module github.com/schollz/progressbar/v3 to v3.13.0 ([a840651](https://github.com/TheOpenDictionary/odict/commit/a840651a5f8156b96018158c061a9ec0f384d198))
* **deps:** update module github.com/stretchr/testify to v1.8.1 ([5deb242](https://github.com/TheOpenDictionary/odict/commit/5deb242b4e9da01a76b1ba972d9d9b206bb6a83b))
* **deps:** update module github.com/urfave/cli/v2 to v2.24.1 ([890daac](https://github.com/TheOpenDictionary/odict/commit/890daaca104e55f515446a0084140e7552781d0c))
* **deps:** update module github.com/urfave/cli/v2 to v2.24.2 ([692296a](https://github.com/TheOpenDictionary/odict/commit/692296a0524fd073a7bd2f0a6a5fe5cce70a38a8))
* **docs:** change XML object to have &lt;dictionary&gt; name ([#23](https://github.com/TheOpenDictionary/odict/issues/23)) ([e02bd23](https://github.com/TheOpenDictionary/odict/commit/e02bd23953d5617ff697ca938b033695b0d35eb5))
* don't use lowercase entry keys ([c57d01e](https://github.com/TheOpenDictionary/odict/commit/c57d01e6655a5e517c811c11b02456e7ae2e48ee))
* downgraded Bazel rules to fix compilation ([2b306c2](https://github.com/TheOpenDictionary/odict/commit/2b306c22f6c5cd6beef635bea50adb22c865435f))
* drop cc from archive import ([d64597a](https://github.com/TheOpenDictionary/odict/commit/d64597a59787af5b0c95216c9d75b29124128bf9))
* fix tests ([8bbf73d](https://github.com/TheOpenDictionary/odict/commit/8bbf73d556d65d5ffb82669769cf4cf15e38e805))
* fixed README links ([a76ba11](https://github.com/TheOpenDictionary/odict/commit/a76ba11d9a5204d7e9c5bb100a7582d0657f1f19))
* fixes ([a1e229e](https://github.com/TheOpenDictionary/odict/commit/a1e229e863e72ea42aadcecb9abe7218bd4c14e9))
* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* Go dependency fixes ([3980033](https://github.com/TheOpenDictionary/odict/commit/39800332c1102a3bbcfa16ec8987952a4f4d1ea3))
* go fixes and added jitpack.yml ([69c28fc](https://github.com/TheOpenDictionary/odict/commit/69c28fc663cc985b36b70207add1a87aeccbc819))
* **go:** added case insensitivity to indexing ([d24ab2b](https://github.com/TheOpenDictionary/odict/commit/d24ab2b1dc598855ab5bc5b0fdb05102f0f59609))
* **go:** fix issue where undefined pos attributes would not default to Unknown ([#82](https://github.com/TheOpenDictionary/odict/issues/82)) ([675e63f](https://github.com/TheOpenDictionary/odict/commit/675e63f2a02540623e8bddaead4bdf43facea7a3))
* **go:** fix issue where usages required a valid part-of-speech ([cf5163a](https://github.com/TheOpenDictionary/odict/commit/cf5163a5683e1c07761951c816bb550e1ee7608f))
* improved indexing and added indexing as default lookup method ([#20](https://github.com/TheOpenDictionary/odict/issues/20)) ([03e9e2f](https://github.com/TheOpenDictionary/odict/commit/03e9e2f1229813b4ec3082aae0bfa61abb0cdb68))
* **java:** added archive as direct data dependency ([30d7c05](https://github.com/TheOpenDictionary/odict/commit/30d7c057b14474e4e03588cf062385853c272e43))
* **java:** added unique maven_install name to avoid conflicts ([93fcfee](https://github.com/TheOpenDictionary/odict/commit/93fcfee206ccff7e72ba125376daee151dba5359))
* **java:** changed java binary to java library ([b03706b](https://github.com/TheOpenDictionary/odict/commit/b03706bdc1dc3c9c96d382a3120d08a2e2f282c4))
* **java:** fixed incorrect bJNI boolean type causing unforced indexing to fail ([8627806](https://github.com/TheOpenDictionary/odict/commit/8627806ff8e8eebdd7c02961cb5d0cf0b5dc35c4))
* **java:** moved shared library into JAR ([05b3208](https://github.com/TheOpenDictionary/odict/commit/05b3208a85b78952a49f37c8163063132f0b7dd2))
* **java:** removed debugging print statement ([444e9a3](https://github.com/TheOpenDictionary/odict/commit/444e9a3ff29560d6798e5b63c22e4ff0e90c4dde))
* **java:** removed print statements ([357c25b](https://github.com/TheOpenDictionary/odict/commit/357c25b8365f48fb6d004e80841955abe4437e38))
* **java:** replaced Java lookups with lookupByKey instead of Bleve ([#21](https://github.com/TheOpenDictionary/odict/issues/21)) ([5717b9d](https://github.com/TheOpenDictionary/odict/commit/5717b9ddeb76e0610eab4ffd1da01e4d84f0e93d))
* JavaScript fixes ([6fbd152](https://github.com/TheOpenDictionary/odict/commit/6fbd1528f63a55f6b2aded8f39b3ab155f62bb80))
* JavaScript fixes ([16ce875](https://github.com/TheOpenDictionary/odict/commit/16ce87517509c08c7081d5d24f0e82fcd3631793))
* **js:** disable term splitting by default ([f739ef0](https://github.com/TheOpenDictionary/odict/commit/f739ef0bc8ef4709b3fea52cc8b00486c547366d))
* **js:** fix prepublish ([73955ed](https://github.com/TheOpenDictionary/odict/commit/73955edf1cd85730661465c6d1859abc1a495db3))
* **js:** update Node configuration ([b0f3850](https://github.com/TheOpenDictionary/odict/commit/b0f385060cc45460d8ca38c699b00ad0cac9aa9e))
* **js:** update types ([0670089](https://github.com/TheOpenDictionary/odict/commit/0670089449ccc1601a8e6a08c41c8faffd981814))
* **make:** fix publish command ([178a6da](https://github.com/TheOpenDictionary/odict/commit/178a6da9215ec7ae9e6736a30ebf6903d230438d))
* **paper:** typo ([e12a70b](https://github.com/TheOpenDictionary/odict/commit/e12a70bb707c82dc7b02172edb97c990bfed676d))
* **pos:** add back missing adjective ([550e8cc](https://github.com/TheOpenDictionary/odict/commit/550e8cc54ff5084e4cc74529103e3bd7b6d909ca))
* **python:** added custom library resolution ([313593b](https://github.com/TheOpenDictionary/odict/commit/313593bbfb7033acb8a2e6de34c5b34638114a96))
* **python:** fix python build ([3df34ac](https://github.com/TheOpenDictionary/odict/commit/3df34ac46051e6e618e890e157527cffad2d44bd))
* **python:** fixed ODict path ([669cace](https://github.com/TheOpenDictionary/odict/commit/669cace5c290e6c7e84ce3d1c4e752e726873798))
* **python:** remove log ([8f43725](https://github.com/TheOpenDictionary/odict/commit/8f4372541b17327788f8e4680e608f45e6ab92ac))
* **python:** updated Python target visibility ([474856d](https://github.com/TheOpenDictionary/odict/commit/474856d4a7bec8c107f0fea89fb9926fc296cc7c))
* readd cc ([95981df](https://github.com/TheOpenDictionary/odict/commit/95981df7a6199a5528e274e165b6378eb5a7aa7e))
* remove log ([3074eb0](https://github.com/TheOpenDictionary/odict/commit/3074eb0b2cfab990506fbc694e24665417925421))
* remove Make from asdf ([35a2735](https://github.com/TheOpenDictionary/odict/commit/35a27350bcef1f5790c63a17bcc11a6fdeb65236))
* removed build cache ([30ff4ef](https://github.com/TheOpenDictionary/odict/commit/30ff4ef1a6ea954de08105153a410b459e6652d8))
* removed Java export target for now ([474e10a](https://github.com/TheOpenDictionary/odict/commit/474e10a46843a4100dbe4bd63e88312ffbb995c3))
* removed jitpack.yml ([128eb8b](https://github.com/TheOpenDictionary/odict/commit/128eb8bb9beb5a57d1ec26ce6a54a16807979906))
* removed unneeded patch files and flatbuffer rules ([ca4f58e](https://github.com/TheOpenDictionary/odict/commit/ca4f58e0fa95d6dba4efb33582d69f038a56dabc))
* store all keys as lowercase ([b405776](https://github.com/TheOpenDictionary/odict/commit/b40577620f406ae98d4e559b2f483327ca74dff2))
* updated for Bazel 5 compatibility ([#25](https://github.com/TheOpenDictionary/odict/issues/25)) ([0deda2a](https://github.com/TheOpenDictionary/odict/commit/0deda2acc58eeced13929404a2c98304ddc41a66))
* various fixes/refactor/improvements ([#12](https://github.com/TheOpenDictionary/odict/issues/12)) ([0ea85db](https://github.com/TheOpenDictionary/odict/commit/0ea85db744efd8de8e48bb2321f1200193263cde))

## [1.8.5](https://github.com/TheOpenDictionary/odict/compare/v1.8.4...v1.8.5) (2023-01-07)


### Bug Fixes

* **ci:** update release-please config ([eceac85](https://github.com/TheOpenDictionary/odict/commit/eceac85af3e236aaa042dd61ade102601e48564c))
* **deps:** update module github.com/google/flatbuffers to v22.12.6+incompatible ([6287eb4](https://github.com/TheOpenDictionary/odict/commit/6287eb4f37846250e4a516edd710d59278516a64))
* **deps:** update module github.com/schollz/progressbar/v3 to v3.13.0 ([a840651](https://github.com/TheOpenDictionary/odict/commit/a840651a5f8156b96018158c061a9ec0f384d198))
* store all keys as lowercase ([b405776](https://github.com/TheOpenDictionary/odict/commit/b40577620f406ae98d4e559b2f483327ca74dff2))

## [1.8.4](https://github.com/TheOpenDictionary/odict/compare/v1.8.3...v1.8.4) (2023-01-05)


### Bug Fixes

* **js:** update types ([0670089](https://github.com/TheOpenDictionary/odict/commit/0670089449ccc1601a8e6a08c41c8faffd981814))

## [1.8.3](https://github.com/TheOpenDictionary/odict/compare/v1.8.2...v1.8.3) (2023-01-04)


### Bug Fixes

* **deps:** update dependency com.google.guava:guava to v31.1-jre ([61f9ece](https://github.com/TheOpenDictionary/odict/commit/61f9eceed4dc3b962c5209919c6e0d2818e3ce74))
* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.6 ([bec0d8c](https://github.com/TheOpenDictionary/odict/commit/bec0d8cdf326f13c2e69c8ce3dacd99e8b3625c6))
* **deps:** update module github.com/schollz/progressbar/v3 to v3.12.2 ([3962131](https://github.com/TheOpenDictionary/odict/commit/39621314bced6b6ee86eaef1026c287a2ca1752d))
* **deps:** update module github.com/stretchr/testify to v1.8.1 ([5deb242](https://github.com/TheOpenDictionary/odict/commit/5deb242b4e9da01a76b1ba972d9d9b206bb6a83b))

## [1.8.2](https://github.com/TheOpenDictionary/odict/compare/v1.8.1...v1.8.2) (2022-12-29)


### Bug Fixes

* **go:** fix issue where usages required a valid part-of-speech ([cf5163a](https://github.com/TheOpenDictionary/odict/commit/cf5163a5683e1c07761951c816bb550e1ee7608f))
* **python:** remove log ([8f43725](https://github.com/TheOpenDictionary/odict/commit/8f4372541b17327788f8e4680e608f45e6ab92ac))

## [1.8.1](https://github.com/TheOpenDictionary/odict/compare/v1.8.0...v1.8.1) (2022-12-17)


### Bug Fixes

* **pos:** add back missing adjective ([550e8cc](https://github.com/TheOpenDictionary/odict/commit/550e8cc54ff5084e4cc74529103e3bd7b6d909ca))

## [1.8.0](https://github.com/TheOpenDictionary/odict/compare/v1.7.0...v1.8.0) (2022-12-16)


### Features

* add support for Japanese-specific POS tags ([#66](https://github.com/TheOpenDictionary/odict/issues/66)) ([2dd7171](https://github.com/TheOpenDictionary/odict/commit/2dd717101323d11481c9bd87e153f25feef3569f))
* created FUNDING.yml ([aa70c6f](https://github.com/TheOpenDictionary/odict/commit/aa70c6fe006af7379a8da64c262aa33b95dc336a))

## [1.7.0](https://github.com/TheOpenDictionary/odict/compare/v1.6.1...v1.7.0) (2022-12-11)


### Features

* add support for entry aliasing ([#61](https://github.com/TheOpenDictionary/odict/issues/61)) ([077d947](https://github.com/TheOpenDictionary/odict/commit/077d94735a8d81f747015fc36045ff265ed1b736))
* add support for pretty-printing entries ([#62](https://github.com/TheOpenDictionary/odict/issues/62)) ([24d82bc](https://github.com/TheOpenDictionary/odict/commit/24d82bcf44b896347f4c9e9210cfe72bea82d978))
* **cli:** add lexicon command ([#58](https://github.com/TheOpenDictionary/odict/issues/58)) ([cb40db1](https://github.com/TheOpenDictionary/odict/commit/cb40db116865e79fc5faf0538755c494bc789462))
* **renovate:** enable automerge and updateLockFiles ([a1d96ad](https://github.com/TheOpenDictionary/odict/commit/a1d96ad8350e564fd7f9174d46cf52085ce76a86))


### Bug Fixes

* **cli:** write lexicon to stdout ([461725c](https://github.com/TheOpenDictionary/odict/commit/461725cb920d5ba497fa4492eba3d6146cc84ce0))
* **deps:** update module github.com/blevesearch/bleve/v2 to v2.3.5 ([#36](https://github.com/TheOpenDictionary/odict/issues/36)) ([3d4f20d](https://github.com/TheOpenDictionary/odict/commit/3d4f20daf7833ecd0d6315c9e92dd052baed0262))

## [1.6.1](https://github.com/TheOpenDictionary/odict/compare/v1.6.0...v1.6.1) (2022-11-12)


### Bug Fixes

* **ci:** updated release-please to only publish for released packages ([7e2e083](https://github.com/TheOpenDictionary/odict/commit/7e2e0831cc4b171047aea100c78eff77e82f1c03))
* don't use lowercase entry keys ([c57d01e](https://github.com/TheOpenDictionary/odict/commit/c57d01e6655a5e517c811c11b02456e7ae2e48ee))
* fix tests ([8bbf73d](https://github.com/TheOpenDictionary/odict/commit/8bbf73d556d65d5ffb82669769cf4cf15e38e805))
* **js:** disable term splitting by default ([f739ef0](https://github.com/TheOpenDictionary/odict/commit/f739ef0bc8ef4709b3fea52cc8b00486c547366d))
* **js:** update Node configuration ([b0f3850](https://github.com/TheOpenDictionary/odict/commit/b0f385060cc45460d8ca38c699b00ad0cac9aa9e))
* **make:** fix publish command ([178a6da](https://github.com/TheOpenDictionary/odict/commit/178a6da9215ec7ae9e6736a30ebf6903d230438d))
* **python:** fix python build ([3df34ac](https://github.com/TheOpenDictionary/odict/commit/3df34ac46051e6e618e890e157527cffad2d44bd))

## [1.6.0](https://github.com/TheOpenDictionary/odict/compare/v1.5.0...v1.6.0) (2022-11-06)


### Features

* 1.5.0 refactor ([#30](https://github.com/TheOpenDictionary/odict/issues/30)) ([193519f](https://github.com/TheOpenDictionary/odict/commit/193519f62e014c496b975c22aeca32a0b9222e84))
* add example sentences to schema ([#32](https://github.com/TheOpenDictionary/odict/issues/32)) ([298e71f](https://github.com/TheOpenDictionary/odict/commit/298e71f92f68f2d9c2d09131a69373a116a3a6da))
* add LICENSE file ([3fba965](https://github.com/TheOpenDictionary/odict/commit/3fba965f77977474fc4a0e32c6b21272f7bbd147))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* added "merge" and "dump" commands ([#10](https://github.com/TheOpenDictionary/odict/issues/10)) ([ca9423c](https://github.com/TheOpenDictionary/odict/commit/ca9423caf24be490f0be00201c336732cc313e40))
* added basic e2e tests and updated godoc ([#11](https://github.com/TheOpenDictionary/odict/issues/11)) ([80ca45c](https://github.com/TheOpenDictionary/odict/commit/80ca45c3af148fdc7b68f3851ee91614b84081ea))
* added CI pipeline ([45c31a8](https://github.com/TheOpenDictionary/odict/commit/45c31a8d4faa0cc320c4f4af40173c6c3499e682))
* added force indexing to Java and added ODICT_INDEX_DIR env variable ([62b2d68](https://github.com/TheOpenDictionary/odict/commit/62b2d6876ce8a34d2f6d0114308e06a3b4bd34e4))
* added in-repo docs ([#22](https://github.com/TheOpenDictionary/odict/issues/22)) ([b64550d](https://github.com/TheOpenDictionary/odict/commit/b64550d3c37902bde3b22e0c690174218e7994d1))
* added index option ([bf9a6b7](https://github.com/TheOpenDictionary/odict/commit/bf9a6b743008bbf843a72c9185e1c52c73aaccbf))
* added Java library + moved to Bazel monorepo ([#13](https://github.com/TheOpenDictionary/odict/issues/13)) ([fdb8304](https://github.com/TheOpenDictionary/odict/commit/fdb83040bab3414ad2d02daa76681dba731371f2))
* added lookup command ([22d2549](https://github.com/TheOpenDictionary/odict/commit/22d25495ab7618cf80d31d2a5cb380f80139d0ff))
* added Python extension ([#14](https://github.com/TheOpenDictionary/odict/issues/14)) ([5235b64](https://github.com/TheOpenDictionary/odict/commit/5235b64a0d2ca82a42662162034d9dbff5ccab81))
* attempt to setup Release Please ([6640e94](https://github.com/TheOpenDictionary/odict/commit/6640e94cc480d12cec04f3a95659d67230813880))
* **cli:** added -i flag to search command ([c08aebc](https://github.com/TheOpenDictionary/odict/commit/c08aebc06474fcd9e56c8075193d21695a9dc035))
* extended Java and Python bindings to hold base64 dictionary in-memory to prevent constant I/O ([#15](https://github.com/TheOpenDictionary/odict/issues/15)) ([9c5c251](https://github.com/TheOpenDictionary/odict/commit/9c5c251e48a46a375807f8b476374ed209b4c316))
* **go:** add `split` command ([#31](https://github.com/TheOpenDictionary/odict/issues/31)) ([71814fc](https://github.com/TheOpenDictionary/odict/commit/71814fce27fda86fddb19b9907097e8dc530fba3))
* **java:** added toJSON method to Entry ([d566821](https://github.com/TheOpenDictionary/odict/commit/d566821f311a33676aa7a0dc46835036d2d63025))
* **java:** updated Java implementation to use more efficient Bleve search ([#19](https://github.com/TheOpenDictionary/odict/issues/19)) ([23d465e](https://github.com/TheOpenDictionary/odict/commit/23d465ef6a79f3d0eb6320a413c8284271c79f42))
* **js:** added repository link ([a1c77e4](https://github.com/TheOpenDictionary/odict/commit/a1c77e48a1387afdbafdade58009627950095b45))
* **jvm:** replaced Klaxon in favor of Moshi in `lookup()` function ([#44](https://github.com/TheOpenDictionary/odict/issues/44)) ([d960814](https://github.com/TheOpenDictionary/odict/commit/d960814cfe3ea06285af1dfe135e4e9799461eaa))
* Ported entire codebase to Go for stability + scalability ([#8](https://github.com/TheOpenDictionary/odict/issues/8)) ([ebc652f](https://github.com/TheOpenDictionary/odict/commit/ebc652f19d0403e32c0c3c1ecd742b125abd2584))
* ported Java lookups to native code to increase performance ([#17](https://github.com/TheOpenDictionary/odict/issues/17)) ([086a063](https://github.com/TheOpenDictionary/odict/commit/086a063be133f5a0aee9f67523342d06c612eb08))
* removed docs directory ([d6bdd8a](https://github.com/TheOpenDictionary/odict/commit/d6bdd8ab24e50217cf13c98e690fbaeed50bd204))
* setting up Maven deployment ([#24](https://github.com/TheOpenDictionary/odict/issues/24)) ([5984beb](https://github.com/TheOpenDictionary/odict/commit/5984beb01f43525d2681151b244e96788f879cc7))
* updated code to export C functions ([bb8163f](https://github.com/TheOpenDictionary/odict/commit/bb8163fed2b60610bf3d7807729b8f1a2bdfad04))


### Bug Fixes

* [BREAKING] updated content length size to be 8 bytes instead of 4 ([639dbab](https://github.com/TheOpenDictionary/odict/commit/639dbab4feac15c4f69049bfb9b7bcfecaf68b47))
* added back library target ([069903e](https://github.com/TheOpenDictionary/odict/commit/069903ef96e63dbc15275cad09e534a7fa3978b2))
* added bridge archive as data dep ([f5d44c9](https://github.com/TheOpenDictionary/odict/commit/f5d44c9faba58846ae51043327faf15f41ec40f0))
* attempt to fix various memory leaks ([#16](https://github.com/TheOpenDictionary/odict/issues/16)) ([bb33fa3](https://github.com/TheOpenDictionary/odict/commit/bb33fa3b5917c0eca798d3c577d8fc617acc83b2))
* **bazel:** referenced patch by repo name ([6d2c80c](https://github.com/TheOpenDictionary/odict/commit/6d2c80caba2d16501792776ab7defaa4d3ac65d6))
* **bazel:** updated to use workspace name ([8252ac4](https://github.com/TheOpenDictionary/odict/commit/8252ac45b4794bf799664efe0665f0f60245245a))
* **bridge:** set lookup to use exact search ([2c783e2](https://github.com/TheOpenDictionary/odict/commit/2c783e2a4d99788300249699a5d5149527f6ccc4))
* **build:** updated dependencies ([f6e4fe6](https://github.com/TheOpenDictionary/odict/commit/f6e4fe6e96223c4a3a489e24b0da5c11f2c31f69))
* changed vellum dependency to not rely on git ([4f52d28](https://github.com/TheOpenDictionary/odict/commit/4f52d28378e29bb016aa7c5e0e9d23f32eadcba7))
* changed vellum git URL ([eb5ce8f](https://github.com/TheOpenDictionary/odict/commit/eb5ce8fd4235371820f56d9857536c722dddc610))
* **ci:** fixed actions ([62c0064](https://github.com/TheOpenDictionary/odict/commit/62c00640d194230b13c438131598a36e229ef9b1))
* consolidate jvm_rules_external ([030a1d9](https://github.com/TheOpenDictionary/odict/commit/030a1d99037e01d49cfd991165d01559b2dfc061))
* corrected module name ([6eca61d](https://github.com/TheOpenDictionary/odict/commit/6eca61d3b12d68496ee0d1072d571db8285ab0ad))
* corrected Python dll path ([60c80fd](https://github.com/TheOpenDictionary/odict/commit/60c80fdae1c6e3d826e7e6a608e4458cca067b46))
* corrected searching feature ([0df307e](https://github.com/TheOpenDictionary/odict/commit/0df307ebe05e9db080a7c336eec964acc407b3b1))
* **docs:** change XML object to have &lt;dictionary&gt; name ([#23](https://github.com/TheOpenDictionary/odict/issues/23)) ([e02bd23](https://github.com/TheOpenDictionary/odict/commit/e02bd23953d5617ff697ca938b033695b0d35eb5))
* don't use lowercase entry keys ([c57d01e](https://github.com/TheOpenDictionary/odict/commit/c57d01e6655a5e517c811c11b02456e7ae2e48ee))
* downgraded Bazel rules to fix compilation ([2b306c2](https://github.com/TheOpenDictionary/odict/commit/2b306c22f6c5cd6beef635bea50adb22c865435f))
* drop cc from archive import ([d64597a](https://github.com/TheOpenDictionary/odict/commit/d64597a59787af5b0c95216c9d75b29124128bf9))
* fixed README links ([a76ba11](https://github.com/TheOpenDictionary/odict/commit/a76ba11d9a5204d7e9c5bb100a7582d0657f1f19))
* fixes ([a1e229e](https://github.com/TheOpenDictionary/odict/commit/a1e229e863e72ea42aadcecb9abe7218bd4c14e9))
* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* Go dependency fixes ([3980033](https://github.com/TheOpenDictionary/odict/commit/39800332c1102a3bbcfa16ec8987952a4f4d1ea3))
* go fixes and added jitpack.yml ([69c28fc](https://github.com/TheOpenDictionary/odict/commit/69c28fc663cc985b36b70207add1a87aeccbc819))
* **go:** added case insensitivity to indexing ([d24ab2b](https://github.com/TheOpenDictionary/odict/commit/d24ab2b1dc598855ab5bc5b0fdb05102f0f59609))
* improved indexing and added indexing as default lookup method ([#20](https://github.com/TheOpenDictionary/odict/issues/20)) ([03e9e2f](https://github.com/TheOpenDictionary/odict/commit/03e9e2f1229813b4ec3082aae0bfa61abb0cdb68))
* **java:** added archive as direct data dependency ([30d7c05](https://github.com/TheOpenDictionary/odict/commit/30d7c057b14474e4e03588cf062385853c272e43))
* **java:** added unique maven_install name to avoid conflicts ([93fcfee](https://github.com/TheOpenDictionary/odict/commit/93fcfee206ccff7e72ba125376daee151dba5359))
* **java:** changed java binary to java library ([b03706b](https://github.com/TheOpenDictionary/odict/commit/b03706bdc1dc3c9c96d382a3120d08a2e2f282c4))
* **java:** fixed incorrect bJNI boolean type causing unforced indexing to fail ([8627806](https://github.com/TheOpenDictionary/odict/commit/8627806ff8e8eebdd7c02961cb5d0cf0b5dc35c4))
* **java:** moved shared library into JAR ([05b3208](https://github.com/TheOpenDictionary/odict/commit/05b3208a85b78952a49f37c8163063132f0b7dd2))
* **java:** removed debugging print statement ([444e9a3](https://github.com/TheOpenDictionary/odict/commit/444e9a3ff29560d6798e5b63c22e4ff0e90c4dde))
* **java:** removed print statements ([357c25b](https://github.com/TheOpenDictionary/odict/commit/357c25b8365f48fb6d004e80841955abe4437e38))
* **java:** replaced Java lookups with lookupByKey instead of Bleve ([#21](https://github.com/TheOpenDictionary/odict/issues/21)) ([5717b9d](https://github.com/TheOpenDictionary/odict/commit/5717b9ddeb76e0610eab4ffd1da01e4d84f0e93d))
* JavaScript fixes ([6fbd152](https://github.com/TheOpenDictionary/odict/commit/6fbd1528f63a55f6b2aded8f39b3ab155f62bb80))
* JavaScript fixes ([16ce875](https://github.com/TheOpenDictionary/odict/commit/16ce87517509c08c7081d5d24f0e82fcd3631793))
* **js:** fix prepublish ([73955ed](https://github.com/TheOpenDictionary/odict/commit/73955edf1cd85730661465c6d1859abc1a495db3))
* **js:** update Node configuration ([b0f3850](https://github.com/TheOpenDictionary/odict/commit/b0f385060cc45460d8ca38c699b00ad0cac9aa9e))
* **make:** fix publish command ([178a6da](https://github.com/TheOpenDictionary/odict/commit/178a6da9215ec7ae9e6736a30ebf6903d230438d))
* **python:** added custom library resolution ([313593b](https://github.com/TheOpenDictionary/odict/commit/313593bbfb7033acb8a2e6de34c5b34638114a96))
* **python:** fixed ODict path ([669cace](https://github.com/TheOpenDictionary/odict/commit/669cace5c290e6c7e84ce3d1c4e752e726873798))
* **python:** updated Python target visibility ([474856d](https://github.com/TheOpenDictionary/odict/commit/474856d4a7bec8c107f0fea89fb9926fc296cc7c))
* readd cc ([95981df](https://github.com/TheOpenDictionary/odict/commit/95981df7a6199a5528e274e165b6378eb5a7aa7e))
* remove log ([3074eb0](https://github.com/TheOpenDictionary/odict/commit/3074eb0b2cfab990506fbc694e24665417925421))
* remove Make from asdf ([35a2735](https://github.com/TheOpenDictionary/odict/commit/35a27350bcef1f5790c63a17bcc11a6fdeb65236))
* removed build cache ([30ff4ef](https://github.com/TheOpenDictionary/odict/commit/30ff4ef1a6ea954de08105153a410b459e6652d8))
* removed Java export target for now ([474e10a](https://github.com/TheOpenDictionary/odict/commit/474e10a46843a4100dbe4bd63e88312ffbb995c3))
* removed jitpack.yml ([128eb8b](https://github.com/TheOpenDictionary/odict/commit/128eb8bb9beb5a57d1ec26ce6a54a16807979906))
* removed unneeded patch files and flatbuffer rules ([ca4f58e](https://github.com/TheOpenDictionary/odict/commit/ca4f58e0fa95d6dba4efb33582d69f038a56dabc))
* updated for Bazel 5 compatibility ([#25](https://github.com/TheOpenDictionary/odict/issues/25)) ([0deda2a](https://github.com/TheOpenDictionary/odict/commit/0deda2acc58eeced13929404a2c98304ddc41a66))
* various fixes/refactor/improvements ([#12](https://github.com/TheOpenDictionary/odict/issues/12)) ([0ea85db](https://github.com/TheOpenDictionary/odict/commit/0ea85db744efd8de8e48bb2321f1200193263cde))
