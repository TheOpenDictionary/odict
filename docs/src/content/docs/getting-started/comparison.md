---
title: Feature Comparison
description: What makes ODict different from other formats?
tableOfContents: false
---

| Feature | ODict | StarDict | DICT | XDXF |
|---------|-------|----------|------|------|
| Human-readable source format | XML | Plain text plus indexes | Plain text database files | XML |
| Compiled binary output | ✅ | ✅ | ❌ | ❌ |
| Offline lookup | ✅ | ✅ | Usually server-backed | ✅ |
| Exact lookup without indexing | ✅ | Uses generated indexes | Depends on server database | Depends on application |
| Built-in full-text search | ✅ | Application-dependent | Server-dependent | Application-dependent |
| Built-in tokenization | ✅ | ❌ | ❌ | ❌ |
| Primary access methods | CLI, Rust, Python, JavaScript, HTTP | Desktop/mobile readers | Network protocol and servers | Import/export format |

Use ODict when you want a dictionary format that is easy to author, compact to ship, and practical to use from applications.
