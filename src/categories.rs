//! The code in this module provides a trait that is implemented against `char` that allows testing
//! or retrieving the Unicode category for the character as well as two `enum`s for identifying
//! character classes.

use crate::data::characters::{CAT_PAGES, CAT_TABLE};

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
/// sub-class.
// `char` is `Copy`, so taking `self` by value is the idiomatic receiver here (as in std's own
// `char::is_alphabetic` etc.), whatever clippy's naming heuristic for `is_*` methods says.
#[allow(clippy::wrong_self_convention)]
pub trait CharacterCategories {
    /// Determines whether a character is class L, letter (Lu, Ll, Lt, Lm, Lo). This includes all
    /// characters used for word formation, both phonetic and ideograms. It does not include symbols
    /// that look like letters but are really numbers, such as the encoded Roman numerals.
    fn is_letter(self) -> bool;
    /// Determines whether a character is class LC, cased letter (Lu, Ll, Lt). This will include
    /// letters in the Adlam, Armenian,
    /// Cherokee, Coptic,  Cyrillic, Deseret,
    /// Georgian (Mtavruli), Glagolitic, Greek, Medefaidrin, Old Hungarian, Osage,
    /// Roman, Vithkuqi and Warang Citi alphabets (including their extensions) along with
    /// special math versions of letters including the Angstrom symbol and Ohm symbol and other
    /// such characters like ℇ, ℑ, ℕ, or Ↄ.
    fn is_letter_cased(self) -> bool;
    /// Determines whether a character is an upper-case letter (Lu)
    fn is_letter_uppercase(self) -> bool;
    /// Determines whether a character is a lower-case letter (Ll)
    fn is_letter_lowercase(self) -> bool;
    /// Determines whether a character is a title-case letter (Lt). This is composed of
    /// Latin digraphs such as ǅ and Greek capital letters with the iota subscript which when
    /// uppercased and rendered with prosgegrammeni should be printed as, e.g., ΑΙ but in titlecase
    /// would be Αι (although other possibilities for rendering both may occur depending on the typeface.
    ///
    /// **Inquiry** Is there a use for being able to verify that something is Lu *or* Lt? A bit-mask
    /// is possible to avoid the logical or and would be mildly faster.
    fn is_letter_titlecase(self) -> bool;
    /// Determines whether a character is a modifier letter (Lm). These are letters or symbols typically
    /// written adjacent to other letters which modify their usage in some way. They are mostly used
    /// in phonetic contexts
    fn is_letter_modifier(self) -> bool;
    /// Determines whether a character is an other letter (Lo). This is all the characters from uncased
    /// scripts alongside a handful of miscellaneous uncased letters such as ª or ƻ.
    fn is_letter_other(self) -> bool;
    /// Determines whether a character is a mark (M). These are characters that combine with other characters in
    /// various ways (e.g., for accents or other modifiers). A character plus a mark is generally viewed as a single grapheme.
    /// **Note** for most cases you will want to use grapheme segmentation to identify individual graphemes and
    /// word segmentation to identify word boundaries rather than examining categories to find word/grapheme
    /// boundaries.
    fn is_mark(self) -> bool;
    /// Determines whether a character is a nonspacing mark (Mn). This is largely diacritics which will
    /// not change the width of the character even if they are placed before or after the base character
    /// (as is the case with notation for Znamenny chant).
    fn is_mark_nonspacing(self) -> bool;
    /// Determines whether a character is a spacing mark (Mc).This is largely used by some scripts derived
    /// from the classical Brahmic script where spacing modifiers are added to consonants to indicate
    /// vowel sounds and for some musical symbols.
    fn is_mark_spacing(self) -> bool;
    /// Determines whether a character is an enclosing mark (Me). This is a small category of combining
    /// marks like ㅤ⃠ which will surround their modified character, e.g., z⃠ which *may* be rendered
    /// as a slashed circle over a z.
    fn is_mark_enclosing(self) -> bool;
    /// Determines if the category is either L or M (this is used by finl to determine whether to continue
    /// a sequence for a named command. If you’re tempted to use this, ask whether you really want this
    /// or if you’re better off using grapheme or word segmentation or UAX31 identifier and pattern
    /// syntax.
    fn is_letter_or_mark(self) -> bool;
    /// Determines whether a character is a number (N). This includes digits in various scripts along with
    /// special representations of numbers like Unicode encodings of Roman numerals or decorated digits
    /// like 3︎⃣ and renderings of fractions.
    fn is_number(self) -> bool;
    /// Determines whether a character is a decimal digit (Nd). This includes the expected 0–9 digits
    /// along with their equivalents in other scripts that use decimal digits, but potentially with
    /// other forms plus some special representations of the digits for mathematics and legacy encodings
    /// (the last include double-struck digits like 𝟚 and LED-style digits like 🯴 (U+1FBF4).
    fn is_number_decimal(self) -> bool;
    /// Determines whether a character is a letterlike numeric character (Nl). This includes Unicode
    /// representation of Roman numerals as well as special characters for non-place value numbering
    /// systems in other scripts (e.g., Greek letter-based numbers, Gothic, Old Persian, Cuneiform, etc.)
    fn is_number_letter(self) -> bool;
    /// Determines whether a character is an other numeric character (No). This includes fractions
    /// in Hindu-Arabic (Western) and other scripts, super- and subscripted digits and decorated
    /// digits along with digits from numbering systems which are not decimal but also not alphabetical
    /// in their form like Old Hungarian or Sogdian.
    fn is_number_other(self) -> bool;
    /// Determines whether a character is a punctuation character (P)
    fn is_punctuation(self) -> bool;
    /// Determines whether a character is a connector punctuation character (Pc). These include
    /// the underscore character and CJK variations on that character.
    fn is_punctuation_connector(self) -> bool;
    /// Determines whether a character is a dash punctuation character (Pd). This includes the hyphen,
    /// typographical dashes and variations for non-Latin scripts.
    fn is_punctuation_dash(self) -> bool;
    /// Determines whether a character is an open punctuation character (Ps). This includes all the
    /// variations on parentheses, braces and brackets in different scripts including mathematical
    /// and computational special symbols along with the low-9
    /// single and double quotation marks (‚ and „) but *not* other quotation marks.
    fn is_punctuation_open(self) -> bool;
    /// Determines whether a character is a close punctuation character (Pe). This includes all the
    /// variations on parentheses, braces and brackets in different scripts including mathematical
    /// and computational special symbols.
    fn is_punctuation_close(self) -> bool;
    /// Determines whether a character is an initial punctuation character (Pi). This includes variations
    /// on opening quotation marks, functionally similar symbols from the New Testament editorial
    /// symbols set and the left vertical bar with quill (which is apparently meant to be used to
    /// build extended versions of ⁅ if I understand the reference in the Unicode character chart).
    fn is_punctuation_initial(self) -> bool;
    /// Determines whether a character is a final punctuation character (Pf). This includes variations
    /// on closing quotation marks, functionally similar symbols from the New Testament editorial
    /// symbols set and the right vertical bar with quill (which is apparently meant to be used to
    /// build extended versions of ⁆ if I understand the reference in the Unicode character chart).
    fn is_punctuation_final(self) -> bool;
    /// Determines whether a character is an other punctuation character (Po). This is all the punctuation
    /// symbols which are not part of the above set.
    fn is_punctuation_other(self) -> bool;
    /// Determines whether a character is a symbol (S). These are non-punctuation symbols including
    /// currency and mathematical symbols as well as dingbats and emoji.
    fn is_symbol(self) -> bool;
    /// Determines whether a character is a math symbol (Sm).
    fn is_symbol_math(self) -> bool;
    /// Determines whether a character is a currency symbol (Sc)
    fn is_symbol_currency(self) -> bool;
    /// Determines whether a character is a modifier symbol (Sk). This is mostly stand-alone versions
    /// of combining accents, along with the Emoji skintone modifiers.
    fn is_symbol_modifier(self) -> bool;
    /// Determines whether a character is an other symbol (So). This is all the symbols which neither
    /// have mathematical uses nor are currency symbols.
    fn is_symbol_other(self) -> bool;
    /// Determines whether a character is a separator (Z). This is all spaces plus the Unicode line
    /// and paragraph separators.
    fn is_separator(self) -> bool;
    /// Determines whether a character is a space separator (Zs). In addition to the standard space
    /// and the non-breaking space (U+A0), there are various typographic spaces and the Ogham space
    /// mark but not printed spaces from other scripts such as the Ge'ez word space (፡).
    fn is_separator_space(self) -> bool;
    /// Determines whether a character is a line separator (Zl). This is the Unicode code point U+2028.
    fn is_separator_line(self) -> bool;
    /// Determines whether a character is a paragraph separator (Zp). This is the Unicode code point U+2029.
    fn is_separator_paragraph(self) -> bool;
    /// Determines whether a character is an other character (C). These consist of control characters,
    /// format characters, surrogates (no test is provided for these since they will not appear in
    /// valid UTF-8 text), unassigned characters and private use characters.
    fn is_other(self) -> bool;
    /// Determines whether a character is a control character (Cc). These are the 65 characters in the
    /// ranges 0x00–0x1f and 0x7f–0x9f.
    ///
    /// **Deprecated:** use the standard library’s [`char::is_control`] instead. Because inherent
    /// methods take precedence over trait methods, `c.is_control()` already calls the standard
    /// library version even with this trait imported; this method is only reached through
    /// `CharacterCategories::is_control(c)`. It now simply delegates to `char::is_control`, which
    /// is safe because the Unicode stability policy guarantees the set of Cc characters will never
    /// change.
    #[deprecated(since = "1.5.1", note = "use the standard library's `char::is_control` instead, which `c.is_control()` already resolves to")]
    fn is_control(self) -> bool;
    /// Determines whether a character is a format character (Cf). This is a mix of non-printing characters
    /// like soft hyphen (U+00AD) and Zero Width Joiner (ZWJ—U+200D) and the tag block which is used
    /// in Emoji sequences for flags along with some printing characters for non-Latin script (mostly
    /// Arabic but also Mongolian).
    fn is_format(self) -> bool;
    /// Determines whether a character is a private use character (Co)
    fn is_private_use(self) -> bool;
    /// Determines whether a character is unassigned (Cn)
    fn is_unassigned(self) -> bool;
    /// Get the major category for a character (L, M, N, P, S, Z or C). Note that the “special” category
    /// of LC cannot be returned by this method since it is not a mutually exclusive category.
    fn get_major_category(self) -> MajorCategory;
    /// get the minor category for a character (Lu, Ll, Lt, Lm, Lo,
    /// Mn, Mc, Me,
    /// Nd, Nl, No,
    /// Pc, Pd, Ps, Pi, Pf, Po,
    /// Sm, Sk, Sc, So,
    /// Zs, Zl, Zp,
    /// Cc, Cf, Co, Cn). Note that the “special” category
    /// of LC cannot be returned by this method since it is not a mutually exclusive category.
    fn get_minor_category(self) -> MinorCategory;
}

