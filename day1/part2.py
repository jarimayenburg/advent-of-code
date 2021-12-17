#! /usr/bin/env python3

count = 0

with open('input.txt', 'r') as file:
    lines = [int(line) for line in file.readlines()]

    prevwindow = lines[0:3]
    for i in range(4, len(lines) + 1):
        window = lines[i-3:i]

        if (sum(prevwindow) < sum(window)):
            count += 1

        prevwindow = window

print(count)
