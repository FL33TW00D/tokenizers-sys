

mkdir -p $(dirname "llama-3-tokenizer.json") && \
curl -L https://huggingface.co/hf-internal-testing/llama3-tokenizer/resolve/main/tokenizer.json --output llama-3-tokenizer.json
