#include <jni.h>
#include <stdlib.h>
#include "../../bindings/ctokenizers.h"

static jstring c_string_to_jstring(JNIEnv *env, const char *c_str) {
    if (!c_str) return NULL;
    return (*env)->NewStringUTF(env, c_str);
}

static const char* jstring_to_c_string(JNIEnv *env, jstring jstr) {
    if (!jstr) return NULL;
    return (*env)->GetStringUTFChars(env, jstr, NULL);
}

static void release_c_string(JNIEnv *env, jstring jstr, const char *c_str) {
    if (c_str) (*env)->ReleaseStringUTFChars(env, jstr, c_str);
}

static jintArray uint32_array_to_jintarray(JNIEnv *env, const uint32_t *array, uintptr_t length) {
    if (!array) return NULL;
    
    jintArray result = (*env)->NewIntArray(env, length);
    if (!result) return NULL;
    
    jint *java_array = malloc(length * sizeof(jint));
    for (uintptr_t i = 0; i < length; i++) {
        java_array[i] = (jint)array[i];
    }
    
    (*env)->SetIntArrayRegion(env, result, 0, length, java_array);
    free(java_array);
    return result;
}

static unsigned int* jintarray_to_uint_array(JNIEnv *env, jintArray jarray, jsize *length) {
    if (!jarray) return NULL;
    
    *length = (*env)->GetArrayLength(env, jarray);
    jint *java_array = (*env)->GetIntArrayElements(env, jarray, NULL);
    if (!java_array) return NULL;
    
    unsigned int *uint_array = malloc(*length * sizeof(unsigned int));
    for (jsize i = 0; i < *length; i++) {
        uint_array[i] = (unsigned int)java_array[i];
    }
    
    (*env)->ReleaseIntArrayElements(env, jarray, java_array, JNI_ABORT);
    return uint_array;
}

JNIEXPORT jlong JNICALL Java_com_tokenizers_Tokenizer_nativeFromPretrained
  (JNIEnv *env, jclass cls, jstring name, jstring revision, jstring token) {
    
    const char *name_str = jstring_to_c_string(env, name);
    if (!name_str) return 0;
    
    struct CFromPretrainedParameters params = {0};
    const char *revision_str = jstring_to_c_string(env, revision);
    const char *token_str = jstring_to_c_string(env, token);
    
    if (revision_str) params.revision = revision_str;
    if (token_str) params.token = token_str;
    
    struct TokenizerHandle *handle = tokenizer_from_pretrained(name_str, &params);
    
    release_c_string(env, name, name_str);
    release_c_string(env, revision, revision_str);
    release_c_string(env, token, token_str);
    
    return (jlong)handle;
}

JNIEXPORT jlong JNICALL Java_com_tokenizers_Tokenizer_nativeFromFile
  (JNIEnv *env, jclass cls, jstring path) {
    
    const char *path_str = jstring_to_c_string(env, path);
    if (!path_str) return 0;
    
    struct TokenizerHandle *handle = tokenizer_from_file(path_str);
    release_c_string(env, path, path_str);
    
    return (jlong)handle;
}

JNIEXPORT jlong JNICALL Java_com_tokenizers_Tokenizer_nativeEncode
  (JNIEnv *env, jclass cls, jlong handle, jstring text, jboolean addSpecialTokens) {
    
    if (handle == 0) return 0;
    
    const char *text_str = jstring_to_c_string(env, text);
    if (!text_str) return 0;
    
    struct CEncoding *encoding = tokenizer_encode((struct TokenizerHandle *)handle, text_str, addSpecialTokens);
    release_c_string(env, text, text_str);
    
    return (jlong)encoding;
}

JNIEXPORT jstring JNICALL Java_com_tokenizers_Tokenizer_nativeDecode
  (JNIEnv *env, jclass cls, jlong handle, jintArray ids, jboolean skipSpecialTokens) {
    
    if (handle == 0) return NULL;
    
    jsize length;
    unsigned int *uint_ids = jintarray_to_uint_array(env, ids, &length);
    if (!uint_ids) return NULL;
    
    char *decoded = tokenizer_decode((struct TokenizerHandle *)handle, uint_ids, length, skipSpecialTokens);
    free(uint_ids);
    
    if (!decoded) return NULL;
    
    jstring result = c_string_to_jstring(env, decoded);
    free_rstring(decoded);
    
    return result;
}

JNIEXPORT void JNICALL Java_com_tokenizers_Tokenizer_nativeFree
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle != 0) {
        tokenizer_free((struct TokenizerHandle *)handle);
    }
}

JNIEXPORT jint JNICALL Java_com_tokenizers_Encoding_nativeGetLength
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle == 0) return 0;
    return (jint)encoding_get_length((struct CEncoding *)handle);
}

JNIEXPORT jintArray JNICALL Java_com_tokenizers_Encoding_nativeGetIds
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle == 0) return NULL;
    
    uintptr_t length;
    const uint32_t *ids = encoding_get_ids((struct CEncoding *)handle, &length);
    return uint32_array_to_jintarray(env, ids, length);
}

JNIEXPORT jobjectArray JNICALL Java_com_tokenizers_Encoding_nativeGetTokens
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle == 0) return NULL;
    
    uintptr_t length;
    char **tokens = encoding_get_tokens((struct CEncoding *)handle, &length);
    if (!tokens) return NULL;
    
    jclass stringClass = (*env)->FindClass(env, "java/lang/String");
    jobjectArray result = (*env)->NewObjectArray(env, length, stringClass, NULL);
    
    if (!result) {
        free_c_char_array(tokens, length);
        return NULL;
    }
    
    for (uintptr_t i = 0; i < length; i++) {
        jstring token = c_string_to_jstring(env, tokens[i]);
        (*env)->SetObjectArrayElement(env, result, i, token);
        (*env)->DeleteLocalRef(env, token);
    }
    
    free_c_char_array(tokens, length);
    return result;
}

JNIEXPORT jintArray JNICALL Java_com_tokenizers_Encoding_nativeGetTypeIds
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle == 0) return NULL;
    
    uintptr_t length;
    const uint32_t *type_ids = encoding_get_type_ids((struct CEncoding *)handle, &length);
    return uint32_array_to_jintarray(env, type_ids, length);
}

JNIEXPORT jintArray JNICALL Java_com_tokenizers_Encoding_nativeGetSpecialTokensMask
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle == 0) return NULL;
    
    uintptr_t length;
    const uint32_t *mask = encoding_get_special_tokens_mask((struct CEncoding *)handle, &length);
    return uint32_array_to_jintarray(env, mask, length);
}

JNIEXPORT jintArray JNICALL Java_com_tokenizers_Encoding_nativeGetAttentionMask
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle == 0) return NULL;
    
    uintptr_t length;
    const uint32_t *mask = encoding_get_attention_mask((struct CEncoding *)handle, &length);
    return uint32_array_to_jintarray(env, mask, length);
}

JNIEXPORT void JNICALL Java_com_tokenizers_Encoding_nativeFree
  (JNIEnv *env, jclass cls, jlong handle) {
    if (handle != 0) {
        encoding_free((struct CEncoding *)handle);
    }
}
