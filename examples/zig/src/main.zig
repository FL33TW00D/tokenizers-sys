const std = @import("std");
const c = @cImport({
    @cInclude("ctokenizers.h");
});

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const params = c.CFromPretrainedParameters{
        .revision = null,
        .token = null,
    };

    const tokenizer = c.tokenizer_from_pretrained("bert-base-uncased", &params);
    if (tokenizer == null) {
        std.debug.print("Failed to create tokenizer\n", .{});
        return error.TokenizerCreationFailed;
    }
    defer c.tokenizer_free(tokenizer);

    const text = "Hello, world!";
    std.debug.print("Text to encode: {s}\n", .{text});
    const encoding = c.tokenizer_encode(tokenizer, text, true);
    if (encoding == null) {
        std.debug.print("Failed to encode text\n", .{});
        return error.EncodingFailed;
    }
    defer c.encoding_free(encoding);

    var length: usize = undefined;
    const ids = c.encoding_get_ids(encoding, &length);
    if (ids == null) {
        std.debug.print("Failed to get IDs\n", .{});
        return error.GetIdsFailed;
    }

    std.debug.print("Encoded IDs: ", .{});
    var i: usize = 0;
    while (i < length) : (i += 1) {
        std.debug.print("{} ", .{ids[i]});
    }
    std.debug.print("\n", .{});

    const decoded = c.tokenizer_decode(tokenizer, ids, length, true);
    if (decoded == null) {
        std.debug.print("Failed to decode text\n", .{});
        return error.DecodingFailed;
    }
    defer c.free_rstring(decoded);

    std.debug.print("Decoded text: {s}\n", .{decoded});
}
