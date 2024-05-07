use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use odict::{lookup::LookupOptions, Dictionary, DictionaryWriter};

mod helpers;

use helpers::EXAMPLE_DICTIONARY_1;

fn bench_lookup(c: &mut Criterion) {
    let dict = EXAMPLE_DICTIONARY_1.to_archive().unwrap();

    c.bench_function("lookup", |b| {
        b.iter(|| {
            dict.lookup(black_box(&vec!["run"]), black_box(LookupOptions::default()))
                .unwrap()
        });
    });
}

fn bench_compile(c: &mut Criterion) {
    let name = "wiktionary";
    let writer = DictionaryWriter::default();
    let input = format!("../examples/{}.xml", name);
    let xml = read_to_string(input).unwrap();
    let dict = Dictionary::from(black_box(&xml)).unwrap();

    c.bench_function("compile", |b| {
        b.iter(|| {
            writer.write_to_bytes(&dict).unwrap();
        });
    });
}

fn bench_parse(c: &mut Criterion) {
    let name = "wiktionary";
    let input = format!("../examples/{}.xml", name);
    let xml = read_to_string(input).unwrap();

    c.bench_function("parse", |b| {
        b.iter(|| {
            Dictionary::from(black_box(&xml)).unwrap();
        });
    });
}

criterion_group!(benches, bench_parse, bench_compile, bench_lookup);
criterion_main!(benches);
