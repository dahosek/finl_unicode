use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufRead, BufReader,Write};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let unicode_version = "14.0.0";
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let data_dir = Path::new(&out_dir).join("data").join(unicode_version);
    std::fs::create_dir_all(&data_dir)?;
    let unicode_data_txt = data_dir.join("UnicodeData.txt");
    let grapheme_break_test_txt = data_dir.join("GraphemeBreakTest.txt");
    let grapheme_break_property_txt = data_dir.join("GraphemeBreakProperty.txt");
    let emoji_data_txt = data_dir.join("emoji-data.txt");

    download_unicode_data(&unicode_data_txt, "ucd/UnicodeData.txt", unicode_version)?;
    build_character_tables(&out_dir, &unicode_data_txt)?;
    download_unicode_data(&grapheme_break_test_txt, "ucd/auxiliary/GraphemeBreakTest.txt", unicode_version)?;
    build_grapheme_break_test(&out_dir, &grapheme_break_test_txt)?;
    download_unicode_data(&grapheme_break_property_txt, "ucd/auxiliary/GraphemeBreakProperty.txt", unicode_version)?;
    download_unicode_data(&emoji_data_txt, "ucd/emoji/emoji-data.txt", unicode_version)?;
    build_grapheme_break_property(&out_dir, &grapheme_break_property_txt, &emoji_data_txt)?;
    Ok(())
}


// We store the category code as a u8 value with the following meaning:
// High nibble:
// - 1 mark
// - 2 Number
// - 3 Punctuation
// - 4 Symbol
// - 5 Seperator
// - 6 Control
// - 7 Control
// - 8 Letter
// - 9 Cased letter
//
// code & xF0 to get the larger category.
// 8/9 for letters allows us to do code & x80 to see if its a letter
//
// Specific codes map as follows:
// Lu	Uppercase_Letter	x90
// Ll	Lowercase_Letter	x91
// Lt	Titlecase_Letter	x92
// Lm	Modifier_Letter	    x80
// Lo	Other_Letter	    x81
// Mn	Nonspacing_Mark	    x10
// Mc	Spacing_Mark	    x11
// Me	Enclosing_Mark	    x12
// Nd	Decimal_Number	    x20
// Nl	Letter_Number	    x21
// No	Other_Number	    x22
// Pc	Connector_Punctuation	x30
// Pd	Dash_Punctuation	x31
// Ps	Open_Punctuation	x32
// Pe	Close_Punctuation	x33
// Pi	Initial_Punctuation	x34
// Pf	Final_Punctuation	x35
// Po	Other_Punctuation	x36
// Sm	Math_Symbol	        x40
// Sc	Currency_Symbol	    x41
// Sk	Modifier_Symbol	    x42
// So	Other_Symbol	    x43
// Zs	Space_Separator	    x50
// Zl	Line_Separator	    x51
// Zp	Paragraph_Separator	x52
// Cc	Control	            x60
// Cf	Format	            x61
// Cs	Surrogate	        x62
// Co	Private_Use	        x63
// Cn	Unassigned	        x64
fn cat_to_u8(cat: &str) -> u8 {
    match cat {
        "Lu" => 0x90,
        "Ll" => 0x91,
        "Lt" => 0x92,
        "Lm" => 0x83,
        "Lo" => 0x84,
        "Mn" => 0x10,
        "Mc" => 0x11,
        "Me" => 0x12,
        "Nd" => 0x20,
        "Nl" => 0x21,
        "No" => 0x22,
        "Pc" => 0x30,
        "Pd" => 0x31,
        "Ps" => 0x32,
        "Pe" => 0x33,
        "Pi" => 0x34,
        "Pf" => 0x35,
        "Po" => 0x36,
        "Sm" => 0x40,
        "Sc" => 0x41,
        "Sk" => 0x42,
        "So" => 0x43,
        "Zs" => 0x50,
        "Zl" => 0x51,
        "Zp" => 0x52,
        "Cc" => 0x61,
        "Cf" => 0x62,
        "Cs" => 0x63,
        "Co" => 0x64,
        _ => 0x60 // Anything we don't recognize is x60, which is Cn - Unassigned
    }
}

