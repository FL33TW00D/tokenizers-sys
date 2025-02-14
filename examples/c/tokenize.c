#include "../bindings/ctokenizers.h"
#include <stdio.h>
#include <stdlib.h>

void print_encoding_info(char* tokens[], const uint32_t* ids, 
                        const uint32_t* type_ids, const uint32_t* special_tokens_mask, 
                        const uint32_t* attention_mask, size_t length) {
    printf("Tokens:\n");
    for (size_t i = 0; i < length; i++) {
        printf("  %s (ID: %u, Type: %d, Special: %d, Attention: %d)\n",
               tokens[i], ids[i], type_ids[i], special_tokens_mask[i], attention_mask[i]);
    }
}

int test_bert_tokenizer(const char* text) {
    TokenizerHandle* tokenizer = tokenizer_from_pretrained("bert-base-cased", NULL);
    if (!tokenizer) {
        printf("Failed to load BERT tokenizer\n");
        return 1;
    }

    printf("\n=== Testing BERT Tokenizer ===\n");
    CEncoding* encoding = tokenizer_encode(tokenizer, text, true);
    if (!encoding) {
        printf("BERT encoding failed\n");
        tokenizer_free(tokenizer);
        return 1;
    }

    size_t length = encoding_get_length(encoding);
    const uint32_t* ids = encoding_get_ids(encoding, &length);
    char** tokens = encoding_get_tokens(encoding, &length);
    const uint32_t* type_ids = encoding_get_type_ids(encoding, &length);
    const uint32_t* special_tokens_mask = encoding_get_special_tokens_mask(encoding, &length);
    const uint32_t* attention_mask = encoding_get_attention_mask(encoding, &length);

    print_encoding_info(tokens, ids, type_ids, special_tokens_mask, attention_mask, length);

    char* decoded = tokenizer_decode(tokenizer, ids, length, true);
    if (decoded) {
        printf("\nBERT decoded text: %s\n", decoded);
        free_rstring(decoded);
    }

    encoding_free(encoding);
    tokenizer_free(tokenizer);
    return 0;
}

int test_gpt2_tokenizer(const char* text) {
    TokenizerHandle* tokenizer = tokenizer_from_pretrained("gpt2", NULL);
    if (!tokenizer) {
        printf("Failed to load GPT-2 tokenizer\n");
        return 1;
    }

    printf("\n=== Testing GPT-2 Tokenizer ===\n");
    CEncoding* encoding = tokenizer_encode(tokenizer, text, true);
    if (!encoding) {
        printf("GPT-2 encoding failed\n");
        tokenizer_free(tokenizer);
        return 1;
    }

    size_t length = encoding_get_length(encoding);
    const uint32_t* ids = encoding_get_ids(encoding, &length);
    char** tokens = encoding_get_tokens(encoding, &length);
    const uint32_t* type_ids = encoding_get_type_ids(encoding, &length);
    const uint32_t* special_tokens_mask = encoding_get_special_tokens_mask(encoding, &length);
    const uint32_t* attention_mask = encoding_get_attention_mask(encoding, &length);

    print_encoding_info(tokens, ids, type_ids, special_tokens_mask, attention_mask, length);

    char* decoded = tokenizer_decode(tokenizer, ids, length, true);
    if (decoded) {
        printf("\nGPT-2 decoded text: %s\n", decoded);
        free_rstring(decoded);
    }

    encoding_free(encoding);
    tokenizer_free(tokenizer);
    return 0;
}

int main() {
    const char* test_text = "The quick brown fox jumps over the lazy dog.";
    
    int bert_result = test_bert_tokenizer(test_text);
    if (bert_result != 0) {
        printf("BERT tokenizer test failed\n");
        return bert_result;
    }

    int gpt2_result = test_gpt2_tokenizer(test_text);
    if (gpt2_result != 0) {
        printf("GPT-2 tokenizer test failed\n");
        return gpt2_result;
    }

    return 0;
}
