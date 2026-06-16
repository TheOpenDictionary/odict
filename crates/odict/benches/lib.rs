use std::hint::black_box;
use std::str::FromStr;

use criterion::{criterion_group, criterion_main, Criterion};
use odict::format::xml::ToXML;
use odict::schema::Dictionary;
use odict::{lookup::LookupOptions, OpenDictionary};

fn bench_lookup(c: &mut Criterion) {
    let dict = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { OpenDictionary::from_remote("wiktionary/eng").await.unwrap() });

    c.bench_function("lookup", |b| {
        b.iter(async || {
            black_box(
                dict.contents()
                    .unwrap()
                    .lookup(&vec!["run"], LookupOptions::default())
                    .unwrap(),
            )
        });
    });
}

fn bench_compile(c: &mut Criterion) {
    let dict = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { OpenDictionary::from_remote("wiktionary/eng").await.unwrap() });

    let xml = dict
        .contents()
        .unwrap()
        .deserialize()
        .unwrap()
        .to_xml(true)
        .unwrap();

    let dict = black_box(odict::schema::Dictionary::from_str(&xml).unwrap());

    c.bench_function("compile", |b| {
        b.iter(|| {
            black_box(dict.build().unwrap().to_bytes().unwrap());
        });
    });
}

fn bench_parse(c: &mut Criterion) {
    let dict = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { OpenDictionary::from_remote("wiktionary/eng").await.unwrap() });

    let xml = dict
        .contents()
        .unwrap()
        .deserialize()
        .unwrap()
        .to_xml(true)
        .unwrap();

    c.bench_function("parse", |b| {
        b.iter(async || black_box(Dictionary::from_str(black_box(&xml)).unwrap()));
    });
}

criterion_group!(benches, bench_parse, bench_compile, bench_lookup);
criterion_main!(benches);
