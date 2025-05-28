# tokenizers-sys

          ┌───────────────────────────┐
          │                           │
          │     Original Rust         │
          │     Implementation        │
          │                           │
          └─────────────┬─────────────┘
                        │
                        │
                        ▼
          ┌───────────────────────────┐
          │                           │
          │       FFI Layer           │
          │     (C Interface)         │
          │                           │
          └─────────────┬─────────────┘
                        │
                        │
    ┌─────────┼─────────┼─────────┬────────────┐
    │         │         │         │            │
    ▼         ▼         ▼         ▼            ▼
┌─────────┐ ┌─────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐
│   Zig   │ │ Go  │ │  Java   │ │  Swift  │ │ Kotlin  │
└─────────┘ └─────┘ └─────────┘ └─────────┘ └─────────┘


## ⚠️ EXPERIMENTAL WARNING

> ** This is NOT an official HuggingFace library.**
> **This is an experimental implementation.** 
> **It is extremely unsafe and most likely buggy!**

## Thesis 

C is the lingua franca of programming, with almost every modern language offering some kind of C interop.
`Tokenizers` is a foundational library, and more languages should be able to use it.

Going from `Rust -> C` is starting off a safe foundation, judicious implementation of a C FFI allows other languages to
use the library without needing to reimplement it in their own language.

