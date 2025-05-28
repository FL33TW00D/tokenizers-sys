package main

/*
#cgo CFLAGS: -I../../bindings
#cgo LDFLAGS: -L../../target/release -ltokenizers_sys
#include "ctokenizers.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"unsafe"
)

type Tokenizer struct {
	handle *C.TokenizerHandle
}

type Encoding struct {
	handle *C.CEncoding
}

func NewTokenizerFromPretrained(name string) (*Tokenizer, error) {
	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))
	
	handle := C.tokenizer_from_pretrained(cName, nil)
	if handle == nil {
		return nil, fmt.Errorf("failed to load tokenizer: %s", name)
	}
	
	return &Tokenizer{handle: handle}, nil
}

func NewTokenizerFromFile(path string) (*Tokenizer, error) {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	
	handle := C.tokenizer_from_file(cPath)
	if handle == nil {
		return nil, fmt.Errorf("failed to load tokenizer from file: %s", path)
	}
	
	return &Tokenizer{handle: handle}, nil
}

func (t *Tokenizer) Encode(text string, addSpecialTokens bool) (*Encoding, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))
	
	encoding := C.tokenizer_encode(t.handle, cText, C.bool(addSpecialTokens))
	if encoding == nil {
		return nil, fmt.Errorf("failed to encode text")
	}
	
	return &Encoding{handle: encoding}, nil
}

func (t *Tokenizer) Decode(ids []uint32, skipSpecialTokens bool) (string, error) {
	if len(ids) == 0 {
		return "", nil
	}
	
	cIds := (*C.uint)(unsafe.Pointer(&ids[0]))
	decoded := C.tokenizer_decode(t.handle, cIds, C.uintptr_t(len(ids)), C.bool(skipSpecialTokens))
	if decoded == nil {
		return "", fmt.Errorf("failed to decode ids")
	}
	defer C.free_rstring(decoded)
	
	return C.GoString(decoded), nil
}

func (t *Tokenizer) Free() {
	if t.handle != nil {
		C.tokenizer_free(t.handle)
		t.handle = nil
	}
}

func (e *Encoding) GetLength() int {
	return int(C.encoding_get_length(e.handle))
}

func (e *Encoding) GetIds() []uint32 {
	var length C.uintptr_t
	cIds := C.encoding_get_ids(e.handle, &length)
	if cIds == nil {
		return nil
	}
	
	return (*[1 << 30]uint32)(unsafe.Pointer(cIds))[:length:length]
}

func (e *Encoding) GetTokens() []string {
	var length C.uintptr_t
	cTokens := C.encoding_get_tokens(e.handle, &length)
	if cTokens == nil {
		return nil
	}
	defer C.free_c_char_array(cTokens, length)
	
	tokens := make([]string, length)
	cTokensSlice := (*[1 << 30]*C.char)(unsafe.Pointer(cTokens))[:length:length]
	for i, cToken := range cTokensSlice {
		tokens[i] = C.GoString(cToken)
	}
	
	return tokens
}

func (e *Encoding) GetTypeIds() []uint32 {
	var length C.uintptr_t
	cTypeIds := C.encoding_get_type_ids(e.handle, &length)
	if cTypeIds == nil {
		return nil
	}
	
	return (*[1 << 30]uint32)(unsafe.Pointer(cTypeIds))[:length:length]
}

func (e *Encoding) GetSpecialTokensMask() []uint32 {
	var length C.uintptr_t
	cMask := C.encoding_get_special_tokens_mask(e.handle, &length)
	if cMask == nil {
		return nil
	}
	
	return (*[1 << 30]uint32)(unsafe.Pointer(cMask))[:length:length]
}

func (e *Encoding) GetAttentionMask() []uint32 {
	var length C.uintptr_t
	cMask := C.encoding_get_attention_mask(e.handle, &length)
	if cMask == nil {
		return nil
	}
	
	return (*[1 << 30]uint32)(unsafe.Pointer(cMask))[:length:length]
}

func (e *Encoding) Free() {
	if e.handle != nil {
		C.encoding_free(e.handle)
		e.handle = nil
	}
}

func printEncodingInfo(tokens []string, ids, typeIds, specialTokensMask, attentionMask []uint32) {
	fmt.Println("Tokens:")
	for i := 0; i < len(tokens) && i < len(ids); i++ {
		var typeId, special, attention uint32
		if i < len(typeIds) {
			typeId = typeIds[i]
		}
		if i < len(specialTokensMask) {
			special = specialTokensMask[i]
		}
		if i < len(attentionMask) {
			attention = attentionMask[i]
		}
		
		fmt.Printf("  %s (ID: %d, Type: %d, Special: %d, Attention: %d)\n",
			tokens[i], ids[i], typeId, special, attention)
	}
}

func testBertTokenizer(text string) error {
	tokenizer, err := NewTokenizerFromPretrained("bert-base-cased")
	if err != nil {
		return fmt.Errorf("failed to load BERT tokenizer: %v", err)
	}
	defer tokenizer.Free()
	
	fmt.Println("\n=== Testing BERT Tokenizer ===")
	encoding, err := tokenizer.Encode(text, true)
	if err != nil {
		return fmt.Errorf("BERT encoding failed: %v", err)
	}
	defer encoding.Free()
	
	ids := encoding.GetIds()
	tokens := encoding.GetTokens()
	typeIds := encoding.GetTypeIds()
	specialTokensMask := encoding.GetSpecialTokensMask()
	attentionMask := encoding.GetAttentionMask()
	
	printEncodingInfo(tokens, ids, typeIds, specialTokensMask, attentionMask)
	
	decoded, err := tokenizer.Decode(ids, true)
	if err != nil {
		return fmt.Errorf("BERT decoding failed: %v", err)
	}
	fmt.Printf("\nBERT decoded text: %s\n", decoded)
	
	return nil
}

func testGpt2Tokenizer(text string) error {
	tokenizer, err := NewTokenizerFromPretrained("gpt2")
	if err != nil {
		return fmt.Errorf("failed to load GPT-2 tokenizer: %v", err)
	}
	defer tokenizer.Free()
	
	fmt.Println("\n=== Testing GPT-2 Tokenizer ===")
	encoding, err := tokenizer.Encode(text, true)
	if err != nil {
		return fmt.Errorf("GPT-2 encoding failed: %v", err)
	}
	defer encoding.Free()
	
	ids := encoding.GetIds()
	tokens := encoding.GetTokens()
	typeIds := encoding.GetTypeIds()
	specialTokensMask := encoding.GetSpecialTokensMask()
	attentionMask := encoding.GetAttentionMask()
	
	printEncodingInfo(tokens, ids, typeIds, specialTokensMask, attentionMask)
	
	decoded, err := tokenizer.Decode(ids, true)
	if err != nil {
		return fmt.Errorf("GPT-2 decoding failed: %v", err)
	}
	fmt.Printf("\nGPT-2 decoded text: %s\n", decoded)
	
	return nil
}

func main() {
	testText := "The quick brown fox jumps over the lazy dog."
	
	if err := testBertTokenizer(testText); err != nil {
		fmt.Printf("BERT tokenizer test failed: %v\n", err)
		return
	}
	
	if err := testGpt2Tokenizer(testText); err != nil {
		fmt.Printf("GPT-2 tokenizer test failed: %v\n", err)
		return
	}
}
