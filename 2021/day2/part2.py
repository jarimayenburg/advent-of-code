#! /usr/bin/env python3

with open('input.txt', 'r') as file:
    pos, depth, aim = 0, 0, 0

    for line in file:
        (command, val) = line.strip().split(' ')

        val = int(val)

        if command == "forward":
            pos += val
            depth += aim * val
        elif command == "down":
            aim += val
        elif command == "up":
            aim -= val

    print(pos * depth)
