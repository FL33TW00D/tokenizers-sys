# Increasing adoption of `tokenizers`

Tokenizers is the defacto reference implementation for tokenization in the ML ecosystem. Unfortunately, usage is
currently siloed in Rust and Python, with no easy way to use it in other languages. This proposal aims to increase the adoption of `tokenizers` by providing a simple, safe way to use it in other languages.

## Motivation

With `tokenizers` only being available in Rust and Python, all other languages are fragmented between implementing their
own FFI from the Rust library, or hand rolling tokenization logic in their own language. This leads to a mountain of
issues, such as:
- [Deep Java Library](https://github.com/deepjavalibrary/djl/issues/3604)
- [Swift-Transformers](https://github.com/huggingface/swift-transformers/issues/116)
- [Go request on `tokenizers`](https://github.com/huggingface/tokenizers/issues/1751)

Some of our partners have signalled their desire for more language bindings for tokenizers too. In the on-device team,
the ability to tokenize in Swift and Kotlin would be a huge win for us.

## Risks

Each new language added will require maintenance and support. Luckily for us, if we maintain a solid FFI layer in Rust,
we decrease the friction for each language community to maintain the C -> Respective Language bindings.
