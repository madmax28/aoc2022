const std = @import("std");

const Error = error{InvalidInput};

const Range = struct {
    from: i32,
    to: i32,

    fn from_u8_slice(slice: []const u8) !Range {
        var nums = std.mem.split(u8, slice, "-");
        const from = try std.fmt.parseInt(i32, nums.next().?, 10);
        const to = try std.fmt.parseInt(i32, nums.next().?, 10);
        return Range{ .from = from, .to = to };
    }
};

const Pair = struct {
    first: Range,
    second: Range,

    fn from_u8_slice(slice: []const u8) !Pair {
        var ranges = std.mem.split(u8, slice, ",");
        const first = try Range.from_u8_slice(ranges.next().?);
        const second = try Range.from_u8_slice(ranges.next().?);
        return Pair{ .first = first, .second = second };
    }

    fn contained(pair: Pair) bool {
        return pair.first.from <= pair.second.from and pair.first.to >= pair.second.to
            or pair.second.from <= pair.first.from and pair.second.to >= pair.first.to;
    }

    fn overlap(pair: Pair) bool {
        return pair.first.to >= pair.second.from and pair.first.from <= pair.second.to;
    }
};

fn count(pairs: []Pair, check: fn (*Pair) bool) i32 {
    var res = 0;
    for (pairs) |pair| {
        if (check(pair)) {
            res += 1;
        }
    }
    return res;
}

pub fn main() !void {
    const input = @embedFile("input/day04");
    const trimmed = std.mem.trim(u8, input, "\n");

    var pairs = std.mem.split(u8, trimmed, "\n");
    var res: i32 = 0;
    while (pairs.next()) |pair_s| {
        const pair = try Pair.from_u8_slice(pair_s);
        if (pair.contained()) {
            res += 1;
        }
    }
    std.debug.print("Part 1: {}\n", .{res});

    pairs = std.mem.split(u8, trimmed, "\n");
    res = 0;
    while (pairs.next()) |pair_s| {
        const pair = try Pair.from_u8_slice(pair_s);
        if (pair.overlap()) {
            res += 1;
        }
    }
    std.debug.print("Part 2: {}\n", .{res});
}
