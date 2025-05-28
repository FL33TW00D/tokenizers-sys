package com.tokenizers;

/**
 * Represents an encoded sequence from the tokenizer
 */
public class Encoding implements AutoCloseable {
    
    private long handle;

    public Encoding(long handle) {
        this.handle = handle;
    }

    /**
     * Get the number of tokens
     * @return The length of the encoding
     */
    public int getLength() {
        return nativeGetLength(handle);
    }

    /**
     * Get the token IDs
     * @return Array of token IDs
     */
    public int[] getIds() {
        return nativeGetIds(handle);
    }

    /**
     * Get the tokens as strings
     * @return Array of token strings
     */
    public String[] getTokens() {
        return nativeGetTokens(handle);
    }

    /**
     * Get the type IDs
     * @return Array of type IDs
     */
    public int[] getTypeIds() {
        return nativeGetTypeIds(handle);
    }

    /**
     * Get the special tokens mask
     * @return Array indicating which tokens are special
     */
    public int[] getSpecialTokensMask() {
        return nativeGetSpecialTokensMask(handle);
    }

    /**
     * Get the attention mask
     * @return Array for attention mask
     */
    public int[] getAttentionMask() {
        return nativeGetAttentionMask(handle);
    }

    /**
     * Free the encoding resources
     */
    @Override
    public void close() {
        if (handle != 0) {
            nativeFree(handle);
            handle = 0;
        }
    }

    @Override
    protected void finalize() throws Throwable {
        close();
        super.finalize();
    }

    // Native method declarations
    private static native int nativeGetLength(long handle);
    private static native int[] nativeGetIds(long handle);
    private static native String[] nativeGetTokens(long handle);
    private static native int[] nativeGetTypeIds(long handle);
    private static native int[] nativeGetSpecialTokensMask(long handle);
    private static native int[] nativeGetAttentionMask(long handle);
    private static native void nativeFree(long handle);
}