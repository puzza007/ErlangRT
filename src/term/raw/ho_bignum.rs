//! Heap object which stores a bignum.
//! Do not import this file directly, use `use term::raw::*;` instead.

use std::mem::size_of;
use std::ptr;
use num::bigint::BigInt;

use emulator::heap::{IHeap, Heap};
use fail::Hopefully;
use rt_defs::{WORD_BYTES, Word};
use term::classify::TermClass;
use term::lterm::*;
use term::raw::heapobj::*;


/// Heap object `HOBignum` is placed on heap by the VM and contains a signed
/// big integer. Bignum digits are stored after the bignum record.
#[allow(dead_code)]
pub struct HOBignum {
  hobj: HeapObjHeader,

  /// The actual value. NOTE: Here we trust `Vec<BigDigit>` to manage the
  /// memory for its digits on the general application heap.
  // TODO: Not nice! Manage own data for HOBignum.
  pub value: BigInt,
}


#[allow(const_err)]
static HOCLASS_BIGNUM: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Bignum,
  dtor: HOBignum::dtor,
  fmt_str: HOBignum::fmt_str,
  term_class: TermClass::Number,
};



impl HOBignum {

  /// Destructor.
  pub unsafe fn dtor(this0: *mut Word) {
    let this = this0 as *mut HOBignum;
    ptr::drop_in_place(this);
  }


  /// Format as a string.
  pub unsafe fn fmt_str(this0: *const Word) -> String {
    let this = this0 as *mut HOBignum;
    format!("Big({})", &(*this).value)
  }


  /// Given a term, unbox it and convert to a `HOBignum` const pointer.
  #[inline]
  #[allow(dead_code)]
  pub unsafe fn from_term(t: LTerm) -> Hopefully<*const HOBignum> {
    heapobj_from_term::<HOBignum>(t, &HOCLASS_BIGNUM)
  }


//  /// Given a term, unbox it and convert to a `HOBignum` mut pointer.
  //  #[inline]
  //  pub fn from_term_mut(t: LTerm) -> *const HOBignum {
  //    let p = t.box_ptr_mut();
  //    p as *mut HOBignum
  //  }

  #[inline]
  fn storage_size() -> usize {
    (size_of::<HOBignum>() + WORD_BYTES - 1) / WORD_BYTES
  }


  fn new(n_words: usize, value: BigInt) -> HOBignum {
    HOBignum {
      hobj: HeapObjHeader::new(n_words, &HOCLASS_BIGNUM),
      value,
    }
  }


  /// Allocate space on heap for `n_bytes` and initialize the fields.
  /// A pointer to binary is returned which manages heap placement automatically
  /// (i.e. heapbin or procbin etc, are used automatically).
  pub unsafe fn place_into(hp: &mut Heap,
                           value: BigInt) -> Hopefully<*mut HOBignum>
  {
    let n_words = HOBignum::storage_size();
    let this = hp.heap_allocate(n_words, false)? as *mut HOBignum;

    ptr::write(this, HOBignum::new(n_words, value));

    Ok(this)
  }


  pub fn make_term(this: *const HOBignum) -> LTerm {
    make_box(this as *const Word)
  }

}
