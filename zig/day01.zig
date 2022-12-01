const std = @import("std");

pub fn main() !void {
    var input = @embedFile("input/day01");

    var max: u32 = 0;
    var elf_iter = std.mem.split(u8, input, "\n\n");
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var elf_list = std.ArrayList(u32).init(gpa.allocator());
    while (elf_iter.next()) |elf| {
        const trimmed = std.mem.trim(u8, elf, "\n");
        var calories_iter = std.mem.split(u8, trimmed, "\n");
        var sum: u32 = 0;
        while (calories_iter.next()) |calories| {
            sum += try std.fmt.parseInt(u32, calories, 10);
        }
        if (sum > max) {
            max = sum;
        }
        try elf_list.append(sum);
    }
    std.debug.print("Part 1: {}\n", .{max});

    var top_three: u32 = 0;
    var elf_slice = try elf_list.toOwnedSlice();
    std.sort.sort(u32, elf_slice, {}, std.sort.desc(u32));
    var i: u8 = 0;
    while (i < 3) {
        top_three += elf_slice[i];
        i += 1;
    }
    std.debug.print("Part 2: {}\n", .{top_three});
}
