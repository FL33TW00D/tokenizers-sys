import com.tokenizers.Tokenizer
import com.tokenizers.Encoding

fun main() {
    val testText = "The quick brown fox jumps over the lazy dog."
    
    println("=== Tokenizers-sys Kotlin Example ===")
    println("Text to tokenize: $testText")
    
    // Test BERT tokenizer
    testTokenizer("bert-base-cased", testText)
    
    // Test GPT-2 tokenizer
    testTokenizer("gpt2", testText)
}

private fun testTokenizer(modelName: String, text: String) {
    println("\n=== Testing $modelName ===")
    
    try {
        Tokenizer.fromPretrained(modelName).use { tokenizer ->
            // Encode the text
            tokenizer.encode(text, true).use { encoding ->
                // Get encoding information
                val ids = encoding.getIds()
                val tokens = encoding.getTokens()
                val typeIds = encoding.getTypeIds()
                val specialTokensMask = encoding.getSpecialTokensMask()
                val attentionMask = encoding.getAttentionMask()
                
                println("Number of tokens: ${encoding.length}")
                println("\nTokens:")
                for (i in tokens.indices) {
                    println("  ${tokens[i]} (ID: ${ids[i]}, Type: ${typeIds[i]}, Special: ${specialTokensMask[i]}, Attention: ${attentionMask[i]})")
                }
                
                // Decode back to text
                val decoded = tokenizer.decode(ids, true)
                println("\nDecoded text: $decoded")
                
                // Verify roundtrip
                if (text.trim() == decoded.trim()) {
                    println("✓ Roundtrip successful!")
                } else {
                    println("✗ Roundtrip failed!")
                    println("  Original: '$text'")
                    println("  Decoded:  '$decoded'")
                }
            }
        }
    } catch (e: Exception) {
        System.err.println("Error testing $modelName: ${e.message}")
        e.printStackTrace()
    }
}
