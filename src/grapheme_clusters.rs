use std::iter::Peekable;
use std::str::CharIndices;

// use crate::grapheme_clusters::ClusterCategory::{EXTEND, PREPEND, SPACING_MARK, ZWJExtCccZwj};
use crate::grapheme_clusters::ClusterMachineState::{CcsBase, CcsExtend, CrLf, Emoji, EmojiZWJ, Flag, HangulSyllableL, HangulSyllableT, HangulSyllableV, Other, Precore, Start};

/// Get the next grapheme cluster from a stream of characters or char indices
///
pub trait GraphemeCluster {
    fn next_cluster(&mut self) -> Option<String>;
}

pub struct Graphemes<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Graphemes<'a> {
    pub fn new(input: &'a str) -> Graphemes<'a> {
        let mut iter = input.char_indices().peekable();
        Graphemes {
            input,
            iter
        }
    }
}

impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&(start, _)) = self.iter.peek() {
            let mut cluster_machine = ClusterMachine::new();
            loop {
                if let Some(&(curr_loc, ch)) = self.iter.peek() {
                    match cluster_machine.find_cluster(ch) {
                        Break::None => { self.iter.next(); }
                        Break::Before => {
                            return Some(&self.input[start..curr_loc]);
                        }
                        Break::After => {
                            self.iter.next();
                            return Some(
                                if let Some(&(curr_loc, _)) = self.iter.peek() {
                                    &self.input[start..curr_loc]
                                } else {
                                    &self.input[start..]
                                });
                        }
                    }
                }
                else {
                    return Some(&self.input[start..]);
                }
            }
        } else {
            None
        }
    }
}

/// We implement the trait for the struct `Peekable<CharIndices>`.
impl GraphemeCluster for Peekable<CharIndices<'_>> {
    #[inline]
    fn next_cluster(&mut self) -> Option<String> {
        if self.peek().is_some() {
            let mut cluster_machine = ClusterMachine::new();
            let mut rv = String::new();
            loop {
                if let Some(&(_, ch)) = self.peek() {
                    let state = cluster_machine.find_cluster(ch);
                    match state {
                        Break::None => {
                            rv.push(ch);
                            self.next();
                        }
                        Break::Before => { return Some(rv); }
                        Break::After => {
                            rv.push(ch);
                            self.next();
                            return Some(rv);
                        }
                    }
                } else {
                    break;
                }
            }
            Some(rv)
        } else {
            None
        }
    }
}


#[derive(PartialEq)]
enum ClusterMachineState {
    Start,
    Precore,
    CcsBase,
    CrLf,
    HangulSyllableL,
    HangulSyllableV,
    HangulSyllableT,
    CcsExtend,
    Flag,
    Emoji,
    EmojiExtend,
    EmojiZWJ,
    Other,
}

#[derive(Debug, PartialEq)]
enum Break {
    None,
    Before,
    After,
}

struct ClusterMachine {
    state: ClusterMachineState,
}

impl ClusterMachine {
    #[inline]
    pub fn new() -> ClusterMachine {
        ClusterMachine {
            state: Start,
        }
    }

