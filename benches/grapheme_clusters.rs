use std::path::PathBuf;
use criterion::{criterion_group, criterion_main, Criterion};

mod finl_test {
    use finl_unicode::grapheme_clusters::Graphemes;

    pub fn read_clusters(input: &String) -> usize {
        let mut cnt = 0;
        Graphemes::new(input).for_each(
            |c| {
                if c.len() == 1 {
                    cnt += 1;
                }
            }
        );
        cnt
    }
}

mod unicode_rs {
   use unicode_segmentation::UnicodeSegmentation;

    pub fn read_clusters(input: &String) -> usize {
        let mut cnt = 0;
        input.graphemes(true).for_each(
            |c| {
                if c.len() == 1 {
                    cnt += 1;
                }
            }
        );
        cnt
    }
}

mod bstr {
    use bstr::ByteSlice;

    pub fn read_clusters(input: &String) -> usize {
        let mut cnt = 0;
        input.as_bytes().graphemes().for_each(
            |c| {
                if c.len() == 1 {
                    cnt += 1;
                }
            }
        );
        cnt
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    let mut graphemes_txt = path.clone();
    graphemes_txt.push("graphemes.txt");

    let graphemes = std::fs::read_to_string(graphemes_txt).unwrap();

    let mut group = c.benchmark_group("Process graphemes");
    group.bench_function("finl_unicode",
                         |b| b.iter(|| {
                             finl_test::read_clusters(&graphemes);
                         })
    );

    group.bench_function("unicode-rs",
                         |b| b.iter(|| {
                             unicode_rs::read_clusters(&graphemes);
                         })
    );

    group.bench_function("bstr",
                         |b| b.iter(|| {
                             bstr::read_clusters(&graphemes);
                         })
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);