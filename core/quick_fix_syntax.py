#!/usr/bin/env python3

print("Quick syntax fix...")

with open('tests/integration_tests.rs', 'r') as f:
    lines = f.readlines()

# Count braces to find syntax errors
open_count = sum(1 for line in lines for char in line if char == '{')
close_count = sum(1 for line in lines for char in line if char == '}')

print(f"Open braces: {open_count}")
print(f"Close braces: {close_count}")

# If mismatched, add missing brace at the end
if open_count > close_count:
    with open('tests/integration_tests.rs', 'a') as f:
        for _ in range(open_count - close_count):
            f.write('}\n')
    print(f"Added {open_count - close_count} closing braces")

print("✅ Syntax fix applied")