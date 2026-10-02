# finl Unicode support

This crate is designed for the Unicode needs of the finl project, but is designed to be usable by other software as well.
In the current release (1.5.x), support is provided for character code identification and grapheme segmentation and Unicode18.0.0.

## Overview 

### Category identification

Loading the `finl_unicode` crate with the `categories` feature will add methods onto the char type to test the category of a character
or identify its category. See the rustdoc for detail.

### Grapheme clusters

Loading the `finl_unicode` crate with the `grapheme_clusters` feature will extend `Peekable<CharIndices>` to have a `next_cluster()` method which will return the next grapheme cluster from the iterator.
There is also a pure cluster iterator available by calling `Graphemes::new(s)` on a `&str`. I don’t use this in finl, but wrote it using the same algorithm as the extension of `Peekable<CharIndices>` for the purposes of benchmarking.¹

## Why?

There *are* existing crates for these purposes, but segmentation lacked the interface for segmentation that I wanted (which was to be able to extend `Peekable<CharIndices>` with a method to fetch the next grapheme cluster if it existed). 
I incorrectly assumed that this would require character code identification, which turned out to be incorrect. 
It turned out, though, that the crate I was using was outdated and possibly abandoned and had an inefficient algorithm so it turned out to be a good thing that I wrote it.
I benchmarked my code against existing crates and discovered that I had managed to eke out performance gains against all of them, so that’s an added bonus.

###  Benchmark results

All benchmarks are generated using Criterion You can replicate them by running `cargo bench` from the project directory. Three numbers are given for all results: low/mean/high, all from the output of Criterion. The mean value is given in **bold**. 

#### Unicode categories
I ran three benchmarks to compare the performance of the crates on my M3 Max MacBook Pro. 
The Japanese text benchmark reads the Project Gutenberg EBook of *Kumogata monsho* by John Falkner and counts the characters in it which are Unicode letters.
The Czech text benchmark reads the Project Gutenberg EBook of *Cítanka pro skoly obecné* by Jan Stastný and Jan Lepar and Josef Sokol (this was to exercise testing against a Latin-alphabet text with lots of diacriticals). 
All letters are counted in the first benchmark and lowercase letters only are counted in the second.
The English text benchmark reads the Project Gutenberg eBook of *Frankenstein* by Mary Wollstonecraft Shelley (to run against a text which is pure ASCII).
All letters and lowercase letters are counted in two benchmarks as with the Czech text. The source code check is from neovim. Again, letters and lowercase letters are counted in the sample.

