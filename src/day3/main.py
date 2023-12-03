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


def is_matching_criteria_2(
    prev_line, current_line, next_line, match: re.Match, max_len: int
) -> bool:
    return False


def solve(lines):
    total = 0
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
            is_match = is_matching_criteria_2(
                prev_line, current_line, next_line, asterix, len(lines[0])
            )
            if is_match:
                print(is_match, prev_line, current_line, next_line)
    return total


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