/// enum for distinguishing Unicode minor categories of characters
#[derive(PartialEq, Debug)]
pub enum MinorCategory {
    /// Uppercase letter
    Lu,
    /// Lowercase letter
    Ll,
    /// Titlecase letter
    Lt,
    /// Modifier letter
    Lm,
    /// Other letter
    Lo,
    /// Non-spacing mark
    Mn,
    /// Spacing mark
    Mc,
    /// Enclosing mark
    Me,
    /// Decimal number
    Nd,
    /// Letterlike number
    Nl,
    /// Other number
    No,
    /// Connector punctuation
    Pc,
    /// Dash punctuation
    Pd,
    /// Opening punctuation
    Ps,
    /// Closing punctuation
    Pe,
    /// Initial punctuation
    Pi,
    /// Final punctuation
    Pf,
    /// Other punctuation
    Po,
    /// Math symbol
    Sm,
    /// Modifier symbol
    Sk,
    /// Currency symbol
    Sc,
    /// Other symbol
    So,
    /// Space separator
    Zs,
    /// Line separator
    Zl,
    /// Paragraph separator
    Zp,
    /// Control character
    Cc,
    /// Format character
    Cf,
    /// Private use character
    Co,
    /// Unassigned character
    Cn,
}

/// enum for distinguishing Unicode major categories of characters
#[derive(PartialEq, Debug)]
pub enum MajorCategory {
    /// Letter
    L,
    /// Mark
    M,
    /// Number
    N,
    /// Punctuation
    P,
    /// Symbol
    S,
    /// Separator
    Z,
    /// Other character
    C,
}

