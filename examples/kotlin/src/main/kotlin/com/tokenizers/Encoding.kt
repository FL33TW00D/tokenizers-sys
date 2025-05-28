package com.tokenizers

class Encoding internal constructor(private val handle: Long) : AutoCloseable {
    
    companion object {
        @JvmStatic
        private external fun nativeGetLength(handle: Long): Int
        
        @JvmStatic
        private external fun nativeGetIds(handle: Long): IntArray?
        
        @JvmStatic
        private external fun nativeGetTokens(handle: Long): Array<String>?
        
        @JvmStatic
        private external fun nativeGetTypeIds(handle: Long): IntArray?
        
        @JvmStatic
        private external fun nativeGetSpecialTokensMask(handle: Long): IntArray?
        
        @JvmStatic
        private external fun nativeGetAttentionMask(handle: Long): IntArray?
        
        @JvmStatic
        private external fun nativeFree(handle: Long)
    }
    
    val length: Int
        get() = nativeGetLength(handle)
    
    fun getIds(): IntArray {
        return nativeGetIds(handle) ?: throw RuntimeException("Failed to get token IDs")
    }
    
    fun getTokens(): Array<String> {
        return nativeGetTokens(handle) ?: throw RuntimeException("Failed to get tokens")
    }
    
    fun getTypeIds(): IntArray {
        return nativeGetTypeIds(handle) ?: throw RuntimeException("Failed to get type IDs")
    }
    
    fun getSpecialTokensMask(): IntArray {
        return nativeGetSpecialTokensMask(handle) ?: throw RuntimeException("Failed to get special tokens mask")
    }
    
    fun getAttentionMask(): IntArray {
        return nativeGetAttentionMask(handle) ?: throw RuntimeException("Failed to get attention mask")
    }
    
    override fun close() {
        if (handle != 0L) {
            nativeFree(handle)
        }
    }
}