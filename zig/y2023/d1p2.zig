const std = @import("std");

const numbers: [9][:0]const u8 = .{
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
};

pub fn main() void {
    const root = std.fs.cwd();

    var buffer: [50000]u8 = undefined;
    const file_contents = root.readFile("input.txt", &buffer) catch unreachable;

    var start_char: ?u8 = null;
    var end_char: ?u8 = null;
    var total: i32 = 0;

    for (file_contents, 0..) |c, i| {
        if (c == '\n') {
            total += (start_char.? * 10) + end_char.?;
            start_char = null;
            end_char = null;
        } else if (std.ascii.isDigit(c)) {
            if (start_char == null) {
                start_char = c;
            }

            end_char = c;
        } else {
            var possible_indexes: [9]?u8 = .{ 0, 1, 2, 3, 4, 5, 6, 7, 8 };

            word_check: for (file_contents[i..file_contents.len], 0..) |inner_c, word_i| {
                for (possible_indexes) |number_index_opt| {
                    if (number_index_opt) |number_index| {
                        const current_number = &numbers[number_index];

                        if (current_number.*[word_i] == inner_c) {
                            if (word_i == (current_number.len - 1)) {
                                if (start_char == null) {
                                    start_char = number_index + 1 + '0';
                                }

                                end_char = number_index + 1 + '0';
                                break :word_check;
                            }
                        } else {
                            possible_indexes[number_index] = null;
                        }
                    } else continue;
                }
            }
        }
    }

    std.debug.print("{}\n", .{total});
}