// Internally defined structure to allow symbolic identification of category codes as numeric values
// in the generated tables.
struct Cat;

impl Cat {
    #![allow(non_upper_case_globals)]
    const Lu: u8 = 0x90;
    const Ll: u8 = 0x91;
    const Lt: u8 = 0x92;
    const LC: u8 = 0x90;
    const Lm: u8 = 0x83;
    const Lo: u8 = 0x84;
    const L: u8 = 0x80;
    const Mn: u8 = 0x10;
    const Mc: u8 = 0x11;
    const Me: u8 = 0x12;
    const M: u8 = 0x10;
    const Nd: u8 = 0x20;
    const Nl: u8 = 0x21;
    const No: u8 = 0x22;
    const N: u8 = 0x20;
    const Pc: u8 = 0x30;
    const Pd: u8 = 0x31;
    const Ps: u8 = 0x32;
    const Pe: u8 = 0x33;
    const Pi: u8 = 0x34;
    const Pf: u8 = 0x35;
    const Po: u8 = 0x36;
    const P: u8 = 0x30;
    const Sm: u8 = 0x40;
    const Sc: u8 = 0x41;
    const Sk: u8 = 0x42;
    const So: u8 = 0x43;
    const S: u8 = 0x40;
    const Zs: u8 = 0x50;
    const Zl: u8 = 0x51;
    const Zp: u8 = 0x52;
    const Z: u8 = 0x50;
    const Cc: u8 = 0x61;
    const Cf: u8 = 0x62;
    //    const Cs: u8 = 0x03;
    const Co: u8 = 0x64;
    const Cn: u8 = 0x60;
    const C: u8 = 0x60;
}

