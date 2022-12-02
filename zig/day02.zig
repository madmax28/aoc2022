const std = @import("std");

const Error = error{InvalidInput};

const Result = enum {
    Lose,
    Draw,
    Win,

    fn score(self: Result) i32 {
        return switch (self) {
            .Lose => 0,
            .Draw => 3,
            .Win => 6,
        };
    }

    fn from_u8(char: u8) !Result {
        return switch (char) {
            'X' => .Lose,
            'Y' => .Draw,
            'Z' => .Win,
            else => Error.InvalidInput,
        };
    }
};

const Move = enum {
    Rock,
    Paper,
    Scissors,

    fn score(self: Move) i32 {
        return switch (self) {
            .Rock => 1,
            .Paper => 2,
            .Scissors => 3,
        };
    }

    fn play(self: Move, other: Move) Result {
        return switch (self) {
            .Rock => return switch (other) {
                .Rock => .Draw,
                .Paper => .Lose,
                .Scissors => .Win,
            },
            .Paper => return switch (other) {
                .Rock => .Win,
                .Paper => .Draw,
                .Scissors => .Lose,
            },
            .Scissors => return switch (other) {
                .Rock => .Lose,
                .Paper => .Win,
                .Scissors => .Draw,
            },
        };
    }

    fn from_u8(char: u8) !Move {
        return switch (char) {
            'A', 'X' => .Rock,
            'B', 'Y' => .Paper,
            'C', 'Z' => .Scissors,
            else => Error.InvalidInput,
        };
    }
};

pub fn main() !void {
    const input = @embedFile("input/day02");
    const trimmed = std.mem.trim(u8, input, "\n");

    var lines = std.mem.split(u8, trimmed, "\n");
    var score: i32 = 0;
    while (lines.next()) |line| {
        const op = try Move.from_u8(line[0]);
        const me = try Move.from_u8(line[2]);
        score += me.score() + me.play(op).score();
    }
    std.debug.print("Part 1: {}\n", .{score});

    lines = std.mem.split(u8, trimmed, "\n");
    score = 0;
    while (lines.next()) |line| {
        const op = try Move.from_u8(line[0]);
        const res = try Result.from_u8(line[2]);
        var me: Move = .Rock;
        for ([_]Move{ .Rock, .Paper, .Scissors }) |cand| {
            if (cand.play(op) == res) {
                me = cand;
                break;
            }
        }
        score += me.score() + res.score();
    }
    std.debug.print("Part 2: {}\n", .{score});
}
