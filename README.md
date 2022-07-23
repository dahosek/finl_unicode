# finl Unicode support

This crate is designed for the Unicode needs of the finl project, but is designed to be usable by other software as well.
In the current release (1.0.x), support is provided for character code identification and grapheme segmentation and Unicode14.0.0.

## Overview 

### Category identification

Loading the `finl_unicode` crate with the `categories` feature will add methods onto the char type to test the category of a character
or identify its category. See the rustdoc for detail.

### Grapheme clusters

Loading the `finl_unicode` crate with the `grapheme_clusters` feature will extend `Peekable<CharIndices>` to have a `next_cluster()` method which will return the next grapheme cluster from the iterator.
There is also a pure cluster iterator available by calling `Graphemes::new(s)` on a `&str`. I don’t use this in finl, but wrote it using the same algorithm as the extension of `Peekable<CharIndices>` for the purposes of benchmarking.¹

## Why?

There *are* existing crates for these purposes, but segmentation lacked the interface for segmentation that I wanted (which was to be able to extend `Peekable<CharIndices>` with a method to fetch the next grapheme cluster if it existed). 
I incorrectly assumed that this would require character code identification, which turned out to be incorrect, but it turned out that the crate I was using was outdated and possibly abandoned and had an inefficient algorithm so it turned out to be a good thing that I wrote it.
I did benchmarks comparing my code against existing crates and discovered that I had managed to eke out performance gains against all of them, so that’s an added bonus.

###  Benchmark results

All benchmarks are generated using Criterion You can replicate them by running `cargo bench` from the project directory. Three numbers are given for all results: low/mean/high, all from the output of Criterion. The mean value is given in **bold**. 

#### Unicode categories
I ran three benchmarks to compare the performance of the crates. 
The Japanese text benchmark reads the Project Gutenberg EBook of *Kumogata monsho* by John Falkner and counts the characters in it which are Unicode letters.
The Czech text benchmark reads the Project Gutenberg EBook of *Cítanka pro skoly obecné* by Jan Stastný and Jan Lepar and Josef Sokol (this was to exercise testing against a Latin-alphabet text with lots of diacriticals). 
All letters and lowercase letters are counted.
The English text benchmark reads the Project Gutenberg eBook of *Frankenstein* by Mary Wollstonecraft Shelley (to run against a text which is pure ASCII).
All letters and lowercase letters are counted.

I compared against [unicode_categories](https://docs.rs/unicode_categories/latest/unicode_categories/) 0.1.1. All times are in ms. Smaller is better.

| Crate                |       Japanese text       | Czech text (all letters) |  Czech text (lowercase)  | English text (all letters) | English text (lowercase) |
|----------------------|:-------------------------:|:------------------------:|:------------------------:|:--------------------------:|:------------------------:|
| `finl_unicode`       | 1.1206/**1.1270**/1.1348  | 0.1668/**0.1753**/0.1851 | 0.2178/**0.2208**/0.2246 | 0.54807/**0.54993**/055198 | 0.7814/**0.7911**/0.8025 |
| `unicode_categories` | 14.080/**14.142**/14.206  | 2.9667/**2.9870**/3.0100 | 1.8623/**1.8868**/1.9168 | 12.182/**12.327**/12.506   | 8.2158/**8.2375**/8.2595 |  

As you can see, this is a clear win (the difference is the choice of algorithm. `finl_unicode` uses two-step table lookup to be able to store categories compactly while `unicode_categories` uses a combination of range checks and binary searches on tables).

#### Grapheme clusters

I compared against [unicode_segmentation](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/) 1.9.0 (part of the unicode-rs project) and [bstr](https://docs.rs/bstr/latest/bstr/) 0.2.17. 
Just one benchmark is run, this time counting the clusters in the graphemes.txt file that’s part of the Unicode documentation. All times are in µs.

| Crate                  |           Time           |
|------------------------|:------------------------:|
| `finl_unicode`         | 117.85/**118.36**/119.03 |
| `unicode_segmentation` | 217.26/**226.33**/236.03 |
| `bstr`                 | 369.75/**383.56**/400.49 |

To be honest, I wasn’t expecting these results, at least not compared to `unicode_segmentation`.
For `bstr`, the segmentation is managed through a synthetically-generated DFA created from a regex, and I would have been surprised if that was faster than a hand-coded algorithm, but I expected to be at best comparable with the unicode-rs people.

## Why not?

You may want to avoid this if you need `no_std` (maybe I’ll cover that in a future version, but probably not). 
If you need other clustering algorithms, I have no near future plans to implement them (but I would do it for money). 
I also do not support legacy clustering algorithms which are supported by `unicode-segmentation`.

## Filthy lucre

I’ve released this under an MIT/Apache license. Do what you like with it. 
I wouldn’t mind contributions to the ongoing support of developing finl, but they’re not necessary (although if you’re Microsoft or Google and you use my code, surely you can throw some dollars in my bank account).

---

1. For technical reasons, the iterator extension returns `Option<String>` rather than `Option<&str>` and thus will autmoatically underperform other implementations which are returning *all* the grapheme clusters. 
For finl, however, I would need an owned value for the string containing the cluster anyway and since I only occasionally need a cluster, I decided it was acceptable to take the performance hit. 
But see the benchmark results for the fact that I apparently managed to implement a faster algorithm anyway when doing an apples-to-apples comparison of speeds. 