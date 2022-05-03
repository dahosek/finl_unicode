use std::iter::Peekable;
use std::str::CharIndices;
// use crate::grapheme_clusters::ClusterCategory::{Extend, Prepend, SpacingMark, ZWJExtCccZwj};
use crate::grapheme_clusters::ClusterMachineState::{CcsExtend, CrLf, HangulSyllableL, HangulSyllableT, HangulSyllableV, Precore, Start, CcsBase, Flag, Emoji, EmojiZWJ, Other};


/// Get the next grapheme cluster from a stream of characters or char indices
///
pub trait GraphemeCluster {
    fn next_cluster(&mut self) -> Option<String>;
}

/// We implement the trait for the struct `Peekable<CharIndices>`.
impl<'a> GraphemeCluster for Peekable<CharIndices<'a>> {
    fn next_cluster(&mut self) -> Option<String> {
        if self.peek().is_some() {
            let mut clusterMachine = ClusterMachine::new();
            let mut rv = String::new();
            loop {
                if let Some(&(_, ch)) = self.peek() {
                let state = clusterMachine.find_cluster(ch);
                match state {
                    Break::None => { rv.push(ch); self.next(); }
                    Break::Before => { return Some(rv) }
                    Break::After => { rv.push(ch); self.next(); return Some(rv) }
                }
                }
                else {
                    break
                }
            }
            Some(rv)
        }
        else {
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
    Other
}

#[derive(Debug,PartialEq)]
enum Break {
    None,
    Before,
    After
}

struct ClusterMachine {
    state: ClusterMachineState,
}

impl ClusterMachine {
    pub fn new() -> ClusterMachine {
        ClusterMachine {
            state: ClusterMachineState::Start,
        }
    }

    /// If we have a cluster, we return the cluster in a `String` in an `Option` long with a `bool`
    /// If the `bool` is true, it means that we are also consuming the character  in the cluster.
    pub fn find_cluster(&mut self, c: char) -> Break {
        if self.state == Start {
            return self.first_character(c);
        }
        let property = get_property(c);

        if property == GraphemeProperty::Control {
            if self.state == CrLf && c == '\n' {
                self.state == ClusterMachineState::Start;
                return Break::After
            }
            else {
                if c == '\r' {
                    self.state == CrLf;
                }
                else {
                    self.state == Start;
                }
                return Break::Before
            }
        }

        match self.state {
            Start => self.first_character(c),
            Precore => {
                self.first_character(c);
                Break::None
            },

            HangulSyllableL => {
                match property {
                    GraphemeProperty::L => Break::None,
                    GraphemeProperty::V | GraphemeProperty::LV => {
                        self.state = HangulSyllableV;
                         Break::None
                    },
                    GraphemeProperty::LVT => {
                        self.state = HangulSyllableT;
                        Break::None
                    }
                    GraphemeProperty::Extend | GraphemeProperty::SpacingMark | GraphemeProperty::ZWJ => {
                        self.state = CcsBase;
                        Break::None
                    },
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            },
            HangulSyllableV => {
                match property {
                    GraphemeProperty::V => Break::None,
                    GraphemeProperty::T => {
                        self.state = HangulSyllableT;
                        Break::None
                    },
                    GraphemeProperty::Extend | GraphemeProperty::SpacingMark | GraphemeProperty::ZWJ => {
                        self.state = CcsBase;
                        Break::None
                    },
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            },
            HangulSyllableT => {
                match property {
                    GraphemeProperty::T => Break::None,
                    GraphemeProperty::Extend | GraphemeProperty::SpacingMark | GraphemeProperty::ZWJ => {
                        self.state = CcsBase;
                        Break::None
                    },
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            },
            CcsExtend => {
                match property {
                    GraphemeProperty::Extend
                        | GraphemeProperty::SpacingMark
                        | GraphemeProperty::ZWJ => Break::None,
                    _ => Break::Before
                }
            },
            Flag => {
                self.state == Start;
                match property {
                    GraphemeProperty::RegionalIndicator => {
                        self.state = Other;
                        Break::None
                    },
                    GraphemeProperty::Extend
                        | GraphemeProperty::SpacingMark
                        | GraphemeProperty::ZWJ => {
                        self.state = CcsExtend;
                        Break::None
                    },
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            },
            Emoji => {
                match property {
                    GraphemeProperty::ZWJ => {
                        self.state = EmojiZWJ;
                        Break::None
                    }
                    GraphemeProperty::Extend | GraphemeProperty::SpacingMark => {
                        self.state = Emoji;
                        Break::None
                    }
                    _ => {
                        self.first_character(c);
                        Break::Before
                    }
                }
            },
            EmojiZWJ => {
                if property == GraphemeProperty::ExtendedGrapheme {
                    self.state = Emoji;
                    Break::None
                } else {
                    Break::Before
                }
            },
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
    fn first_character(&mut self, c: char) -> Break {
        if c == '\r' {
            self.state = CrLf;
            return Break::None;
        }
        let property = get_property(c);
        if property == GraphemeProperty::Control {
            self.state == Start;
            return Break::After
        }
        match property {
            GraphemeProperty::Prepend => {
                self.state = Precore;
            },
            GraphemeProperty::Extend  => {
                self.state = CcsExtend;
            },
            GraphemeProperty::SpacingMark  => {
                self.state = CcsExtend;
            },
            GraphemeProperty::L  => {
                self.state = HangulSyllableL;
            },
            GraphemeProperty::V  => {
                self.state = HangulSyllableV;
            },
            GraphemeProperty::T  => {
                self.state = HangulSyllableT;
            },
            GraphemeProperty::LV  => {
                self.state = HangulSyllableV;
            },
            GraphemeProperty::LVT  => {
                self.state = HangulSyllableT;
            },
            // GraphemeProperty::ZWJ  => {
            //     self.state = Start;
            //     return Break::After
            // },
            GraphemeProperty::ExtendedGrapheme  => {
                self.state = Emoji;
            },
            GraphemeProperty::RegionalIndicator => {
                self.state = Flag;
            }
            _ => {
                self.state = Other;
            },
        }
        Break::None
    }
}

#[inline]
fn is_continuation(property: u8) -> bool {
    property !=0 && property & 0xc == 0
}

#[inline]
fn is_hangul(property: u8) -> bool {
    property & 0x8 != 0
}

struct GraphemeProperty {}

// must continue: 1 2 3
//    Extend SpacingMark ZWJ
// Hangul 8-12
//    L 0xc V 0x8 T 0x9 LV 0xd LVT 0xe
// Control includes CR/LF 4
// Prepend 5
// ExtendedGrapheme 6
// Other 0
impl GraphemeProperty {
    const Other: u8 =0x00;
    const Extend :u8 = 0x01;
    const SpacingMark :u8 = 0x02;
    const ZWJ :u8 = 0x03;
    const Control :u8 = 0x04;
    const Prepend: u8 = 0x05;
    const ExtendedGrapheme :u8 = 0x06;
    const RegionalIndicator:u8 = 0x07;
    const L :u8 = 0x0c;
    const V :u8 = 0x08;
    const T :u8 = 0x09;
    const LV :u8 = 0x0d;
    const LVT :u8 = 0x0e;
}

enum Either {
    Code(u8),
    Page(u16)
}

impl Either {
    #[inline]
    pub fn get_code(&self, index:u8) -> u8 {
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
        assert_eq!(clusters.len(), expected_output.len(), "Lengths did not match\n\t{message}\n\tOutput: {clusters:?}\n\tExpected: {expected_output:?}");
        clusters.iter().zip(expected_output.into_iter())
            .for_each(|(actual, &expected)| assert_eq!(actual.as_str(), expected));
    }

    #[test]
    fn big_master_test() {
        include!(concat!(env!("OUT_DIR"), "/grapheme_test.rs"));
    }
}