
/// Trait to provide methods that provide boolean tests on most Unicode character categories.
///
/// There is no `is_surrogate()` method since surrogate character codes are not valid values
/// for a Rust `char`.
///
/// Importing the trait will provide the methods on the `char` type.
///
/// Character codes are determined using a multistage table approach which allows constant-time
/// determination of character codes. Some special tricks are employed to enable fast determination
/// of composite classes (L, LC, M, N, P, S, Z, C) without requiring a check for each individual
/// sub-class. This is managed by storing character categories encoded as two-nibble sequences
/// in a single byte. The first nibble gives the composite class, the second the particular
/// sub class within the composite class. Classes M, N, P, S, Z and C use nibble values in the range
/// 0..7 which allows us to use 9 for LC and 8 for the non-casing letter classes so we can simply
/// verify that `code & 0x80 == 0x80` for all letters while using `code & 0xFF` to test all other
/// classes.
///
/// Important note: For efficiency's sake, unassigned character codes which occur in a 256-character
/// page in which
/// all other characters have the same code will not be identified as unassigned but will instead
/// identify as the same code as the other characters in their page. For example, `\u{0x1311}` (among
/// others) is an unassigned code point in the Ethiopic block. All code points in the range
/// `0x1300..0x13ff` are either unassigned or letters, so the unassigned code points will identify
/// as letters (Lo to be precise). This slight inaccuracy allows faster identification of the
/// categories of the assigned categories in that page as well as reducing the memory requirements
/// for the tables.
///

pub trait CharacterCategories {
    /// Determines whether a character is class L, letter (Lu, Ll, Lt, Lm, Lo).
    fn is_letter(self) -> bool;
    /// Determines whether a character is class LC, cased letter (Lu, Ll, Lt)
    fn is_cased_letter(self) -> bool;
    /// Determines whether a character is an upper-case letter (Lu)
    fn is_uppercase_letter(self) -> bool;
    /// Determines whether a character is a lower-case letter (Ll)
    fn is_lowercase_letter(self) -> bool;
    /// Determines whether a character is a title-case letter (Lt)
    ///
    /// **Inquiry** Is there a use for being able to verify that something is Lu *or* Lt? A bit-mask
    /// is possible to avoid the logical or and would be mildly faster.
    fn is_titlecase_letter(self) -> bool;
    /// Determines whether a character is a modifier letter (Lm)
    fn is_modifier_letter(self) -> bool;
    /// Determines whether a character is an other letter (Lo)
    fn is_other_letter(self) -> bool;
    /// Determines whether a character is a mark (M)
    fn is_mark(self) -> bool;
    /// Determines whether a character is a nonspacing mark (Mn)
    fn is_nonspacing_mark(self) -> bool;
    /// Determines whether a character is a spacing mark (Mc)
    fn is_spacing_mark(self) -> bool;
    /// Determines whether a character is an enclosing mark (Me)
    fn is_enclosing_mark(self) -> bool;
    /// Determines whether a character is a number (N)
    fn is_number(self) -> bool;
    /// Determines whether a character is a decimal digit (Nd)
    fn is_decimal_number(self) -> bool;
    /// Determines whether a character is a letterlike numeric character (Nl)
    fn is_letter_number(self) -> bool;
    /// Determines whether a character is an other numeric character (No)
    fn is_other_number(self) -> bool;
    /// Determines whether a character is a punctuation character (P)
    fn is_punctuation(self) -> bool;
    /// Determines whether a character is a connector punctuation character (Pc)
    fn is_connector_punctuation(self) -> bool;
    /// Determines whether a character is a dash punctuation character (Pd)
    fn is_dash_punctuation(self) -> bool;
    /// Determines whether a character is an open punctuation character (Ps)
    fn is_open_punctuation(self) -> bool;
    /// Determines whether a character is a close punctuation character (Pe)
    fn is_close_punctuation(self) -> bool;
    /// Determines whether a character is an initial punctuation character (Pi)
    fn is_initial_punctuation(self) -> bool;
    /// Determines whether a character is a final punctuation character (Pf)
    fn is_final_punctuation(self) -> bool;
    /// Determines whether a character is an other punctuation character (Po)
    fn is_other_punctuation(self) -> bool;
    /// Determines whether a character is a symbol (S)
    fn is_symbol(self) -> bool;
    /// Determines whether a character is a math symbol (Sm)
    fn is_math_symbol(self) -> bool;
    /// Determines whether a character is a currency symbol (Sc)
    fn is_currency_symbol(self) -> bool;
    /// Determines whether a character is a modifier symbol (Sk)
    fn is_modifier_symbol(self) -> bool;
    /// Determines whether a character is an other symbol (So)
    fn is_other_symbol(self) -> bool;
    /// Determines whether a character is a separator (Z)
    fn is_separator(self) -> bool;
    /// Determines whether a character is a space separator (Zs)
    fn is_space_separator(self) -> bool;
    /// Determines whether a character is a line separator (Zl)
    fn is_line_separator(self) -> bool;
    /// Determines whether a character is a paragraph separator (Zp)
    fn is_paragraph_separator(self) -> bool;
    /// Determines whether a character is an other character (C)
    fn is_other(self) -> bool;
    /// Determines whether a character is a control character (Cc)
    fn is_control(self) -> bool;
    /// Determines whether a character is a firnat character (Cf)
    fn is_format(self) -> bool;
    /// Determines whether a character is a private use character (Co)
    fn is_private_use(self) -> bool;
    /// Determines whether a character is unassigned (Cn)
    fn is_unassigned(self) -> bool;
}

