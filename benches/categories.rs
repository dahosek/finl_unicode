use std::path::PathBuf;
use criterion::{criterion_group, criterion_main, Criterion};

mod finl_test {
    use finl_unicode::categories::CharacterCategories;

    #[inline]
    pub fn letter_test(c: &char) -> bool {
        c.is_letter()
    }

    #[inline]
    pub fn lc_test(c: &char) -> bool {
        c.is_letter_lowercase()
    }
}

mod uc_test {
    use unicode_categories::UnicodeCategories;
    #[inline]
    pub fn letter_test(c: &char) -> bool {
        c.is_letter()
    }

    #[inline]
    pub fn lc_test(c: &char) -> bool {
        c.is_letter_lowercase()
    }

}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    let mut japanese_txt = path.clone();
    japanese_txt.push("35018-0.txt");
    let mut czech_txt = path.clone();
    czech_txt.push("59765-0.txt");
    let mut english_txt = path.clone();
    english_txt.push("84-0.txt");

    let japanese = std::fs::read_to_string(japanese_txt).unwrap();
    let czech = std::fs::read_to_string(czech_txt).unwrap();
    let english = std::fs::read_to_string(english_txt).unwrap();

    let mut group = c.benchmark_group("Process Japanese text file");
    group.bench_function("finl_unicode",
        |b| b.iter(|| {
            japanese.chars().filter(|c| finl_test::letter_test(c)).count();
        })
    );
    group.bench_function("unicode_categories",
                         |b| b.iter(|| {
                             japanese.chars().filter(|c| uc_test::letter_test(c)).count();
                         })
    );
    group.finish();

    let mut group = c.benchmark_group("Process Czech text file");
    group.bench_function("finl_unicode",
                         |b| b.iter(|| {
                             czech.chars().filter(|c| finl_test::letter_test(c)).count();
                         })
    );
    group.bench_function("unicode_categories",
                         |b| b.iter(|| {
                             czech.chars().filter(|c| uc_test::letter_test(c)).count();
                         })
    );
    group.finish();

    let mut group = c.benchmark_group("Process Czech text file for lowercase");
    group.bench_function("finl_unicode",
                         |b| b.iter(|| {
                             czech.chars().filter(|c| finl_test::lc_test(c)).count();
                         })
    );
    group.bench_function("unicode_categories",
                         |b| b.iter(|| {
                             czech.chars().filter(|c| uc_test::lc_test(c)).count();
                         })
    );
    group.finish();

    let mut group = c.benchmark_group("Process English text file");
    group.bench_function("finl_unicode",
                         |b| b.iter(|| {
                             english.chars().filter(|c| finl_test::letter_test(c)).count();
                         })
    );
    group.bench_function("unicode_categories",
                         |b| b.iter(|| {
                             english.chars().filter(|c| uc_test::letter_test(c)).count();
                         })
    );
    group.finish();

    let mut group = c.benchmark_group("Process English text file for lowercase");
    group.bench_function("finl_unicode",
                         |b| b.iter(|| {
                             english.chars().filter(|c| finl_test::lc_test(c)).count();
                         })
    );
    group.bench_function("unicode_categories",
                         |b| b.iter(|| {
                             english.chars().filter(|c| uc_test::lc_test(c)).count();
                         })
    );
    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);