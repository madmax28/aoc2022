const std = @import("std");

const Error = error{InvalidInput};

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const alloc = gpa.allocator();

pub fn main() !void {
    const input = @embedFile("input/day05");
    const trimmed = std.mem.trim(u8, input, "\n");
    defer _ = gpa.deinit();

    var parts = std.mem.split(u8, trimmed, "\n\n");

    var lines = std.mem.split(u8, parts.next().?, "\n");
    var stacks = std.ArrayList(std.ArrayList(u8)).init(alloc);
    defer stacks.deinit();
    while (lines.next()) |line| {
        var num_stacks = line.len / 4 + 1;
        if (stacks.items.len == 0) {
            try stacks.appendNTimes(std.ArrayList(u8).init(alloc), num_stacks);
        }

        while (num_stacks > 0) {
            var c: u8 = line[1 + 4 * (num_stacks - 1)];
            if (c != ' ') {
                try stacks.items[num_stacks - 1].append(c);
            }
            num_stacks -= 1;
        }
    }
    for (stacks.items) |stack| {
        std.mem.reverse(u8, stack.items);
    }

    lines = std.mem.split(u8, parts.next().?, "\n");
    var ops = std.ArrayList([3]usize).init(alloc);
    defer ops.deinit();
    while (lines.next()) |line| {
        var words = std.mem.split(u8, line, " ");
        _ = words.next();
        const count = try std.fmt.parseInt(usize, words.next().?, 10);
        _ = words.next();
        const from = try std.fmt.parseInt(usize, words.next().?, 10) - 1;
        _ = words.next();
        const to = try std.fmt.parseInt(usize, words.next().?, 10) - 1;
        try ops.append([3]usize{ count, from, to });
    }

    var stacks2 = try stacks.clone();
    defer stacks2.deinit();
    var idx: usize = 0;
    while (idx < stacks.items.len) {
        stacks2.items[idx] = try stacks.items[idx].clone();
        idx += 1;
    }

    for (ops.items) |op| {
        var i = op[0];
        while (i > 0) {
            try stacks.items[op[2]].append(stacks.items[op[1]].pop());
            i -= 1;
        }
    }

    std.debug.print("Part 1: ", .{});
    for (stacks.items) |*stack| {
        std.debug.print("{c}", .{stack.pop()});
        stack.deinit();
    }
    std.debug.print("\n", .{});

    for (ops.items) |op| {
        var tmp = std.ArrayList(u8).init(alloc);
        defer tmp.deinit();

        var i = op[0];
        while (i > 0) {
            try tmp.append(stacks2.items[op[1]].pop());
            i -= 1;
        }

        i = tmp.items.len;
        while (i > 0) {
            try stacks2.items[op[2]].append(tmp.items[i - 1]);
            i -= 1;
        }
    }

    std.debug.print("Part 2: ", .{});
    for (stacks2.items) |*stack| {
        std.debug.print("{c}", .{stack.pop()});
        stack.deinit();
    }
    std.debug.print("\n", .{});
}