    /// If we have a cluster, we return the cluster in a `String` in an `Option` long with a `bool`
    /// If the `bool` is true, it means that we are also consuming the character  in the cluster.
    #[inline]
    pub fn find_cluster(&mut self, c: char) -> Break {
        if self.state == Start {
            return self.first_character(c);
        }
        let property = get_property(c);

        if property == GraphemeProperty::CONTROL {
            return if self.state == CrLf && c == '\n' {
                self.state = Start;
                Break::After
            } else {
                if c == '\r' {
                    self.state = CrLf;
                } else {
                    self.state = Start;
                }
                Break::Before
            }
        }

        match self.state {
            Start => self.first_character(c),
            Precore => {
                self.first_character(c);
                Break::None
            }

            HangulSyllableL => {
                match property {
                    GraphemeProperty::L => Break::None,
                    GraphemeProperty::V | GraphemeProperty::LV => {
                        self.state = HangulSyllableV;
                        Break::None
                    }
                    GraphemeProperty::LVT => {
                        self.state = HangulSyllableT;
                        Break::None
                    }
                    GraphemeProperty::EXTEND | GraphemeProperty::SPACING_MARK | GraphemeProperty::ZWJ => {
                        self.state = CcsBase;
                        Break::None
                    }
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            }
            HangulSyllableV => {
                match property {
                    GraphemeProperty::V => Break::None,
                    GraphemeProperty::T => {
                        self.state = HangulSyllableT;
                        Break::None
                    }
                    GraphemeProperty::EXTEND | GraphemeProperty::SPACING_MARK | GraphemeProperty::ZWJ => {
                        self.state = CcsBase;
                        Break::None
                    }
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            }
            HangulSyllableT => {
                match property {
                    GraphemeProperty::T => Break::None,
                    GraphemeProperty::EXTEND | GraphemeProperty::SPACING_MARK | GraphemeProperty::ZWJ => {
                        self.state = CcsBase;
                        Break::None
                    }
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            }
            CcsExtend => {
                match property {
                    GraphemeProperty::EXTEND
                    | GraphemeProperty::SPACING_MARK
                    | GraphemeProperty::ZWJ => Break::None,
                    _ => Break::Before
                }
            }
            Flag => {
                self.state = Start;
                match property {
                    GraphemeProperty::REGIONAL_INDICATOR => {
                        self.state = Other;
                        Break::None
                    }
                    GraphemeProperty::EXTEND
                    | GraphemeProperty::SPACING_MARK
                    | GraphemeProperty::ZWJ => {
                        self.state = CcsExtend;
                        Break::None
                    }
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            }
            Emoji => {
                match property {
                    GraphemeProperty::ZWJ => {
                        self.state = EmojiZWJ;
                        Break::None
                    }
                    GraphemeProperty::EXTEND | GraphemeProperty::SPACING_MARK => {
                        self.state = Emoji;
                        Break::None
                    }
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            }
            EmojiZWJ => {
                if property == GraphemeProperty::EXTENDED_GRAPHEME {
                    self.state = Emoji;
                    Break::None
                } else {
                    Break::Before
                }
            }
            CrLf => Break::Before,
            _ => {
                if is_continuation(property) {
                    Break::None
                } else {
                    self.first_character(c);
                    Break::Before
                }
            }
        }
    }
    #[inline]
    fn first_character(&mut self, c: char) -> Break {
        if c == '\r' {
            self.state = CrLf;
            return Break::None;
        }
        let property = get_property(c);
        if property == GraphemeProperty::CONTROL {
            self.state = Start;
            return Break::After;
        }
        match property {
            GraphemeProperty::PREPEND => {
                self.state = Precore;
            }
            GraphemeProperty::EXTEND => {
                self.state = CcsExtend;
            }
            GraphemeProperty::SPACING_MARK => {
                self.state = CcsExtend;
            }
            GraphemeProperty::L => {
                self.state = HangulSyllableL;
            }
            GraphemeProperty::V => {
                self.state = HangulSyllableV;
            }
            GraphemeProperty::T => {
                self.state = HangulSyllableT;
            }
            GraphemeProperty::LV => {
                self.state = HangulSyllableV;
            }
            GraphemeProperty::LVT => {
                self.state = HangulSyllableT;
            }
            // GraphemeProperty::ZWJ  => {
            //     self.state = Start;
            //     return Break::After
            // },
            GraphemeProperty::EXTENDED_GRAPHEME => {
                self.state = Emoji;
            }
            GraphemeProperty::REGIONAL_INDICATOR => {
                self.state = Flag;
            }
            _ => {
                self.state = Other;
            }
        }
        Break::None
    }
}

#[inline]
fn is_continuation(property: u8) -> bool {
    property != 0 && property & 0xc == 0
}

#[inline]
fn is_hangul(property: u8) -> bool {
    property & 0x8 != 0
}

struct GraphemeProperty {}

// must continue: 1 2 3
//    EXTEND SPACING_MARK ZWJ
// Hangul 8-12
//    L 0xc V 0x8 T 0x9 LV 0xd LVT 0xe
// CONTROL includes CR/LF 4
// PREPEND 5
// EXTENDED_GRAPHEME 6
// OTHER 0
impl GraphemeProperty {
    const OTHER: u8 = 0x00;
    const EXTEND: u8 = 0x01;
    const SPACING_MARK: u8 = 0x02;
    const ZWJ: u8 = 0x03;
    const CONTROL: u8 = 0x04;
    const PREPEND: u8 = 0x05;
    const EXTENDED_GRAPHEME: u8 = 0x06;
    const REGIONAL_INDICATOR: u8 = 0x07;
    const L: u8 = 0x0c;
    const V: u8 = 0x08;
    const T: u8 = 0x09;
    const LV: u8 = 0x0d;
    const LVT: u8 = 0x0e;
}

enum Either {
    Code(u8),
    Page(u16),
}

impl Either {
    #[inline]
    pub fn get_code(&self, index: u8) -> u8 {
        match self {
            &Either::Code(code) => code,
            &Either::Page(page) => GP_PAGES[usize::from(page)][usize::from(index)]
        }
    }
}

#[inline]
fn get_property(c: char) -> u8 {
    GP_TABLE[c as usize >> 8]
        .get_code(c as u8)
}


include!(concat!(env!("OUT_DIR"), "/grapheme_property.rs"));



#[cfg(test)]
mod tests {
    use crate::grapheme_clusters::*;

    #[test]
    fn low_level_interface_test() {
        let mut machine = ClusterMachine::new();
        assert_eq!(machine.find_cluster('\r'), Break::None);
        assert_eq!(machine.find_cluster('a'), Break::Before);
        assert_eq!(machine.find_cluster('\r'), Break::Before);
        assert_eq!(machine.find_cluster('\n'), Break::After);
    }

    #[test]
    fn can_get_clusters() {
        let mut peekable_index = "\r\ne\u{301}f".char_indices().peekable();
        assert_eq!(Some("\r\n".to_string()), peekable_index.next_cluster());
        assert_eq!(Some("e\u{301}".to_string()), peekable_index.next_cluster());
        assert_eq!(Some("f".to_string()), peekable_index.next_cluster());
    }

    fn grapheme_test(input: &str, expected_output: &[&str], message: &str) {
        let mut iter = input.char_indices().peekable();
        let mut clusters = vec!();
        while let Some(cluster) = iter.next_cluster() {
            clusters.push(cluster);
        }
        assert_eq!(clusters.len(), expected_output.len(), "Lengths did not match on Grapheme Cluster\n\t{message}\n\tOutput: {clusters:?}\n\tExpected: {expected_output:?}");
        clusters.iter().zip(expected_output.into_iter())
            .for_each(|(actual, &expected)| assert_eq!(actual.as_str(), expected, "GraphemeCluster mismatch: {message}"));

        let iter = Graphemes::new(input);
        let clusters = iter.collect::<Vec<&str>>();
        assert_eq!(clusters.len(), expected_output.len(), "Lengths did not match on Grapheme Cluster Indices\n\t{message}\n\tOutput: {clusters:?}\n\tExpected: {expected_output:?}");
        clusters.iter().zip(expected_output.into_iter())
            .for_each(|(actual, &expected)| assert_eq!(*actual, expected, "Grapheme cluster indices mismatch: {message}\n{} ≠ {}", actual.escape_unicode(), expected.escape_unicode()));
    }

    #[test]
    fn big_master_test() {
        include!(concat!(env!("OUT_DIR"), "/grapheme_test.rs"));
    }
}