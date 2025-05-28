import com.tokenizers.Tokenizer;
import com.tokenizers.Encoding;

/**
 * Example demonstrating how to use the tokenizers-sys Java bindings
 */
public class TokenizerExample {
    
    public static void main(String[] args) {
        String testText = "The quick brown fox jumps over the lazy dog.";
        
        System.out.println("=== Tokenizers-sys Java Example ===");
        System.out.println("Text to tokenize: " + testText);
        
        // Test BERT tokenizer
        testTokenizer("bert-base-cased", testText);
        
        // Test GPT-2 tokenizer
        testTokenizer("gpt2", testText);
    }
    
    private static void testTokenizer(String modelName, String text) {
        System.out.println("\n=== Testing " + modelName + " ===");
        
        try (Tokenizer tokenizer = Tokenizer.fromPretrained(modelName)) {
            // Encode the text
            try (Encoding encoding = tokenizer.encode(text, true)) {
                // Get encoding information
                int[] ids = encoding.getIds();
                String[] tokens = encoding.getTokens();
                int[] typeIds = encoding.getTypeIds();
                int[] specialTokensMask = encoding.getSpecialTokensMask();
                int[] attentionMask = encoding.getAttentionMask();
                
                System.out.println("Number of tokens: " + encoding.getLength());
                System.out.println("\nTokens:");
                for (int i = 0; i < tokens.length; i++) {
                    System.out.printf("  %s (ID: %d, Type: %d, Special: %d, Attention: %d)%n",
                        tokens[i], ids[i], typeIds[i], specialTokensMask[i], attentionMask[i]);
                }
                
                // Decode back to text
                String decoded = tokenizer.decode(ids, true);
                System.out.println("\nDecoded text: " + decoded);
                
                // Verify roundtrip
                if (text.trim().equals(decoded.trim())) {
                    System.out.println("✓ Roundtrip successful!");
                } else {
                    System.out.println("✗ Roundtrip failed!");
                    System.out.println("  Original: '" + text + "'");
                    System.out.println("  Decoded:  '" + decoded + "'");
                }
            }
        } catch (Exception e) {
            System.err.println("Error testing " + modelName + ": " + e.getMessage());
            e.printStackTrace();
        }
    }
}