#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CEncoding CEncoding;

typedef struct TokenizerHandle TokenizerHandle;

typedef struct CFromPretrainedParameters {
  const char *revision;
  const char *token;
} CFromPretrainedParameters;

void encoding_free(struct CEncoding *encoding);

uintptr_t encoding_get_length(const struct CEncoding *encoding);

const uint32_t *encoding_get_ids(const struct CEncoding *encoding, uintptr_t *length);

char **encoding_get_tokens(const struct CEncoding *encoding, uintptr_t *length);

const uint32_t *encoding_get_type_ids(const struct CEncoding *encoding, uintptr_t *length);

const uint32_t *encoding_get_special_tokens_mask(const struct CEncoding *encoding,
                                                 uintptr_t *length);

const uint32_t *encoding_get_attention_mask(const struct CEncoding *encoding, uintptr_t *length);

struct CEncoding *encoding_get_overflowing(const struct CEncoding *encoding, uintptr_t *length);

void free_c_char_array(char **array, uintptr_t length);

void free_encoding_array(struct CEncoding *array, uintptr_t length);

/**
 * Frees the tokenizer handle
 *
 * # Safety
 * Handle must be a valid TokenizerHandle
 */
void tokenizer_free(struct TokenizerHandle *handle);

/**
 * Creates a new tokenizer from a pretrained model identifier with parameters
 *
 * # Safety
 * - `name` must be a valid C string
 * - `params` must be either null or a valid pointer to FromPretrainedParametersFFI
 * Returns a pointer to the TokenizerHandle
 * The caller is responsible for freeing the memory using tokenizer_free()
 */
struct TokenizerHandle *tokenizer_from_pretrained(const char *name,
                                                  const struct CFromPretrainedParameters *params);

/**
 * Instantiate a new :class:`~tokenizers.Tokenizer` from the given buffer.
 *
 * Args:
 *     buffer (:obj:`bytes`):
 *         A buffer containing a previously serialized :class:`~tokenizers.Tokenizer`
 *
 * Returns:
 *     :class:`~tokenizers.Tokenizer`: The new tokenizer
 */
struct TokenizerHandle *tokenizer_from_buffer(const uint8_t *buffer, uintptr_t len);

/**
 * Creates a new `Tokenizer` from a file containing a serialized tokenizer
 *
 * # Safety
 * - path must be a valid null-terminated C string
 * - The file at path must contain a valid JSON serialized tokenizer
 * - The caller is responsible for freeing the memory using tokenizer_free()
 *
 * # Arguments
 * * path - Path to the file containing the serialized tokenizer
 *
 * # Returns
 * * Pointer to the TokenizerHandle on success
 * * NULL pointer on failure
 */
struct TokenizerHandle *tokenizer_from_file(const char *path);

/**
 * Encodes text using the given tokenizer
 *
 * # Safety
 * Handle must be a valid TokenizerHandle
 * Text must be a valid C string
 */
struct CEncoding *tokenizer_encode(struct TokenizerHandle *handle,
                                   const char *text,
                                   bool add_special_tokens);

/**
 * Decodes a sequence of ids into a string
 *
 * # Safety
 * Handle must be a valid TokenizerHandle
 * Ids must be a valid array of c_uint
 * Length must be the length of the ids array
 *
 * # Returns
 * A pointer to the decoded string.
 */
char *tokenizer_decode(struct TokenizerHandle *handle,
                       const unsigned int *ids,
                       uintptr_t length,
                       bool skip_special_tokens);

/**
 * He who allocates must deallocate
 * If rust, allocates the string, **DO NOT CALL `free()`**, allow rust
 * to deallocate it with `free_rstring`
 */
void free_rstring(char *s);
