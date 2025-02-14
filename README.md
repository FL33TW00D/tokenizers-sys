# tokenizers-sys

Surfacing `tokenizers` to the world via C.

## ⚠️ EXPERIMENTAL WARNING

**This is an experimental implementation.** 
**It is extremely unsafe and most likely buggy!**

## Reasoning

C is the lingua franca of programming. Almost every language has some kind of C interop.
`Tokenizers` is a foundational library, it is a shame to see it only usable via `Rust` and `Python`.

Going from `Rust -> C` is starting off a fully safe foundation.
If we _carefully_ expose this via a C interface, we can be reasonably certain that it is safe (in contrast to going `C -> Rust`)

## Getting Started

Run `compile_ex.sh` on ARM Macs to tokenize some text in C!
Try `zig build run` inside `/examples/zig` to tokenize some text from Zig!

The possibilities are endless! Which self respecting languages _don't_ have a C interop?


## TODO
- [ ] Cross-compiling build system (Makefile or Zig)
- [x] Tokenizer
- [ ] Decoders
- [ ] Encoding
- [ ] Pre-tokenizers
- [ ] Trainers
- [ ] Models
- [ ] Normalizers
