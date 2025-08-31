use std::hint::black_box;
use std::{fs::read_to_string, str::FromStr};

use criterion::{criterion_group, criterion_main, Criterion};
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
    let name = "example1";
    let input = format!("../examples/{}.xml", name);
    let xml = read_to_string(input).unwrap();
    let dict = black_box(odict::schema::Dictionary::from_str(&xml).unwrap());

    c.bench_function("compile", |b| {
        b.iter(|| {
            black_box(dict.build().unwrap().to_bytes().unwrap());
        });
    });
}

// fn bench_parse(c: &mut Criterion) {
//     let name = "wiktionary";
//     let input = format!("../examples/{}.xml", name);
//     let xml = read_to_string(input).unwrap();

//     c.bench_function("parse", |b| {
//         b.iter(|| {
//             Dictionary::from_str(black_box(&xml)).unwrap();
//         });
//     });
// }

criterion_group!(
    benches,
    //  bench_parse,
    bench_compile,
    bench_lookup
);
criterion_main!(benches);
