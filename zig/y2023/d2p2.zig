const std = @import("std");

const MAX_BUFFER_SIZE = 50000;

pub fn main() !void {
    const root = std.fs.cwd();

    var input_buffer: [MAX_BUFFER_SIZE]u8 = undefined;
    const file_contents = try root.readFile("input.txt", &input_buffer);

    var sum: u32 = 0;
    var curr_index: u32 = 0;

    var line_char_count: u8 = 0;
    var valid_line: bool = true;
    var inside_game: bool = false;

    var curr_val: ?u32 = null;

    var minimum_red: u32 = 0;
    var minimum_blue: u32 = 0;
    var minimum_green: u32 = 0;

    while (curr_index < file_contents.len) : (curr_index += 1) {
        const curr_char = &file_contents[curr_index];

        if (curr_char.* == '\n') {
            line_char_count = 0;
            if (valid_line) {
                sum += minimum_red * minimum_blue * minimum_green;
            }

            minimum_green = 0;
            minimum_red = 0;
            minimum_blue = 0;
            valid_line = true;
            inside_game = false;
            continue;
        } else {
            defer {
                line_char_count += 1;
            }

            if (curr_char.* == ':') {
                inside_game = true;
                continue;
            }

            if (!valid_line or curr_char.* == ' ' or curr_char.* == ',' or curr_char.* == ';' or !inside_game) continue;

            if (std.ascii.isDigit(curr_char.*)) {
                curr_val = ((curr_val orelse 0) * 10) + (curr_char.* - '0');
                continue;
            }

            switch (curr_char.*) {
                'r' => {
                    if (curr_val.? > minimum_red) {
                        minimum_red = curr_val.?;
                    }
                    curr_index += 2;
                    line_char_count += 2;
                    curr_val = null;
                },
                'b' => {
                    if (curr_val.? > minimum_blue) {
                        minimum_blue = curr_val.?;
                    }
                    curr_index += 3;
                    line_char_count += 3;
                    curr_val = null;
                },
                'g' => {
                    if (curr_val.? > minimum_green) {
                        minimum_green = curr_val.?;
                    }
                    curr_index += 4;
                    line_char_count += 4;
                    curr_val = null;
                },
                else => {},
            }
        }
    }

    std.debug.print("{}\n", .{sum});
}