struct Cat;
impl Cat {
    const Lu: u8 = 0x90;
    const Ll: u8 = 0x91;
    const Lt: u8 = 0x92;
    const LC: u8 = 0x90;
    const Lm: u8 = 0x83;
    const Lo: u8 = 0x84;
    const L:  u8 = 0x80;
    const Mn: u8 = 0x10;
    const Mc: u8 = 0x11;
    const Me: u8 = 0x12;
    const M:  u8 = 0x10;
    const Nd: u8 = 0x20;
    const Nl: u8 = 0x21;
    const No: u8 = 0x22;
    const N:  u8 = 0x20;
    const Pc: u8 = 0x30;
    const Pd: u8 = 0x31;
    const Ps: u8 = 0x32;
    const Pe: u8 = 0x33;
    const Pi: u8 = 0x34;
    const Pf: u8 = 0x35;
    const Po: u8 = 0x36;
    const P:  u8 = 0x30;
    const Sm: u8 = 0x40;
    const Sc: u8 = 0x41;
    const Sk: u8 = 0x42;
    const So: u8 = 0x43;
    const S:  u8 = 0x40;
    const Zs: u8 = 0x50;
    const Zl: u8 = 0x51;
    const Zp: u8 = 0x52;
    const Z:  u8 = 0x50;
    const Cc: u8 = 0x01;
    const Cf: u8 = 0x02;
    //    const Cs: u8 = 0x03;
    const Co: u8 = 0x04;
    const Cn: u8 = 0x00;
    const C:  u8 = 0x00;
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
            &Either::Page(page) => CAT_PAGES[usize::from(page)][usize::from(index)]
        }
    }
}
#[inline]
fn get_code(c: char) -> u8 {
    CAT_TABLE[c as usize >> 8]
        .get_code(c as u8)
}
include!(concat!(env!("OUT_DIR"), "/characters.rs"));


impl CharacterCategories for char {
    #[inline]
    fn is_letter(self) -> bool {
        get_code(self) & Cat::L == Cat::L
    }

    #[inline]
    fn is_cased_letter(self) -> bool {
        get_code(self) & 0xf0 == Cat::LC
    }

    #[inline]
    fn is_uppercase_letter(self) -> bool {
        get_code(self) == Cat::Lu
    }

    #[inline]
    fn is_lowercase_letter(self) -> bool {
        get_code(self) == Cat::Ll
    }

    #[inline]
    fn is_titlecase_letter(self) -> bool {
        get_code(self) == Cat::Lt
    }

    #[inline]
    fn is_modifier_letter(self) -> bool {
        get_code(self) == Cat::Lm
    }

    #[inline]
    fn is_other_letter(self) -> bool {
        get_code(self) == Cat::Lo
    }

    #[inline]
    fn is_mark(self) -> bool {
        get_code(self) & 0xf0 == Cat::M
    }

    #[inline]
    fn is_nonspacing_mark(self) -> bool {
        get_code(self) == Cat::Mn
    }

    #[inline]
    fn is_spacing_mark(self) -> bool {
        get_code(self) == Cat::Mc
    }

    #[inline]
    fn is_enclosing_mark(self) -> bool {
        get_code(self) == Cat::Me
    }

    #[inline]
    fn is_number(self) -> bool {
        get_code(self) & 0xf0 == Cat::N
    }

    #[inline]
    fn is_decimal_number(self) -> bool {
        get_code(self) == Cat::Nd
    }

    #[inline]
    fn is_letter_number(self) -> bool {
        get_code(self) == Cat::Nl
    }

    #[inline]
    fn is_other_number(self) -> bool {
        get_code(self) == Cat::No
    }

    #[inline]
    fn is_punctuation(self) -> bool {
        get_code(self) & 0xf0 == Cat::P
    }

    #[inline]
    fn is_connector_punctuation(self) -> bool {
        get_code(self) == Cat::Pc
    }

    #[inline]
    fn is_dash_punctuation(self) -> bool {
        get_code(self) == Cat::Pd
    }

    #[inline]
    fn is_open_punctuation(self) -> bool {
        get_code(self) == Cat::Ps
    }

    #[inline]
    fn is_close_punctuation(self) -> bool {
        get_code(self) == Cat::Pe
    }

