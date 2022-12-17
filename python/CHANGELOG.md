# Changelog

## [0.0.9](https://github.com/TheOpenDictionary/odict/compare/v0.0.8...v0.0.9) (2022-12-17)


### Features

* 1.5.0 refactor ([#30](https://github.com/TheOpenDictionary/odict/issues/30)) ([193519f](https://github.com/TheOpenDictionary/odict/commit/193519f62e014c496b975c22aeca32a0b9222e84))
* add support for entry aliasing ([#61](https://github.com/TheOpenDictionary/odict/issues/61)) ([077d947](https://github.com/TheOpenDictionary/odict/commit/077d94735a8d81f747015fc36045ff265ed1b736))
* add support for Japanese-specific POS tags ([#66](https://github.com/TheOpenDictionary/odict/issues/66)) ([2dd7171](https://github.com/TheOpenDictionary/odict/commit/2dd717101323d11481c9bd87e153f25feef3569f))
* add support for pretty-printing entries ([#62](https://github.com/TheOpenDictionary/odict/issues/62)) ([24d82bc](https://github.com/TheOpenDictionary/odict/commit/24d82bcf44b896347f4c9e9210cfe72bea82d978))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* added force indexing to Java and added ODICT_INDEX_DIR env variable ([62b2d68](https://github.com/TheOpenDictionary/odict/commit/62b2d6876ce8a34d2f6d0114308e06a3b4bd34e4))
* added Python extension ([#14](https://github.com/TheOpenDictionary/odict/issues/14)) ([5235b64](https://github.com/TheOpenDictionary/odict/commit/5235b64a0d2ca82a42662162034d9dbff5ccab81))
* extended Java and Python bindings to hold base64 dictionary in-memory to prevent constant I/O ([#15](https://github.com/TheOpenDictionary/odict/issues/15)) ([9c5c251](https://github.com/TheOpenDictionary/odict/commit/9c5c251e48a46a375807f8b476374ed209b4c316))
* **go:** add `split` command ([#31](https://github.com/TheOpenDictionary/odict/issues/31)) ([71814fc](https://github.com/TheOpenDictionary/odict/commit/71814fce27fda86fddb19b9907097e8dc530fba3))
* **java:** updated Java implementation to use more efficient Bleve search ([#19](https://github.com/TheOpenDictionary/odict/issues/19)) ([23d465e](https://github.com/TheOpenDictionary/odict/commit/23d465ef6a79f3d0eb6320a413c8284271c79f42))


### Bug Fixes

* attempt to fix various memory leaks ([#16](https://github.com/TheOpenDictionary/odict/issues/16)) ([bb33fa3](https://github.com/TheOpenDictionary/odict/commit/bb33fa3b5917c0eca798d3c577d8fc617acc83b2))
* corrected Python dll path ([60c80fd](https://github.com/TheOpenDictionary/odict/commit/60c80fdae1c6e3d826e7e6a608e4458cca067b46))
* fix tests ([8bbf73d](https://github.com/TheOpenDictionary/odict/commit/8bbf73d556d65d5ffb82669769cf4cf15e38e805))
* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* improved indexing and added indexing as default lookup method ([#20](https://github.com/TheOpenDictionary/odict/issues/20)) ([03e9e2f](https://github.com/TheOpenDictionary/odict/commit/03e9e2f1229813b4ec3082aae0bfa61abb0cdb68))
* **java:** moved shared library into JAR ([05b3208](https://github.com/TheOpenDictionary/odict/commit/05b3208a85b78952a49f37c8163063132f0b7dd2))
* **java:** replaced Java lookups with lookupByKey instead of Bleve ([#21](https://github.com/TheOpenDictionary/odict/issues/21)) ([5717b9d](https://github.com/TheOpenDictionary/odict/commit/5717b9ddeb76e0610eab4ffd1da01e4d84f0e93d))
* **python:** added custom library resolution ([313593b](https://github.com/TheOpenDictionary/odict/commit/313593bbfb7033acb8a2e6de34c5b34638114a96))
* **python:** fix python build ([3df34ac](https://github.com/TheOpenDictionary/odict/commit/3df34ac46051e6e618e890e157527cffad2d44bd))
* **python:** fixed ODict path ([669cace](https://github.com/TheOpenDictionary/odict/commit/669cace5c290e6c7e84ce3d1c4e752e726873798))
* **python:** updated Python target visibility ([474856d](https://github.com/TheOpenDictionary/odict/commit/474856d4a7bec8c107f0fea89fb9926fc296cc7c))
* remove log ([3074eb0](https://github.com/TheOpenDictionary/odict/commit/3074eb0b2cfab990506fbc694e24665417925421))

## [0.0.8](https://github.com/TheOpenDictionary/odict/compare/v0.0.7...v0.0.8) (2022-12-16)


### Features

* 1.5.0 refactor ([#30](https://github.com/TheOpenDictionary/odict/issues/30)) ([193519f](https://github.com/TheOpenDictionary/odict/commit/193519f62e014c496b975c22aeca32a0b9222e84))
* add support for entry aliasing ([#61](https://github.com/TheOpenDictionary/odict/issues/61)) ([077d947](https://github.com/TheOpenDictionary/odict/commit/077d94735a8d81f747015fc36045ff265ed1b736))
* add support for Japanese-specific POS tags ([#66](https://github.com/TheOpenDictionary/odict/issues/66)) ([2dd7171](https://github.com/TheOpenDictionary/odict/commit/2dd717101323d11481c9bd87e153f25feef3569f))
* add support for pretty-printing entries ([#62](https://github.com/TheOpenDictionary/odict/issues/62)) ([24d82bc](https://github.com/TheOpenDictionary/odict/commit/24d82bcf44b896347f4c9e9210cfe72bea82d978))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* added force indexing to Java and added ODICT_INDEX_DIR env variable ([62b2d68](https://github.com/TheOpenDictionary/odict/commit/62b2d6876ce8a34d2f6d0114308e06a3b4bd34e4))
* added Python extension ([#14](https://github.com/TheOpenDictionary/odict/issues/14)) ([5235b64](https://github.com/TheOpenDictionary/odict/commit/5235b64a0d2ca82a42662162034d9dbff5ccab81))
* extended Java and Python bindings to hold base64 dictionary in-memory to prevent constant I/O ([#15](https://github.com/TheOpenDictionary/odict/issues/15)) ([9c5c251](https://github.com/TheOpenDictionary/odict/commit/9c5c251e48a46a375807f8b476374ed209b4c316))
* **go:** add `split` command ([#31](https://github.com/TheOpenDictionary/odict/issues/31)) ([71814fc](https://github.com/TheOpenDictionary/odict/commit/71814fce27fda86fddb19b9907097e8dc530fba3))
* **java:** updated Java implementation to use more efficient Bleve search ([#19](https://github.com/TheOpenDictionary/odict/issues/19)) ([23d465e](https://github.com/TheOpenDictionary/odict/commit/23d465ef6a79f3d0eb6320a413c8284271c79f42))


### Bug Fixes

* attempt to fix various memory leaks ([#16](https://github.com/TheOpenDictionary/odict/issues/16)) ([bb33fa3](https://github.com/TheOpenDictionary/odict/commit/bb33fa3b5917c0eca798d3c577d8fc617acc83b2))
* corrected Python dll path ([60c80fd](https://github.com/TheOpenDictionary/odict/commit/60c80fdae1c6e3d826e7e6a608e4458cca067b46))
* fix tests ([8bbf73d](https://github.com/TheOpenDictionary/odict/commit/8bbf73d556d65d5ffb82669769cf4cf15e38e805))
* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* improved indexing and added indexing as default lookup method ([#20](https://github.com/TheOpenDictionary/odict/issues/20)) ([03e9e2f](https://github.com/TheOpenDictionary/odict/commit/03e9e2f1229813b4ec3082aae0bfa61abb0cdb68))
* **java:** moved shared library into JAR ([05b3208](https://github.com/TheOpenDictionary/odict/commit/05b3208a85b78952a49f37c8163063132f0b7dd2))
* **java:** replaced Java lookups with lookupByKey instead of Bleve ([#21](https://github.com/TheOpenDictionary/odict/issues/21)) ([5717b9d](https://github.com/TheOpenDictionary/odict/commit/5717b9ddeb76e0610eab4ffd1da01e4d84f0e93d))
* **python:** added custom library resolution ([313593b](https://github.com/TheOpenDictionary/odict/commit/313593bbfb7033acb8a2e6de34c5b34638114a96))
* **python:** fix python build ([3df34ac](https://github.com/TheOpenDictionary/odict/commit/3df34ac46051e6e618e890e157527cffad2d44bd))
* **python:** fixed ODict path ([669cace](https://github.com/TheOpenDictionary/odict/commit/669cace5c290e6c7e84ce3d1c4e752e726873798))
* **python:** updated Python target visibility ([474856d](https://github.com/TheOpenDictionary/odict/commit/474856d4a7bec8c107f0fea89fb9926fc296cc7c))
* remove log ([3074eb0](https://github.com/TheOpenDictionary/odict/commit/3074eb0b2cfab990506fbc694e24665417925421))

## [0.0.7](https://github.com/TheOpenDictionary/odict/compare/v0.0.6...v0.0.7) (2022-12-11)


### Features

* 1.5.0 refactor ([#30](https://github.com/TheOpenDictionary/odict/issues/30)) ([193519f](https://github.com/TheOpenDictionary/odict/commit/193519f62e014c496b975c22aeca32a0b9222e84))
* add support for entry aliasing ([#61](https://github.com/TheOpenDictionary/odict/issues/61)) ([077d947](https://github.com/TheOpenDictionary/odict/commit/077d94735a8d81f747015fc36045ff265ed1b736))
* add support for pretty-printing entries ([#62](https://github.com/TheOpenDictionary/odict/issues/62)) ([24d82bc](https://github.com/TheOpenDictionary/odict/commit/24d82bcf44b896347f4c9e9210cfe72bea82d978))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* added force indexing to Java and added ODICT_INDEX_DIR env variable ([62b2d68](https://github.com/TheOpenDictionary/odict/commit/62b2d6876ce8a34d2f6d0114308e06a3b4bd34e4))
* added Python extension ([#14](https://github.com/TheOpenDictionary/odict/issues/14)) ([5235b64](https://github.com/TheOpenDictionary/odict/commit/5235b64a0d2ca82a42662162034d9dbff5ccab81))
* extended Java and Python bindings to hold base64 dictionary in-memory to prevent constant I/O ([#15](https://github.com/TheOpenDictionary/odict/issues/15)) ([9c5c251](https://github.com/TheOpenDictionary/odict/commit/9c5c251e48a46a375807f8b476374ed209b4c316))
* **go:** add `split` command ([#31](https://github.com/TheOpenDictionary/odict/issues/31)) ([71814fc](https://github.com/TheOpenDictionary/odict/commit/71814fce27fda86fddb19b9907097e8dc530fba3))
* **java:** updated Java implementation to use more efficient Bleve search ([#19](https://github.com/TheOpenDictionary/odict/issues/19)) ([23d465e](https://github.com/TheOpenDictionary/odict/commit/23d465ef6a79f3d0eb6320a413c8284271c79f42))


### Bug Fixes

* attempt to fix various memory leaks ([#16](https://github.com/TheOpenDictionary/odict/issues/16)) ([bb33fa3](https://github.com/TheOpenDictionary/odict/commit/bb33fa3b5917c0eca798d3c577d8fc617acc83b2))
* corrected Python dll path ([60c80fd](https://github.com/TheOpenDictionary/odict/commit/60c80fdae1c6e3d826e7e6a608e4458cca067b46))
* fix tests ([8bbf73d](https://github.com/TheOpenDictionary/odict/commit/8bbf73d556d65d5ffb82669769cf4cf15e38e805))
* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* improved indexing and added indexing as default lookup method ([#20](https://github.com/TheOpenDictionary/odict/issues/20)) ([03e9e2f](https://github.com/TheOpenDictionary/odict/commit/03e9e2f1229813b4ec3082aae0bfa61abb0cdb68))
* **java:** moved shared library into JAR ([05b3208](https://github.com/TheOpenDictionary/odict/commit/05b3208a85b78952a49f37c8163063132f0b7dd2))
* **java:** replaced Java lookups with lookupByKey instead of Bleve ([#21](https://github.com/TheOpenDictionary/odict/issues/21)) ([5717b9d](https://github.com/TheOpenDictionary/odict/commit/5717b9ddeb76e0610eab4ffd1da01e4d84f0e93d))
* **python:** added custom library resolution ([313593b](https://github.com/TheOpenDictionary/odict/commit/313593bbfb7033acb8a2e6de34c5b34638114a96))
* **python:** fix python build ([3df34ac](https://github.com/TheOpenDictionary/odict/commit/3df34ac46051e6e618e890e157527cffad2d44bd))
* **python:** fixed ODict path ([669cace](https://github.com/TheOpenDictionary/odict/commit/669cace5c290e6c7e84ce3d1c4e752e726873798))
* **python:** updated Python target visibility ([474856d](https://github.com/TheOpenDictionary/odict/commit/474856d4a7bec8c107f0fea89fb9926fc296cc7c))
* remove log ([3074eb0](https://github.com/TheOpenDictionary/odict/commit/3074eb0b2cfab990506fbc694e24665417925421))

## [0.0.6](https://github.com/TheOpenDictionary/odict/compare/v0.0.5...v0.0.6) (2022-11-12)


### Features

* 1.5.0 refactor ([#30](https://github.com/TheOpenDictionary/odict/issues/30)) ([193519f](https://github.com/TheOpenDictionary/odict/commit/193519f62e014c496b975c22aeca32a0b9222e84))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* added force indexing to Java and added ODICT_INDEX_DIR env variable ([62b2d68](https://github.com/TheOpenDictionary/odict/commit/62b2d6876ce8a34d2f6d0114308e06a3b4bd34e4))
* added Python extension ([#14](https://github.com/TheOpenDictionary/odict/issues/14)) ([5235b64](https://github.com/TheOpenDictionary/odict/commit/5235b64a0d2ca82a42662162034d9dbff5ccab81))
* extended Java and Python bindings to hold base64 dictionary in-memory to prevent constant I/O ([#15](https://github.com/TheOpenDictionary/odict/issues/15)) ([9c5c251](https://github.com/TheOpenDictionary/odict/commit/9c5c251e48a46a375807f8b476374ed209b4c316))
* **go:** add `split` command ([#31](https://github.com/TheOpenDictionary/odict/issues/31)) ([71814fc](https://github.com/TheOpenDictionary/odict/commit/71814fce27fda86fddb19b9907097e8dc530fba3))
* **java:** updated Java implementation to use more efficient Bleve search ([#19](https://github.com/TheOpenDictionary/odict/issues/19)) ([23d465e](https://github.com/TheOpenDictionary/odict/commit/23d465ef6a79f3d0eb6320a413c8284271c79f42))


### Bug Fixes

* attempt to fix various memory leaks ([#16](https://github.com/TheOpenDictionary/odict/issues/16)) ([bb33fa3](https://github.com/TheOpenDictionary/odict/commit/bb33fa3b5917c0eca798d3c577d8fc617acc83b2))
* corrected Python dll path ([60c80fd](https://github.com/TheOpenDictionary/odict/commit/60c80fdae1c6e3d826e7e6a608e4458cca067b46))
* fix tests ([8bbf73d](https://github.com/TheOpenDictionary/odict/commit/8bbf73d556d65d5ffb82669769cf4cf15e38e805))
* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* improved indexing and added indexing as default lookup method ([#20](https://github.com/TheOpenDictionary/odict/issues/20)) ([03e9e2f](https://github.com/TheOpenDictionary/odict/commit/03e9e2f1229813b4ec3082aae0bfa61abb0cdb68))
* **java:** moved shared library into JAR ([05b3208](https://github.com/TheOpenDictionary/odict/commit/05b3208a85b78952a49f37c8163063132f0b7dd2))
* **java:** replaced Java lookups with lookupByKey instead of Bleve ([#21](https://github.com/TheOpenDictionary/odict/issues/21)) ([5717b9d](https://github.com/TheOpenDictionary/odict/commit/5717b9ddeb76e0610eab4ffd1da01e4d84f0e93d))
* **python:** added custom library resolution ([313593b](https://github.com/TheOpenDictionary/odict/commit/313593bbfb7033acb8a2e6de34c5b34638114a96))
* **python:** fix python build ([3df34ac](https://github.com/TheOpenDictionary/odict/commit/3df34ac46051e6e618e890e157527cffad2d44bd))
* **python:** fixed ODict path ([669cace](https://github.com/TheOpenDictionary/odict/commit/669cace5c290e6c7e84ce3d1c4e752e726873798))
* **python:** updated Python target visibility ([474856d](https://github.com/TheOpenDictionary/odict/commit/474856d4a7bec8c107f0fea89fb9926fc296cc7c))
* remove log ([3074eb0](https://github.com/TheOpenDictionary/odict/commit/3074eb0b2cfab990506fbc694e24665417925421))

## [0.0.5](https://github.com/TheOpenDictionary/odict/compare/v0.0.4...v0.0.5) (2022-11-06)


### Features

* 1.5.0 refactor ([#30](https://github.com/TheOpenDictionary/odict/issues/30)) ([193519f](https://github.com/TheOpenDictionary/odict/commit/193519f62e014c496b975c22aeca32a0b9222e84))
* add TypeScript library ([#33](https://github.com/TheOpenDictionary/odict/issues/33)) ([24e2a39](https://github.com/TheOpenDictionary/odict/commit/24e2a397931ed6d9fcbf71b7cc347f5c8bc9f52d))
* added force indexing to Java and added ODICT_INDEX_DIR env variable ([62b2d68](https://github.com/TheOpenDictionary/odict/commit/62b2d6876ce8a34d2f6d0114308e06a3b4bd34e4))
* added Python extension ([#14](https://github.com/TheOpenDictionary/odict/issues/14)) ([5235b64](https://github.com/TheOpenDictionary/odict/commit/5235b64a0d2ca82a42662162034d9dbff5ccab81))
* extended Java and Python bindings to hold base64 dictionary in-memory to prevent constant I/O ([#15](https://github.com/TheOpenDictionary/odict/issues/15)) ([9c5c251](https://github.com/TheOpenDictionary/odict/commit/9c5c251e48a46a375807f8b476374ed209b4c316))
* **go:** add `split` command ([#31](https://github.com/TheOpenDictionary/odict/issues/31)) ([71814fc](https://github.com/TheOpenDictionary/odict/commit/71814fce27fda86fddb19b9907097e8dc530fba3))
* **java:** updated Java implementation to use more efficient Bleve search ([#19](https://github.com/TheOpenDictionary/odict/issues/19)) ([23d465e](https://github.com/TheOpenDictionary/odict/commit/23d465ef6a79f3d0eb6320a413c8284271c79f42))


### Bug Fixes

* attempt to fix various memory leaks ([#16](https://github.com/TheOpenDictionary/odict/issues/16)) ([bb33fa3](https://github.com/TheOpenDictionary/odict/commit/bb33fa3b5917c0eca798d3c577d8fc617acc83b2))
* corrected Python dll path ([60c80fd](https://github.com/TheOpenDictionary/odict/commit/60c80fdae1c6e3d826e7e6a608e4458cca067b46))
* fixes splitting and return 2D array from CI ([#35](https://github.com/TheOpenDictionary/odict/issues/35)) ([4e80729](https://github.com/TheOpenDictionary/odict/commit/4e80729de3a96059774753b89ec336b66f1d729f))
* improved indexing and added indexing as default lookup method ([#20](https://github.com/TheOpenDictionary/odict/issues/20)) ([03e9e2f](https://github.com/TheOpenDictionary/odict/commit/03e9e2f1229813b4ec3082aae0bfa61abb0cdb68))
* **java:** moved shared library into JAR ([05b3208](https://github.com/TheOpenDictionary/odict/commit/05b3208a85b78952a49f37c8163063132f0b7dd2))
* **java:** replaced Java lookups with lookupByKey instead of Bleve ([#21](https://github.com/TheOpenDictionary/odict/issues/21)) ([5717b9d](https://github.com/TheOpenDictionary/odict/commit/5717b9ddeb76e0610eab4ffd1da01e4d84f0e93d))
* **python:** added custom library resolution ([313593b](https://github.com/TheOpenDictionary/odict/commit/313593bbfb7033acb8a2e6de34c5b34638114a96))
* **python:** fixed ODict path ([669cace](https://github.com/TheOpenDictionary/odict/commit/669cace5c290e6c7e84ce3d1c4e752e726873798))
* **python:** updated Python target visibility ([474856d](https://github.com/TheOpenDictionary/odict/commit/474856d4a7bec8c107f0fea89fb9926fc296cc7c))
* remove log ([3074eb0](https://github.com/TheOpenDictionary/odict/commit/3074eb0b2cfab990506fbc694e24665417925421))