#[inline]
fn get_code(c: char) -> u8 {
    CAT_PAGES[usize::from(CAT_TABLE[(c as usize) >> 8])][(c as usize) & 0xff]
}

impl CharacterCategories for char {
    #[inline]
    fn get_major_category(self) -> MajorCategory {
        match get_code(self) & 0xf0 {
            Cat::L => MajorCategory::L,
            Cat::LC => MajorCategory::L,
            Cat::M => MajorCategory::M,
            Cat::N => MajorCategory::N,
            Cat::P => MajorCategory::P,
            Cat::S => MajorCategory::S,
            Cat::Z => MajorCategory::Z,
            Cat::C => MajorCategory::C,
            _ => {
                panic!("Corrupt character data")
            }
        }
    }

    #[inline]
    fn get_minor_category(self) -> MinorCategory {
        match get_code(self) {
            Cat::Lu => MinorCategory::Lu,
            Cat::Ll => MinorCategory::Ll,
            Cat::Lt => MinorCategory::Lt,
            Cat::Lm => MinorCategory::Lm,
            Cat::Lo => MinorCategory::Lo,
            Cat::Mn => MinorCategory::Mn,
            Cat::Mc => MinorCategory::Mc,
            Cat::Me => MinorCategory::Me,
            Cat::Nd => MinorCategory::Nd,
            Cat::Nl => MinorCategory::Nl,
            Cat::No => MinorCategory::No,
            Cat::Pc => MinorCategory::Pc,
            Cat::Pd => MinorCategory::Pd,
            Cat::Ps => MinorCategory::Ps,
            Cat::Pe => MinorCategory::Pe,
            Cat::Pi => MinorCategory::Pi,
            Cat::Pf => MinorCategory::Pf,
            Cat::Po => MinorCategory::Po,
            Cat::Sm => MinorCategory::Sm,
            Cat::Sc => MinorCategory::Sc,
            Cat::Sk => MinorCategory::Sk,
            Cat::So => MinorCategory::So,
            Cat::Zs => MinorCategory::Zs,
            Cat::Zl => MinorCategory::Zl,
            Cat::Zp => MinorCategory::Zp,
            Cat::Cc => MinorCategory::Cc,
            Cat::Cf => MinorCategory::Cf,
            Cat::Co => MinorCategory::Co,
            Cat::Cn => MinorCategory::Cn,
            _ => {
                panic!("Corrupt character data")
            }
        }
    }
    #[inline]
    fn is_letter(self) -> bool {
        get_code(self) & Cat::L == Cat::L
    }