    #[inline]
    fn is_initial_punctuation(self) -> bool {
        get_code(self) == Cat::Pi
    }

    #[inline]
    fn is_final_punctuation(self) -> bool {
        get_code(self) == Cat::Pf
    }

    #[inline]
    fn is_other_punctuation(self) -> bool {
        get_code(self) == Cat::Po
    }

    #[inline]
    fn is_symbol(self) -> bool {
        get_code(self) & 0xf0 == Cat::S
    }

    #[inline]
    fn is_math_symbol(self) -> bool {
        get_code(self) == Cat::Sm
    }

    #[inline]
    fn is_currency_symbol(self) -> bool {
        get_code(self) == Cat::Sc
    }

    #[inline]
    fn is_modifier_symbol(self) -> bool {
        get_code(self) == Cat::Sk
    }

    #[inline]
    fn is_other_symbol(self) -> bool {
        get_code(self) == Cat::So
    }

    #[inline]
    fn is_separator(self) -> bool {
        get_code(self) & 0xf0 == Cat::Z
    }

    #[inline]
    fn is_space_separator(self) -> bool {
        get_code(self) == Cat::Zs
    }

    #[inline]
    fn is_line_separator(self) -> bool {
        get_code(self) == Cat::Zl
    }

    #[inline]
    fn is_paragraph_separator(self) -> bool {
        get_code(self) == Cat::Zp
    }

    #[inline]
    fn is_other(self) -> bool {
        get_code(self) & 0xf0 == Cat::C
    }

    #[inline]
    fn is_control(self) -> bool {
        get_code(self) == Cat::Cc
    }

    #[inline]
    fn is_format(self) -> bool {
        get_code(self) == Cat::Cf
    }
    
    #[inline]
    fn is_private_use(self) -> bool {
        get_code(self) == Cat::Co
    }

    #[inline]
    fn is_unassigned(self) -> bool {
        get_code(self) == Cat::Cn
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use crate::categories::*;

    #[test]
    fn character_categories() {
        println!("{}", mem::size_of::<Either>());
        assert!('a'.is_letter());
        assert!(!'a'.is_uppercase_letter());
        assert!('Ü'.is_uppercase_letter());
        assert!('Я'.is_uppercase_letter());

        // test data from http://www.i18nguy.com/unicode/supplementary-test.html
        "𠜎 𠜱 𠝹 𠱓 𠱸 𠲖 𠳏 𠳕 𠴕 𠵼 𠵿 𠸎 𠸏 𠹷 𠺝 𠺢 𠻗 𠻹 𠻺 𠼭 𠼮 𠽌 𠾴 𠾼 𠿪 𡁜 𡁯 𡁵 𡁶 𡁻 𡃁 𡃉 𡇙 𢃇 𢞵 𢫕 𢭃 𢯊 𢱑 𢱕 𢳂 𢴈 𢵌 𢵧 𢺳 𣲷 𤓓 𤶸 𤷪 𥄫 𦉘 𦟌 𦧲 𦧺 𧨾 𨅝 𨈇 𨋢 𨳊 𨳍 𨳒 𩶘".chars()
            .filter(|&c| c != ' ')
            .for_each(
                |c| {
                    assert!(c.is_letter());
                    assert!(c.is_other_letter());
                    assert!(!c.is_cased_letter());
                }
            );

        // test data from https://en.wikipedia.org/wiki/Module:Unicode_data/testcases
        assert!('\t'.is_control());
        assert!(' '.is_space_separator());
        assert!('['.is_open_punctuation(), "Got character code of {}", get_code('['));
        assert!(']'.is_close_punctuation());
        assert!('^'.is_modifier_symbol());
        assert!('A'.is_uppercase_letter());
        assert!('\u{00AD}'.is_format());
        assert!('¾'.is_other_number());
        assert!('«'.is_initial_punctuation());
        assert!('»'.is_final_punctuation());
        assert!('\u{0300}'.is_nonspacing_mark());
        assert!('\u{0488}'.is_enclosing_mark());
        assert!('٣'.is_decimal_number());
        assert!('子'.is_other_letter());
        assert!('ᾮ'.is_titlecase_letter());
        assert!('\u{1B44}'.is_spacing_mark());
        assert!('∈'.is_math_symbol());
        assert!('‿'.is_connector_punctuation());
        assert!('↹'.is_other_symbol());
        assert!('⸗'.is_dash_punctuation());
        assert!('Ⅷ'.is_letter_number());
        assert!('\u{2028}'.is_line_separator());
        assert!('\u{2029}'.is_paragraph_separator());
        assert!('ゞ'.is_modifier_letter());
        //assert!('\u{D800}'.is_surrogate());
        assert!('￡'.is_currency_symbol());
        assert!('\u{FFFF}'.is_unassigned());
        assert!('\u{100000}'.is_private_use());

    }

}
