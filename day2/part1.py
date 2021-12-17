#! /usr/bin/env python3

with open('input.txt', 'r') as file:
    pos, depth = 0, 0

    for line in file:
        (command, val) = line.strip().split(' ')

        val = int(val)

        if command == "forward":
            pos += val
        elif command == "down":
            depth += val
        elif command == "up":
            depth -= val

print(pos * depth)
