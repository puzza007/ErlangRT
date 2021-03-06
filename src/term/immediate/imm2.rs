//!
//! Immediate2 values used to represent smaller special term types.
//! Bit composition is - `.... .... ..bb aaPP`, where `PP` is primary tag,
//! `aa` is imm1 tag, and `bb` is imm2 tag
//!
//! Max value for imm2 is 64-6=58, or 32-6=26 bits.
//!
use rt_defs;
use rt_defs::Word;
use term::immediate::imm1::*;

use bit_field::BitField;

/// Bit position for imm1 tag
pub const IMM2_TAG_FIRST: u8 = 4;
pub const IMM2_TAG_LAST: u8 = 6;

/// Bit position for the value after imm1 tag
pub const IMM2_VALUE_FIRST: u8 = IMM2_TAG_LAST;
pub const IMM2_VALUE_LAST: u8 = rt_defs::WORD_BITS as u8;


pub const TAG_IMM2_ATOM: Word = 0;
pub const TAG_IMM2_CATCH: Word = 1;
pub const TAG_IMM2_IMM3: Word = 2;
/// Special includes unique values like NIL, NONVALUE
pub const TAG_IMM2_SPECIAL: Word = 3;

/// Max value for the Immediate2 enum (for assertions).
pub const IMMEDIATE2_MAX: Word = 3;


/// Values are pre-shifted to allow the 4 bits of header to line up with the 4
/// bits shift, and so that they look exactly same when printed as hex.
pub const TAG_IMM2_SPECIAL_NIL: Word = 0x0101_0101 << 4;
pub const TAG_IMM2_SPECIAL_NONVALUE: Word = 0x0303_0303 << 4;
pub const TAG_IMM2_SPECIAL_EMPTY_TUPLE: Word = 0x0505_0505 << 4;
pub const TAG_IMM2_SPECIAL_EMPTY_BIN: Word = 0x0707_0707 << 4;


/// Trim to have only immediate2 bits and return them as an convenient enum.
#[inline]
pub fn get_imm2_tag(val: Word) -> Word {
  let t: Word = val.get_bits(IMM2_TAG_FIRST..IMM2_TAG_LAST);
  assert!(t <= IMMEDIATE2_MAX);
  t
}


/// Remove tag bits from imm2 value by shifting it right
#[inline]
pub fn get_imm2_value(val: Word) -> Word {
  assert!(is_immediate2(val));
  val.get_bits(IMM2_VALUE_FIRST..IMM2_VALUE_LAST)
}


/// Precomposed bits for immediate2 values
pub const IMM2_PREFIX: Word = IMM1_PREFIX
    | (TAG_IMM1_IMM2 << IMM1_TAG_FIRST);


/// Precomposed bits for atom imm2
pub const IMM2_ATOM_PREFIX: Word = IMM2_PREFIX
    | (TAG_IMM2_ATOM << IMM2_TAG_FIRST);


//--- Imm2 values tagged special ---

/// Special Primary tag+Immed1+Immed2 bits precomposed
pub const IMM2_SPECIAL_PREFIX: Word = IMM2_PREFIX
    | (TAG_IMM2_SPECIAL << IMM2_TAG_FIRST);

/// Precomposed bits for NIL constant
pub const IMM2_SPECIAL_NIL_RAW: Word = IMM2_SPECIAL_PREFIX
    | (TAG_IMM2_SPECIAL_NIL << IMM2_VALUE_FIRST);

/// Precomposed bits for `NON_VALUE` constant
pub const IMM2_SPECIAL_NONVALUE_RAW: Word = IMM2_SPECIAL_PREFIX
    | (TAG_IMM2_SPECIAL_NONVALUE << IMM2_VALUE_FIRST);

/// Precomposed bits for `Empty tuple` constant
pub const IMM2_SPECIAL_EMPTY_TUPLE_RAW: Word = IMM2_SPECIAL_PREFIX
    | (TAG_IMM2_SPECIAL_EMPTY_TUPLE << IMM2_VALUE_FIRST);

/// Precomposed bits for `Empty binary` constant
pub const IMM2_SPECIAL_EMPTY_BIN_RAW: Word = IMM2_SPECIAL_PREFIX
    | (TAG_IMM2_SPECIAL_EMPTY_BIN << IMM2_VALUE_FIRST);


/// Get prefix bits BEFORE imm2 tag
#[inline]
pub fn get_imm2_prefix(val: Word) -> Word {
  val.get_bits(0..IMM2_TAG_FIRST)
}


/// Get prefix bits BEFORE imm2 tag plus the tag itself (for raw comparisons)
#[inline]
pub fn get_imm2_prefix_and_tag(val: Word) -> Word {
  val.get_bits(0..IMM2_VALUE_FIRST)
}



#[inline]
pub fn is_immediate2(val: Word) -> bool {
  get_imm2_prefix(val) == IMM2_PREFIX
}


/// Given a value raw preset bits, compose them together and form an imm2 `LTerm`
#[inline]
pub fn combine_imm2_prefix_and_val(val: Word, prefix0: Word) -> Word {
  let mut prefix = prefix0;
  assert!(prefix < (1 << IMM2_VALUE_FIRST));
  assert!(val < (1 << (IMM2_VALUE_LAST - IMM2_VALUE_FIRST)));
  *prefix.set_bits(IMM2_VALUE_FIRST..IMM2_VALUE_LAST, val)
}


#[inline]
pub const fn combine_imm2_prefix_and_val_const(val: Word, prefix0: Word) -> Word {
  prefix0 | (val << IMM2_VALUE_FIRST)
}