I compared against [unicode_categories](https://docs.rs/unicode_categories/latest/unicode_categories/) 0.1.1. All times are in ms. Smaller is better.

| Benchmark                | `finl_unicode`              | `unicode_categories`        |
|--------------------------|-----------------------------|-----------------------------|
| Japanese text            | 0.26270/**0.26330**/0.26394 | 6.5762/**6.5904**/6.6051    |
| Czech text               | 0.07565/**0.07595**/0.07624 | 1.7186/**1.7261**/1.7345    |
| Czech text (lowercase)   | 0.07642/**0.07680**/0.07717 | 0.58435/**0.58744**/0.59079 |
| English text             | 0.24936/**0.25033**/0.25132 | 6.9849/**7.0021**/7.0219    |
| English text (lowercase) | 0.24601/**0.24646**/0.24694 | 2.6310/**2.6340**/2.6371    |
| Source code              | 0.02727/**0.02733**/0.02740 | 1.0621/**1.0652**/1.0687    |
| Source code (lowercase)  | 0.02736/**0.02741**/0.02745 | 0.30283/**0.30362**/0.30446 |

As you can see, this is a clear win (the difference is the choice of algorithm. `finl_unicode` uses two-step table lookup to be able to store categories compactly while `unicode_categories` uses a combination of range checks and binary searches on tables).

#### Grapheme clusters

I compared against [unicode_segmentation](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/) 1.13.3 (part of the unicode-rs project) and [bstr](https://docs.rs/bstr/latest/bstr/) 1.13.1. 
Comparisons are run against graphemes.txt, derived from the Unicode test suite, plus several language
texts that were part of the `unicode_segmentation` benchmark suite. 

All times are in µs, smaller is better.

| Benchmark         | `finl_unicode`           | `unicode_segmentation`   | `bstr`                   |
|-------------------|--------------------------|--------------------------|--------------------------|
| Unicode graphemes | 54.690/**54.801**/54.918 | 220.05/**220.34**/220.63 | 262.26/**263.05**/263.89 |
| Arabic text       | 108.41/**108.87**/109.34 | 315.56/**316.26**/317.00 | 1031.0/**1034.8**/1038.8 |
| English text      | 151.20/**151.47**/151.77 | 468.32/**469.54**/470.85 | 342.28/**343.43**/344.69 |
| Hindi text        | 84.608/**85.208**/85.898 | 395.39/**396.49**/397.72 | 823.94/**825.76**/827.68 |
| Japanese text     | 64.420/**64.524**/64.631 | 319.04/**319.58**/320.13 | 964.42/**966.19**/968.03 |
| Korean text       | 115.66/**116.03**/116.41 | 404.36/**405.57**/406.87 | 1267.5/**1269.9**/1272.4 |
| Mandarin text     | 61.850/**61.965**/62.102 | 302.59/**303.48**/304.50 | 875.73/**877.33**/878.99 |
| Russian text      | 108.96/**109.30**/109.77 | 322.96/**324.14**/325.35 | 860.84/**868.11**/875.76 |
| Source code       | 159.04/**159.29**/159.57 | 459.64/**460.81**/461.96 | 482.66/**484.42**/486.23 |

With the move from benchmarking on Intel to Apple Silicon, the performance difference for my code versus the other
libraries were generally expanded, but it appears that with updates, that difference has narrowed in some cases.
In the course of the latest updates to handle indic conjunct clustering, some minor internal changes were done to the
routine which gave minor speed increases for non-Hindi text after an initial performance regression.
In 1.5.1, restructuring the emoji/Indic states of the clustering state machine and storing the lookup tables as
`static` rather than `const` gave a further 2–10% speedup on most texts (about 20% on Korean).

## Why not?

You may want to avoid this if you need `no_std` (I’m looking at an update that will create a new contract for clusters
which will remove the allocations and I just need to decide whether this will involve removing the existing interface or merely
deprecating it). 

If you need other clustering algorithms, I have no near future plans to implement them (but I would do it for money). 

There is no equivalent to `unicode_segmentation`’s `GraphemeCursor` as I don’t need that functionality 
for finl. Reverse iteration over graphemes is not supported, nor do I have plans to support it.

I do not support legacy clustering algorithms which are supported by `unicode-segmentation`. However, the Unicode
specification discourages the use of legacy clustering which is only documented for backwards compatability with very old versions of the Unicode standard.²


## Unicode copyright notice

This package incorporates data from Unicode Inc.
Copyright © 1991–2025 Unicode, Inc. All rights reserved.

## Support

I’ve released this under an MIT/Apache License. Do what you like with it. 
I wouldn’t mind contributions to the ongoing support of developing finl, but they’re not necessary (although if you’re Microsoft or Google and you use my code, surely you can throw some dollars in my bank account).
I guarantee no warranty or support, although if you care to throw some money my way, I can prioritize your requests.

## Version history

- **1.0.0** Initial release
- **1.0.1** Build-process changes to make docs.rs documentation build
- **1.0.2** More changes because the first round apparently weren’t enough
- **1.1.0** Add support for Unicode 15.0.0, added new benchmark comparisons.
- **1.2.0** Allow grapheme clustering to work on any `Peekable` iterator over `char` or `(usize,char)`.
- **1.3.0** Add support for Unicode 16.0.0 (significant changes required for Indic Conjunct clusters), update license documentation and benchmark comparisons.
- **1.4.0** Add support for Unicode 17.0.0
- **1.5.0** Add support for Unicode 18.0.0 (more changes for Indic Conjunct clusters), some internal improvements.
- **1.5.1** Fix grapheme clustering of emoji ZWJ sequences: Extend/SpacingMark/ZWJ after an emoji ZWJ now attach (GB9/GB9a), a SpacingMark no longer continues a GB11 sequence, and a virama after an emoji no longer blocks GB11.

---

1. For technical reasons, the iterator extension returns `Option<String>` rather than `Option<&str>` and thus will automatically underperform other implementations which are returning *all* the grapheme clusters. 
For finl, however, I would need an owned value for the string containing the cluster anyway and since I only occasionally need a cluster, I decided it was acceptable to take the performance hit. 
But see the benchmark results for the fact that I apparently managed to implement a faster algorithm anyway when doing an apples-to-apples comparison of speeds. 
2. Pure speculation, but I think that this might be the entire reason for the difference in performance between `finl_unicode` and `unicode_segmentation`. However, I have not looked at the source code to confirm my suspicion.