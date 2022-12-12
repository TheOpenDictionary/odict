# Changelog

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