// Credit to https://here-be-braces.com/fast-lookup-of-unicode-properties/ for the broad outline of
// how this would work. The coding of categories into bytes is my own.
fn build_character_tables(out_dir: &OsStr, unicode_data_txt: &PathBuf) -> Result<(), Box<dyn Error>> {
    let characters_rs = Path::new(out_dir).join("characters.rs");
    let mut characters_rs = File::create(characters_rs)?;
    let unicode_data = File::open(unicode_data_txt)?;
    let unicode_data = BufReader::new(unicode_data);

    // First we build a raw (large) index of all the character codes in numeric format
    // Note that we actually allocate an array slightly larger than Unicode uses
    let mut raw_categories = [0x60u8;0x110000];
    let mut range_start = 0;
    for line in unicode_data.lines() {
        let line = line.unwrap();
        let mut fields = line.split(";");
        let char_code = fields.next();
        let char_name = fields.next();
        let category = fields.next().unwrap();
        if let Some(char_code) = char_code {
            let char_code = usize::from_str_radix(char_code, 16)?;
            if let Some(char_name) = char_name {
                if char_name.ends_with(", First>") {
                    range_start = char_code;
                }
                else if char_name.ends_with(", Last>") {
                    let cat_code = cat_to_u8(category);
                    for i in range_start..=char_code {
                        raw_categories[i] = cat_code;
                    }
                }
                else {
                    raw_categories[char_code] = cat_to_u8(category);
                }
            }
        }
    }

    // Then we break it down into pages (wrapping the result with a bit of Rust boilerplate)
    writeln!(characters_rs, "const CAT_TABLE: [u8;0x1100] = [")?;
    let mut page_index = HashMap::new();
    let mut page_number = 0u8;
    for page in 0 .. 0x1100 {
        let page_start = page << 8;
        let page_data  = raw_categories[page_start..page_start+0x100].to_vec();
        let &mut page_ref = page_index.entry(page_data).or_insert(page_number);
        if page_ref == page_number {
            page_number += 1
        }
        writeln!(characters_rs, "\t {page_ref}, // {page:#x}")?;
        // let mut values_seen: HashSet<u8> = raw_categories[page_start..page_start+0x100].iter().map(|x| *x).collect();
        // values_seen.remove(&0);
        // if values_seen.len() <= 1 {
        //     let single_code = values_seen.iter().next().cloned().unwrap_or(0);
        //     // There was only one code present in the page
        //     writeln!(characters_rs, "\tEither::Code({single_code:#x}), // {page:#x}")?;
        // }
        // else {
        //     writeln!(characters_rs, "\tEither::Page({page_number}), // {page:#x} -- {}", values_seen.len())?;
        //
        //     cat_pages.push(raw_categories[page_start..page_start+0x100].to_vec());
        //     page_number += 1;
        // }
    }
    writeln!(characters_rs, "];")?;
    let cat_pages = page_index.iter()
        .map(|(k, v)| (v,k))
        .sorted_by(|(a,_),(b,_)| Ord::cmp(a,b))
        .map(|(_, page)| page )
        .collect_vec();
    writeln!(characters_rs, "const CAT_PAGES: [[u8;256];{}] = {cat_pages:#x?};", cat_pages.len())?;
    Ok(())
}

fn build_grapheme_break_test(out_dir: &OsString, grapheme_break_test_txt: &PathBuf) -> Result<(), Box<dyn Error>> {
    let grapheme_test_rs = Path::new(out_dir).join("grapheme_test.rs");
    let mut grapheme_test_rs = File::create(grapheme_test_rs)?;
    let grapheme_break_test = File::open(grapheme_break_test_txt)?;
    let grapheme_break_test = BufReader::new(grapheme_break_test);
    let mut grapheme_bench_txt = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    grapheme_bench_txt.push("resources");
    grapheme_bench_txt.push("graphemes.txt");
    let mut grapheme_bench_txt = File::create(grapheme_bench_txt)?;

    writeln!(grapheme_test_rs, "{{")?;
    for line in grapheme_break_test.lines() {
        let line = line.unwrap();
        if let Some((map, comment)) = line.split_once('#') {
            if map.len() > 0 {
                let mut input_string = String::new();
                let mut output_string:Vec<String> = vec!();
                let mut current_grapheme = String::new();
                writeln!(grapheme_bench_txt, "{}", map)?;
                for token in map.split_whitespace() {
                    match token {
                        "÷" => {
                            if current_grapheme.len() > 0 {
                                output_string.push(current_grapheme);
                                current_grapheme = String::new();
                            }
                        }
                        "×" => {} // No action necessary here (!)
                        hex_code => {
                            write!(grapheme_bench_txt, "{}", char::from_u32(u32::from_str_radix(hex_code, 16).unwrap()).unwrap())?;
                            let hex_code = "\\u{".to_string() + hex_code + "}";
                            input_string.push_str(&hex_code);
                            current_grapheme.push_str(&hex_code);
                        }
                    }
                }
                writeln!(grapheme_bench_txt)?;
                let output_string = output_string.join("\", \"");

                writeln!(grapheme_test_rs, "\tgrapheme_test(\"{input_string}\",\n\t\t&[\"{output_string}\"],\n\t\t\"{comment}\"\n\t);")?;
            }
        }
    }
    writeln!(grapheme_test_rs, "}}")?;
    Ok(())
}


