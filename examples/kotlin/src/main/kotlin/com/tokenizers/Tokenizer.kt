package com.tokenizers

class Tokenizer private constructor(private val handle: Long) : AutoCloseable {
    
    companion object {
        init {
            System.loadLibrary("tokenizers_jni")
        }
        
        @JvmStatic
        fun fromPretrained(name: String, revision: String? = null, token: String? = null): Tokenizer {
            val handle = nativeFromPretrained(name, revision, token)
            if (handle == 0L) {
                throw RuntimeException("Failed to create tokenizer from pretrained model: $name")
            }
            return Tokenizer(handle)
        }
        
        @JvmStatic
        fun fromFile(path: String): Tokenizer {
            val handle = nativeFromFile(path)
            if (handle == 0L) {
                throw RuntimeException("Failed to create tokenizer from file: $path")
            }
            return Tokenizer(handle)
        }
        
        @JvmStatic
        private external fun nativeFromPretrained(name: String, revision: String?, token: String?): Long
        
        @JvmStatic
        private external fun nativeFromFile(path: String): Long
        
        @JvmStatic
        private external fun nativeEncode(handle: Long, text: String, addSpecialTokens: Boolean): Long
        
        @JvmStatic
        private external fun nativeDecode(handle: Long, ids: IntArray, skipSpecialTokens: Boolean): String?
        
        @JvmStatic
        private external fun nativeFree(handle: Long)
    }
    
    fun encode(text: String, addSpecialTokens: Boolean = true): Encoding {
        val encodingHandle = nativeEncode(handle, text, addSpecialTokens)
        if (encodingHandle == 0L) {
            throw RuntimeException("Failed to encode text: $text")
        }
        return Encoding(encodingHandle)
    }
    
    fun decode(ids: IntArray, skipSpecialTokens: Boolean = true): String {
        return nativeDecode(handle, ids, skipSpecialTokens)
            ?: throw RuntimeException("Failed to decode token IDs")
    }
    
    override fun close() {
        if (handle != 0L) {
            nativeFree(handle)
        }
    }
}