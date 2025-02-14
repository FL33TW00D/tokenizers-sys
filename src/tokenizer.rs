use std::collections::HashMap;
use std::ffi::{c_char, c_uint, CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

use tk::tokenizer::Tokenizer;
use tokenizers::{self as tk, FromPretrainedParameters};

use crate::encoding::CEncoding;

//Opaque struct to hide Tokenizer internals
pub struct TokenizerHandle(*mut Tokenizer);

/// Frees the tokenizer handle
///
/// # Safety
/// Handle must be a valid TokenizerHandle
#[no_mangle]
pub unsafe extern "C" fn tokenizer_free(handle: *mut TokenizerHandle) {
    if !handle.is_null() {
        let tokenizer_handle = unsafe { Box::from_raw(handle) };
        let _ = unsafe { Box::from_raw(tokenizer_handle.0) };
    }
}

#[repr(C)]
pub struct CFromPretrainedParameters {
    pub revision: *const c_char,
    pub token: *const c_char,
}

impl From<&CFromPretrainedParameters> for Option<FromPretrainedParameters> {
    fn from(params: &CFromPretrainedParameters) -> Self {
        unsafe {
            let CFromPretrainedParameters { revision, token } = params;
            let revision = match revision.is_null() {
                true => "main",
                false => CStr::from_ptr(*revision).to_str().unwrap_or(""),
            };

            let token = match token.is_null() {
                true => None,
                false => Some(CStr::from_ptr(*token).to_str().unwrap_or("").to_owned()),
            };

            Some(FromPretrainedParameters {
                revision: revision.to_owned(),
                user_agent: HashMap::new(), //User agent is not supported via FFI
                token,
            })
        }
    }
}

/// Creates a new tokenizer from a pretrained model identifier with parameters
///
/// # Safety
/// - `name` must be a valid C string
/// - `params` must be either null or a valid pointer to FromPretrainedParametersFFI
/// Returns a pointer to the TokenizerHandle
/// The caller is responsible for freeing the memory using tokenizer_free()
#[no_mangle]
pub unsafe extern "C" fn tokenizer_from_pretrained(
    name: *const c_char,
    params: *const CFromPretrainedParameters,
) -> *mut TokenizerHandle {
    if name.is_null() {
        return ptr::null_mut();
    }

    let c_name = match CStr::from_ptr(name).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let params = params.as_ref().and_then(|p| p.into());

    match Tokenizer::from_pretrained(c_name, params) {
        Ok(tokenizer) => {
            let handle = Box::new(TokenizerHandle(Box::into_raw(Box::new(tokenizer))));
            Box::into_raw(handle)
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Instantiate a new :class:`~tokenizers.Tokenizer` from the given buffer.
///
/// Args:
///     buffer (:obj:`bytes`):
///         A buffer containing a previously serialized :class:`~tokenizers.Tokenizer`
///
/// Returns:
///     :class:`~tokenizers.Tokenizer`: The new tokenizer
#[no_mangle]
pub unsafe extern "C" fn tokenizer_from_buffer(
    buffer: *const u8,
    len: usize,
) -> *mut TokenizerHandle {
    assert!(len > 0, "Tokenizer src buffer len must be greater than 0");
    if buffer.is_null() {
        return ptr::null_mut();
    }

    match Tokenizer::from_bytes(std::slice::from_raw_parts(buffer, len)) {
        Ok(tokenizer) => {
            let handle = Box::new(TokenizerHandle(Box::into_raw(Box::new(tokenizer))));
            Box::into_raw(handle)
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Creates a new `Tokenizer` from a file containing a serialized tokenizer
///
/// # Safety
/// - path must be a valid null-terminated C string
/// - The file at path must contain a valid JSON serialized tokenizer
/// - The caller is responsible for freeing the memory using tokenizer_free()
///
/// # Arguments
/// * path - Path to the file containing the serialized tokenizer
///
/// # Returns
/// * Pointer to the TokenizerHandle on success
/// * NULL pointer on failure
#[no_mangle]
pub unsafe extern "C" fn tokenizer_from_file(path: *const c_char) -> *mut TokenizerHandle {
    if path.is_null() {
        return ptr::null_mut();
    }

    let str_bytes = CStr::from_ptr(path).to_bytes();
    let osstr = OsStr::from_bytes(str_bytes);
    let path: &Path = osstr.as_ref();

    match Tokenizer::from_file(path) {
        Ok(tokenizer) => {
            let handle = Box::new(TokenizerHandle(Box::into_raw(Box::new(tokenizer))));
            Box::into_raw(handle)
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Encodes text using the given tokenizer
///
/// # Safety
/// Handle must be a valid TokenizerHandle
/// Text must be a valid C string
#[no_mangle]
pub unsafe extern "C" fn tokenizer_encode(
    handle: *mut TokenizerHandle,
    text: *const c_char,
    add_special_tokens: bool,
) -> *mut CEncoding {
    let tokenizer = unsafe {
        if handle.is_null() {
            return ptr::null_mut();
        }
        &mut *(*handle).0
    };

    let text = match unsafe { CStr::from_ptr(text).to_str() } {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match tokenizer.encode(text, add_special_tokens) {
        Ok(encoding) => Box::into_raw(Box::new(CEncoding::new(encoding))),
        Err(_) => ptr::null_mut(),
    }
}

/// Decodes a sequence of ids into a string
///
/// # Safety
/// Handle must be a valid TokenizerHandle
/// Ids must be a valid array of c_uint
/// Length must be the length of the ids array
///
/// # Returns
/// A pointer to the decoded string.
#[no_mangle]
pub unsafe extern "C" fn tokenizer_decode(
    handle: *mut TokenizerHandle,
    ids: *const c_uint,
    length: usize,
    skip_special_tokens: bool,
) -> *mut c_char {
    assert!(length > 0, "Length must be greater than 0");
    let tokenizer = unsafe {
        if handle.is_null() {
            return ptr::null_mut();
        }
        &mut *(*handle).0
    };

    let ids_slice = unsafe { std::slice::from_raw_parts(ids, length) };
    match tokenizer.decode(ids_slice, skip_special_tokens) {
        Ok(text) => match CString::new(text) {
            Ok(s) => s.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        Err(_) => ptr::null_mut(),
    }
}

/// He who allocates must deallocate
/// If rust, allocates the string, **DO NOT CALL `free()`**, allow rust
/// to deallocate it with `free_rstring`
#[no_mangle]
pub unsafe extern "C" fn free_rstring(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::*;
    use std::ffi::CString;
    use std::fs::File;
    use std::io::Read;
    use std::ptr;

    fn create_test_tokenizer() -> *mut TokenizerHandle {
        let model = CString::new("bert-base-uncased").unwrap();
        unsafe { tokenizer_from_pretrained(model.as_ptr(), ptr::null()) }
    }

    #[test]
    fn test_ffi_basics() -> anyhow::Result<()> {
        unsafe {
            let handle = tokenizer_from_pretrained(ptr::null(), ptr::null());
            assert!(handle.is_null());

            let handle = create_test_tokenizer();
            let text = CString::new("Hello world!").map_err(|n| anyhow::anyhow!(n))?;
            let encoding = tokenizer_encode(handle, text.as_ptr(), true);
            encoding_free(encoding);
            tokenizer_free(handle);

            Ok(())
        }
    }

    #[test]
    fn test_tokenizer_creation_and_free() {
        unsafe {
            let handle = create_test_tokenizer();
            assert!(!handle.is_null());
            tokenizer_free(handle);
        }
    }

    #[test]
    fn test_tokenizer_null_inputs() {
        unsafe {
            let handle = tokenizer_from_pretrained(ptr::null(), ptr::null());
            assert!(handle.is_null());

            let handle = tokenizer_from_file(ptr::null());
            assert!(handle.is_null());
        }
    }

    #[test]
    fn test_tokenizer_encode_decode() {
        unsafe {
            let handle = create_test_tokenizer();
            assert!(!handle.is_null());

            let text = CString::new("Hello world").unwrap();
            let encoding = tokenizer_encode(handle, text.as_ptr(), true);
            assert!(!encoding.is_null());

            let mut ids_length: usize = 0;
            let ids = encoding_get_ids(encoding, &mut ids_length);
            assert!(!ids.is_null());

            // Test decoding
            let decoded = tokenizer_decode(handle, ids as *const c_uint, ids_length, true);
            assert!(!decoded.is_null());

            // Cleanup
            encoding_free(encoding);
            free_rstring(decoded);
            tokenizer_free(handle);
        }
    }

    #[test]
    fn test_invalid_tokenizer_from_buffer() {
        unsafe {
            let buffer: Vec<u8> = vec![1, 2, 3, 4];
            let handle = tokenizer_from_buffer(buffer.as_ptr(), buffer.len());
            assert!(handle.is_null());
        }
    }

    #[test]
    fn test_valid_tokenizer_from_buffer() {
        let mut file =
            File::open("resources/llama-3-tokenizer.json").expect("Failed to open tokenizer file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        unsafe {
            let handle = tokenizer_from_buffer(buffer.as_ptr(), buffer.len());
            assert!(
                !handle.is_null(),
                "Tokenizer handle should not be null for valid JSON"
            );
            //tokenize something to prove it's correct
            let text = CString::new("Hello world").unwrap();
            let encoding = tokenizer_encode(handle, text.as_ptr(), true);
            assert!(!encoding.is_null());
            encoding_free(encoding);

            if !handle.is_null() {
                tokenizer_free(handle);
            }
        }
    }

    #[test]
    fn test_tokenizer_from_file() {
        unsafe {
            let invalid_path = CString::new("/path/to/nonexistent/file").unwrap();
            let handle = tokenizer_from_file(invalid_path.as_ptr());
            assert!(handle.is_null());
        }
    }
}