fn encode_property(property: &str) -> u8 {
    match property {
        "Prepend" => 0x05,
        "CR" => 0x04,
        "LF" => 0x04,
        "Control" => 0x04,
        "Extend" => 0x01,
        "SpacingMark" => 0x02,
        "L" => 0x0c,
        "V" => 0x08,
        "T" => 0x09,
        "LV" => 0x0d,
        "LVT" => 0x0e,
        "ZWJ" => 0x03,
        "Extended_Grapheme" => 0x06,
        "Regional_Indicator" => 0x07,
        _ => 0x00,
    }
}

fn str_to_range(range: &str) -> RangeInclusive<usize> {
    if let Some((first, last)) = range.split_once("..") {
        u32::from_str_radix(first, 16).unwrap() as usize ..=
            u32::from_str_radix(last,16).unwrap() as usize
    }
    else {
        let val = u32::from_str_radix(range, 16).unwrap() as usize;
        val..=val
    }
}

fn build_grapheme_break_property(out_dir: &OsString, grapheme_break_property_txt: &PathBuf, emoji_data_txt: &PathBuf) -> Result<(), Box<dyn Error>> {
    let grapheme_property_rs = Path::new(out_dir).join("grapheme_property.rs");
    let mut grapheme_property_rs = File::create(grapheme_property_rs)?;
    let grapheme_break_property = File::open(grapheme_break_property_txt)?;
    let grapheme_break_property = BufReader::new(grapheme_break_property);
    let emoji_data = File::open(emoji_data_txt)?;
    let emoji_data = BufReader::new(emoji_data);

    // first pass: build an array of all the properties
    let mut raw_grapheme_properties = [0u8;0x110000];
    for line in grapheme_break_property.lines() {
        let line = line.unwrap();
        if let Some((line, _)) = line.split_once('#') {
            if let Some((range, property)) = line.split_once(';') {
                let range = range.trim();
                let property = property.trim();
                raw_grapheme_properties.get_mut(str_to_range(range)).unwrap().fill(encode_property(property));
            }
        }
    }

    // add extended graphemes from emoji data
    for line in emoji_data.lines() {
        let line = line.unwrap();
        if let Some((line, _)) = line.split_once('#') {
            if let Some((range, property)) = line.split_once(';') {
                let range = range.trim();
                let property = property.trim();
                if property == "Extended_Pictographic" {
                    raw_grapheme_properties.get_mut(str_to_range(range)).unwrap().fill(0x06);
                }
            }
        }
    }

    // Then we break it down into pages (wrapping the result with a bit of Rust boilerplate)
    writeln!(grapheme_property_rs, "const GP_TABLE: [Either;0x1100] = [")?;
    let mut cat_pages = vec!();
    let mut page_number = 0;
    for page in 0 .. 0x1100 {
        let page_start = page << 8;
        let values_seen: HashSet<u8> = raw_grapheme_properties[page_start..page_start+0x100].iter().map(|x| *x).collect();
        if values_seen.len() == 1 {
            let single_code = values_seen.iter().next().cloned().unwrap_or(0);
            // There was only one code present in the page
            writeln!(grapheme_property_rs, "\tEither::Code({single_code:#x}), // {page:#x}")?;
        }
        else {
            writeln!(grapheme_property_rs, "\tEither::Page({page_number}), // {page:#x} -- {}", values_seen.len())?;

            cat_pages.push(raw_grapheme_properties[page_start..page_start+0x100].to_vec());
            page_number += 1;
        }
    }
    writeln!(grapheme_property_rs, "];")?;
    writeln!(grapheme_property_rs, "const GP_PAGES: [[u8;256];{}] = [", cat_pages.len())?;
    for (page, idx) in cat_pages.iter().zip(0..) {
        writeln!(grapheme_property_rs, "/* {idx} */\t{page:?},")?;
    }
    //{cat_pages:x?};
    writeln!(grapheme_property_rs, "];")?;

    Ok(())
}


fn download_unicode_data(local_txt_data_file: &PathBuf, remote_txt_data_file: &str, unicode_version: &str) -> Result<(), Box<dyn Error>> {
    let url_base = "https://www.unicode.org/Public/".to_owned() + unicode_version + "/";
    let client = Client::new();
    if !local_txt_data_file.exists() {
        let mut remote_data = client.get(url_base.clone() + remote_txt_data_file).send()?;
        let mut file = File::create(local_txt_data_file)?;
        std::io::copy(&mut remote_data, &mut file)?;
    }
    Ok(())
}