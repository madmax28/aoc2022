const std = @import("std");
const Error = error{InvalidInput};

fn priority(c: u8) i32 {
    if (std.ascii.isLower(c)) {
        return c - 96;
    } else {
        return c - 64 + 26;
    }
}

pub fn main() !void {
    const input = @embedFile("input/day03");
    const trimmed = std.mem.trim(u8, input, "\n");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    var alloc = gpa.allocator();

    var elves = std.mem.split(u8, trimmed, "\n");
    var res: i32 = 0;
    while (elves.next()) |elf| {
        var comp1 = std.AutoHashMap(u8, bool).init(alloc);
        defer comp1.deinit();
        var i: usize = 0;
        while (i < elf.len / 2) {
            try comp1.put(elf[i], true);
            i += 1;
        }

        while (i < elf.len) {
            if (comp1.contains(elf[i])) {
                res += priority(elf[i]);
                break;
            }
            i += 1;
        }
    }
    std.debug.print("Part 1: {}\n", .{res});

    res = 0;
    elves = std.mem.split(u8, trimmed, "\n");
    while (elves.next()) |elf| {
        var items = std.AutoHashMap(u8, bool).init(alloc);
        var i: i32 = 0;
        while (i < 3) {
            if (i == 0) {
                for (elf) |item| {
                    try items.put(item, true);
                }
            } else {
                var items_new = std.AutoHashMap(u8, bool).init(alloc);
                for (elves.next().?) |item| {
                    if (items.contains(item)) {
                        try items_new.put(item, true);
                    }
                }
                items.deinit();
                items = items_new;
            }

            i += 1;
        }

        var iter = items.iterator();
        res += priority(iter.next().?.key_ptr.*);
        items.deinit();
    }
    std.debug.print("Part 2: {}\n", .{res});
}
