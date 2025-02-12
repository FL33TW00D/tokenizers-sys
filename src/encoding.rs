use std::ffi::CString;
use std::os::raw::c_char;
use tokenizers as tk;

pub struct CEncoding {
    encoding: tk::tokenizer::Encoding,
}

impl CEncoding {
    pub fn new(encoding: tk::tokenizer::Encoding) -> Self {
        Self { encoding }
    }
}

impl From<tk::tokenizer::Encoding> for CEncoding {
    fn from(encoding: tk::tokenizer::Encoding) -> Self {
        Self::new(encoding)
    }
}

impl From<CEncoding> for tk::tokenizer::Encoding {
    fn from(encoding: CEncoding) -> Self {
        encoding.encoding
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_free(encoding: *mut CEncoding) {
    if !encoding.is_null() {
        let _ = Box::from_raw(encoding);
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_length(encoding: *const CEncoding) -> usize {
    if encoding.is_null() {
        return 0;
    }
    (*encoding).encoding.get_ids().len()
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_ids(
    encoding: *const CEncoding,
    length: *mut usize,
) -> *const u32 {
    if encoding.is_null() || length.is_null() {
        return std::ptr::null();
    }
    let enc = &(*encoding).encoding;
    let ids = enc.get_ids();
    *length = ids.len();
    ids.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_tokens(
    encoding: *const CEncoding,
    length: *mut usize,
) -> *mut *mut c_char {
    if encoding.is_null() || length.is_null() {
        return std::ptr::null_mut();
    }
    let enc = &(*encoding).encoding;
    let tokens = enc.get_tokens();
    *length = tokens.len();

    let c_tokens: Vec<*mut c_char> = tokens
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap().into_raw())
        .collect();

    Box::into_raw(c_tokens.into_boxed_slice()) as *mut *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_type_ids(
    encoding: *const CEncoding,
    length: *mut usize,
) -> *const u32 {
    if encoding.is_null() || length.is_null() {
        return std::ptr::null();
    }
    let enc = &(*encoding).encoding;
    let type_ids = enc.get_type_ids();
    *length = type_ids.len();
    type_ids.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_special_tokens_mask(
    encoding: *const CEncoding,
    length: *mut usize,
) -> *const u32 {
    if encoding.is_null() || length.is_null() {
        return std::ptr::null();
    }
    let enc = &(*encoding).encoding;
    let special_tokens_mask = enc.get_special_tokens_mask();
    *length = special_tokens_mask.len();
    special_tokens_mask.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_attention_mask(
    encoding: *const CEncoding,
    length: *mut usize,
) -> *const u32 {
    if encoding.is_null() || length.is_null() {
        return std::ptr::null();
    }
    let enc = &(*encoding).encoding;
    let attention_mask = enc.get_attention_mask();
    *length = attention_mask.len();
    attention_mask.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_offsets(
    encoding: *const CEncoding,
    length: *mut usize,
) -> *const (usize, usize) {
    if encoding.is_null() || length.is_null() {
        return std::ptr::null();
    }
    let enc = &(*encoding).encoding;
    let offsets = enc.get_offsets();
    *length = offsets.len();
    offsets.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_overflowing(
    encoding: *const CEncoding,
    length: *mut usize,
) -> *mut CEncoding {
    if encoding.is_null() || length.is_null() {
        return std::ptr::null_mut();
    }
    let enc = &(*encoding).encoding;
    let overflowing = enc.get_overflowing().clone(); //Python wrapper clones
    *length = overflowing.len();

    let c_encodings: Vec<CEncoding> = overflowing.into_iter().map(|e| e.into()).collect();
    Box::into_raw(c_encodings.into_boxed_slice()) as *mut CEncoding
}

#[no_mangle]
pub unsafe extern "C" fn free_c_char_array(array: *mut *mut c_char, length: usize) {
    if array.is_null() {
        return;
    }
    let tokens = std::slice::from_raw_parts(array, length);
    for &ptr in tokens {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
    let _ = Box::from_raw(std::slice::from_raw_parts_mut(array, length));
}

#[no_mangle]
pub unsafe extern "C" fn free_encoding_array(array: *mut CEncoding, length: usize) {
    if array.is_null() {
        return;
    }
    let _ = Box::from_raw(std::slice::from_raw_parts_mut(array, length));
}
