build:
    cargo build --release

fetch-resources:
    mkdir -p $(dirname "llama-3-tokenizer.json") && \   
    curl -L https://huggingface.co/hf-internal-testing/llama3-tokenizer/resolve/main/tokenizer.json --output llama-3-tokenizer.json

# C example
c-example: build
    clang -arch arm64 \
        examples/c/tokenize.c \
        -Wall \
        -Wextra \
        -L target/release \
        -ltokenizers_sys \
        -lc++ \
        -framework Security \
        -framework Foundation \
        -rpath @executable_path/../target/release \
        -o examples/c/tokenize

run-c: c-example 
    cd examples/c && ./tokenize

go-example: build
    cd examples/go && CGO_LDFLAGS="-L../../target/release -ltokenizers_sys" \
        CGO_CFLAGS="-I../../bindings" \
        go build -o tokenizers-go-example main.go

run-go: go-example
    cd examples/go && LD_LIBRARY_PATH=../../target/release ./tokenizers-go-example

java-example: build
    cd examples/java && mvn compile exec:exec@compile-jni

run-java: java-example 
    cd examples/java && mvn exec:exec

zig-example: build
    cd examples/zig && cp ../../target/release/libtokenizers_sys.dylib lib/ && zig build

run-zig: zig-example
    cd examples/zig && ./zig-out/bin/tokenizer-example

kotlin-example: build
    cd examples/kotlin && gradle compileJni build

run-kotlin: kotlin-example
    cd examples/kotlin && gradle run


