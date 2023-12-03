import os
import re
import sys
from itertools import zip_longest


ints = set(map(str, list(range(10))))


def is_matching_criteria(
    prev_line, current_line, next_line, match: re.Match, max_len: int
) -> bool:
    for i in range(match.start() - 1, match.end() + 1):
        if i < 0 or i == max_len:
            continue

        if prev_line is not None:
            if prev_line[i] != "." and prev_line[i] not in ints:
                return True
        if next_line is not None:
            if next_line[i] != "." and next_line[i] not in ints:
                return True

        if i == match.start() - 1 or i == match.end():
            if current_line[i] != "." and current_line[i] not in ints:
                return True

    return False


def get_gear_match(prev_line, current_line, next_line, gear: re.Match, max_len: int):
    gear_matches = []
    if prev_line:
        matches = re.finditer(r"(\d+)", prev_line)
        for match in matches:
            if match.start() - 1 <= gear.start() <= match.end():
                gear_matches.append(match.group())

    string_on_left = current_line[: gear.start()]
    string_on_right = current_line[gear.start() + 1 :]
    number_left = re.search(r"(\d+)$", string_on_left)
    number_right = re.search(r"^(\d+)", string_on_right)
    if number_left:
        gear_matches.append(number_left.group())
    if number_right:
        gear_matches.append(number_right.group())

    if next_line:
        matches = re.finditer(r"(\d+)", next_line)
        for match in matches:
            if match.start() - 1 <= gear.start() <= match.end():
                gear_matches.append(match.group())

    # print(gear_matches)
    if len(gear_matches) == 2:
        return int(gear_matches[0]) * int(gear_matches[1])

    return 0


def solve(lines):
    total = 0
    total_gears = 0
    for prev_line, current_line, next_line in zip_longest(
        [None, *lines], lines, lines[1:]
    ):
        if current_line is None:
            continue

        for number in re.finditer(r"(\d+)", current_line):
            is_match = is_matching_criteria(
                prev_line, current_line, next_line, number, len(lines[0])
            )
            if is_match:
                total += int(number.group(1))

        for asterix in re.finditer(r"(\*)", current_line):
            gear = get_gear_match(
                prev_line, current_line, next_line, asterix, len(lines[0])
            )
            total_gears += gear
    return total, total_gears


def main():
    filename = sys.argv[1:]

    if not filename:
        print("give a file")
        sys.exit(1)

    with open(filename[0], "r") as f:
        lines = [line.strip() for line in f.readlines()]
        total = solve(lines)
        print(total)


if __name__ == "__main__":
    main()
