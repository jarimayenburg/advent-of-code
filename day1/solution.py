#! /usr/bin/env python3

count = 0

with open('input.txt', 'r') as file:
    lines = file.readlines()

    prevline = lines[0]
    for line in lines[1:]:
        if (int(line.rstrip()) > int(prevline.rstrip())):
            count += 1
        prevline = line

print(count)
