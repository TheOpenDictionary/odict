# Changelog

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
