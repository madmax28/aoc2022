const std = @import("std");

const Error = error{InvalidInput};

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const alloc = gpa.allocator();

pub fn main() !void {
    const input = @embedFile("input/day06");
    const trimmed = std.mem.trim(u8, input, "\n");
    defer _ = gpa.deinit();

    var idx: usize = 0;
    while (idx < trimmed.len - 4) : (idx += 1) {
        var set = std.AutoHashMap(u8, bool).init(alloc);
        defer set.deinit();
        var needle: usize = 0;
        while (needle < 4) : (needle += 1) {
            try set.put(trimmed[idx + needle], true);
        }
        if (set.count() == 4) {
            std.debug.print("Part 1: {}\n", .{idx + 4});
            break;
        }
    }

    idx = 0;
    while (idx < trimmed.len - 14) : (idx += 1) {
        var set = std.AutoHashMap(u8, bool).init(alloc);
        defer set.deinit();
        var needle: usize = 0;
        while (needle < 14) : (needle += 1) {
            try set.put(trimmed[idx + needle], true);
        }
        if (set.count() == 14) {
            std.debug.print("Part 1: {}\n", .{idx + 14});
            break;
        }
    }
}
