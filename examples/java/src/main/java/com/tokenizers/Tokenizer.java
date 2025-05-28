package com.tokenizers;

/**
 * Java wrapper for the tokenizers-sys C bindings
 */
public class Tokenizer implements AutoCloseable {
    
    static {
        // Load the native library
        System.loadLibrary("tokenizers_jni");
    }

    private long handle;

    private Tokenizer(long handle) {
        this.handle = handle;
    }

    /**
     * Create a tokenizer from a pretrained model
     * @param name The model name (e.g., "bert-base-cased", "gpt2")
     * @return A new Tokenizer instance
     */
    public static Tokenizer fromPretrained(String name) {
        long handle = nativeFromPretrained(name, null, null);
        if (handle == 0) {
            throw new RuntimeException("Failed to load tokenizer: " + name);
        }
        return new Tokenizer(handle);
    }

    /**
     * Create a tokenizer from a pretrained model with parameters
     * @param name The model name
     * @param revision The model revision (can be null)
     * @param token The auth token (can be null)
     * @return A new Tokenizer instance
     */
    public static Tokenizer fromPretrained(String name, String revision, String token) {
        long handle = nativeFromPretrained(name, revision, token);
        if (handle == 0) {
            throw new RuntimeException("Failed to load tokenizer: " + name);
        }
        return new Tokenizer(handle);
    }

    /**
     * Create a tokenizer from a file
     * @param path Path to the tokenizer file
     * @return A new Tokenizer instance
     */
    public static Tokenizer fromFile(String path) {
        long handle = nativeFromFile(path);
        if (handle == 0) {
            throw new RuntimeException("Failed to load tokenizer from file: " + path);
        }
        return new Tokenizer(handle);
    }

    /**
     * Encode text into tokens
     * @param text The text to encode
     * @param addSpecialTokens Whether to add special tokens
     * @return An Encoding object
     */
    public Encoding encode(String text, boolean addSpecialTokens) {
        long encodingHandle = nativeEncode(handle, text, addSpecialTokens);
        if (encodingHandle == 0) {
            throw new RuntimeException("Failed to encode text");
        }
        return new Encoding(encodingHandle);
    }

    /**
     * Decode token IDs back to text
     * @param ids Array of token IDs
     * @param skipSpecialTokens Whether to skip special tokens
     * @return The decoded text
     */
    public String decode(int[] ids, boolean skipSpecialTokens) {
        return nativeDecode(handle, ids, skipSpecialTokens);
    }

    /**
     * Free the tokenizer resources
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
    private static native long nativeFromPretrained(String name, String revision, String token);
    private static native long nativeFromFile(String path);
    private static native long nativeEncode(long handle, String text, boolean addSpecialTokens);
    private static native String nativeDecode(long handle, int[] ids, boolean skipSpecialTokens);
    private static native void nativeFree(long handle);
}