    #[inline]
    fn is_letter_cased(self) -> bool {
        get_code(self) & 0xf0 == Cat::LC
    }

    #[inline]
    fn is_letter_uppercase(self) -> bool {
        get_code(self) == Cat::Lu
    }

    #[inline]
    fn is_letter_lowercase(self) -> bool {
        get_code(self) == Cat::Ll
    }

    #[inline]
    fn is_letter_titlecase(self) -> bool {
        get_code(self) == Cat::Lt
    }

    #[inline]
    fn is_letter_modifier(self) -> bool {
        get_code(self) == Cat::Lm
    }

    #[inline]
    fn is_letter_other(self) -> bool {
        get_code(self) == Cat::Lo
    }

    #[inline]
    fn is_mark(self) -> bool {
        get_code(self) & 0xf0 == Cat::M
    }

    #[inline]
    fn is_mark_nonspacing(self) -> bool {
        get_code(self) == Cat::Mn
    }

    #[inline]
    fn is_mark_spacing(self) -> bool {
        get_code(self) == Cat::Mc
    }

    #[inline]
    fn is_mark_enclosing(self) -> bool {
        get_code(self) == Cat::Me
    }

    #[inline]
    fn is_letter_or_mark(self) -> bool {
        get_code(self) & 0x60 == 0x00
    }

    #[inline]
    fn is_number(self) -> bool {
        get_code(self) & 0xf0 == Cat::N
    }

    #[inline]
    fn is_number_decimal(self) -> bool {
        get_code(self) == Cat::Nd
    }

    #[inline]
    fn is_number_letter(self) -> bool {
        get_code(self) == Cat::Nl
    }

    #[inline]
    fn is_number_other(self) -> bool {
        get_code(self) == Cat::No
    }

    #[inline]
    fn is_punctuation(self) -> bool {
        get_code(self) & 0xf0 == Cat::P
    }

    #[inline]
    fn is_punctuation_connector(self) -> bool {
        get_code(self) == Cat::Pc
    }

    #[inline]
    fn is_punctuation_dash(self) -> bool {
        get_code(self) == Cat::Pd
    }

    #[inline]
    fn is_punctuation_open(self) -> bool {
        get_code(self) == Cat::Ps
    }

    #[inline]
    fn is_punctuation_close(self) -> bool {
        get_code(self) == Cat::Pe
    }

    #[inline]
    fn is_punctuation_initial(self) -> bool {
        get_code(self) == Cat::Pi
    }

    #[inline]
    fn is_punctuation_final(self) -> bool {
        get_code(self) == Cat::Pf
    }

    #[inline]
    fn is_punctuation_other(self) -> bool {
        get_code(self) == Cat::Po
    }

    #[inline]
    fn is_symbol(self) -> bool {
        get_code(self) & 0xf0 == Cat::S
    }

    #[inline]
    fn is_symbol_math(self) -> bool {
        get_code(self) == Cat::Sm
    }

    #[inline]
    fn is_symbol_currency(self) -> bool {
        get_code(self) == Cat::Sc
    }

    #[inline]
    fn is_symbol_modifier(self) -> bool {
        get_code(self) == Cat::Sk
    }

    #[inline]
    fn is_symbol_other(self) -> bool {
        get_code(self) == Cat::So
    }

