from __future__ import annotations

import sys
from pathlib import Path


def insert_before_occurrence(lines: list[str], needle: str, insert_line: str, occurrence: int = 1) -> None:
    seen = 0

    for index, line in enumerate(lines):
        if line != needle:
            continue

        seen += 1
        if seen != occurrence:
            continue

        previous_index = index - 1
        while previous_index >= 0 and lines[previous_index].strip() == "":
            previous_index -= 1

        if previous_index >= 0 and lines[previous_index] == insert_line:
            return

        lines.insert(index, insert_line)
        return

    raise ValueError(f"Could not find occurrence {occurrence} of line: {needle}")


def insert_before_all_occurrences(lines: list[str], needle: str, insert_line: str) -> None:
    index = 0

    while index < len(lines):
        if lines[index] != needle:
            index += 1
            continue

        previous_index = index - 1
        while previous_index >= 0 and lines[previous_index].strip() == "":
            previous_index -= 1

        if previous_index >= 0 and lines[previous_index] == insert_line:
            index += 1
            continue

        lines.insert(index, insert_line)
        index += 2


def insert_block_before_occurrence(lines: list[str], needle: str, block: list[str]) -> None:
    for index, line in enumerate(lines):
        if line != needle:
            continue

        block_start = index - len(block)
        if block_start >= 0 and lines[block_start:index] == block:
            return

        for block_line in reversed(block):
            lines.insert(index, block_line)

        return

    raise ValueError(f"Could not find line: {needle}")


def insert_before_occurrence_or_previous(
    lines: list[str], needle: str, insert_line: str, previous_line: str
) -> None:
    for index, line in enumerate(lines):
        if line != needle:
            continue

        insert_index = index
        if index > 0 and lines[index - 1] == previous_line:
            insert_index = index - 1

        previous_index = insert_index - 1
        while previous_index >= 0 and lines[previous_index].strip() == "":
            previous_index -= 1

        if previous_index >= 0 and lines[previous_index] == insert_line:
            return

        lines.insert(insert_index, insert_line)
        return

    raise ValueError(f"Could not find line: {needle}")


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: python3 postprocess_generated_lib_rs.py <path-to-lib.rs>", file=sys.stderr)
        return 1

    lib_rs_path = Path(sys.argv[1])
    if not lib_rs_path.is_file():
        print(f"File not found: {lib_rs_path}", file=sys.stderr)
        return 1

    lines = lib_rs_path.read_text().splitlines()

    insert_before_occurrence(lines, '#[allow(unused_imports)]', '#[cfg(feature = "client")]', 1)
    insert_before_occurrence(lines, '#[allow(unused_imports)]', '#[cfg(feature = "client")]', 2)
    insert_block_before_occurrence(
        lines,
        '/// Types used as operation parameters and responses.',
        ['#[cfg(feature = "client")]', 'pub mod progenitor_client;', ''],
    )
    insert_before_occurrence(
        lines,
        '/// Types used as operation parameters and responses.',
        '#[cfg(feature = "types")]',
    )
    insert_before_occurrence_or_previous(
        lines,
        '///Client for DSH Tenant Resource Management REST API',
        '#[cfg(feature = "client")]',
        '#[derive(Clone, Debug)]',
    )
    insert_before_all_occurrences(lines, 'impl Client {', '#[cfg(feature = "client")]')
    insert_before_occurrence(
        lines,
        'impl ClientInfo<()> for Client {',
        '#[cfg(feature = "client")]',
    )
    insert_before_occurrence(
        lines,
        'impl ClientHooks<()> for &Client {}',
        '#[cfg(feature = "client")]',
    )
    insert_before_occurrence(
        lines,
        '/// Items consumers will typically use such as the Client.',
        '#[cfg(feature = "client")]',
    )

    lib_rs_path.write_text('\n'.join(lines) + '\n')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())