    #[inline]
    fn is_separator(self) -> bool {
        get_code(self) & 0xf0 == Cat::Z
    }

    #[inline]
    fn is_separator_space(self) -> bool {
        get_code(self) == Cat::Zs
    }

    #[inline]
    fn is_separator_line(self) -> bool {
        get_code(self) == Cat::Zl
    }

    #[inline]
    fn is_separator_paragraph(self) -> bool {
        get_code(self) == Cat::Zp
    }

    #[inline]
    fn is_other(self) -> bool {
        get_code(self) & 0xf0 == Cat::C
    }

    #[inline]
    fn is_control(self) -> bool {
        // Hard-coded ranges in std are about twice as fast as the table lookup.
        char::is_control(self)
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
    use crate::categories::*;

    #[test]
    fn character_categories() {
        assert!('a'.is_letter());
        assert!(!'a'.is_letter_uppercase());
        assert!('Ü'.is_letter_uppercase());
        assert!('Я'.is_letter_uppercase());

        // test data from http://www.i18nguy.com/unicode/supplementary-test.html
        "𠜎 𠜱 𠝹 𠱓 𠱸 𠲖 𠳏 𠳕 𠴕 𠵼 𠵿 𠸎 𠸏 𠹷 𠺝 𠺢 𠻗 𠻹 𠻺 𠼭 𠼮 𠽌 𠾴 𠾼 𠿪 𡁜 𡁯 𡁵 𡁶 𡁻 𡃁 𡃉 𡇙 𢃇 𢞵 𢫕 𢭃 𢯊 𢱑 𢱕 𢳂 𢴈 𢵌 𢵧 𢺳 𣲷 𤓓 𤶸 𤷪 𥄫 𦉘 𦟌 𦧲 𦧺 𧨾 𨅝 𨈇 𨋢 𨳊 𨳍 𨳒 𩶘".chars()
            .filter(|&c| c != ' ')
            .for_each(
                |c| {
                    assert!(c.is_letter());
                    assert!(c.is_letter_other());
                    assert!(!c.is_letter_cased());
                }
            );

        // test data from https://en.wikipedia.org/wiki/Module:Unicode_data/testcases
        assert!('\t'.is_control());
        assert!(' '.is_separator_space());
        assert!(
            '['.is_punctuation_open(),
            "Got character code of {}",
            get_code('[')
        );
        assert!(']'.is_punctuation_close());
        assert!('^'.is_symbol_modifier());
        assert!('A'.is_letter_uppercase());
        assert!('\u{00AD}'.is_format());
        assert!('¾'.is_number_other());
        assert!('«'.is_punctuation_initial());
        assert!('»'.is_punctuation_final());
        assert!('\u{0300}'.is_mark_nonspacing());
        assert!('\u{0488}'.is_mark_enclosing());
        assert!('٣'.is_number_decimal());
        assert!('子'.is_letter_other());
        assert!('ᾮ'.is_letter_titlecase());
        assert!('\u{1B44}'.is_mark_spacing());
        assert!('∈'.is_symbol_math());
        assert!('‿'.is_punctuation_connector());
        assert!('↹'.is_symbol_other());
        assert!('⸗'.is_punctuation_dash());
        assert!('Ⅷ'.is_number_letter());
        assert!('\u{2028}'.is_separator_line());
        assert!('\u{2029}'.is_separator_paragraph());
        assert!('ゞ'.is_letter_modifier());
        //assert!('\u{D800}'.is_surrogate());
        assert!('￡'.is_symbol_currency());
        assert!('\u{FFFF}'.is_unassigned());
        // Fully qualified: std has an unstable `char::is_private_use` that would otherwise shadow ours.
        assert!(CharacterCategories::is_private_use('\u{100000}'));

        for c in "edä\u{0301}\u{20dd}\u{903}".chars() {
            assert!(c.is_letter_or_mark());
        }
        for c in "45.$😀".chars() {
            assert!(!c.is_letter_or_mark());
        }

        assert_eq!(']'.get_minor_category(), MinorCategory::Pe);
        assert_eq!(']'.get_major_category(), MajorCategory::P);
    }
}
