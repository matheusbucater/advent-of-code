from more_itertools import windowed
import random

"""
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
"""

outputs = ["29", "83", "13", "24", "42", "14", "76"]
digits = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "1": 1,
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9
}
values = []
with open("./day-01/input.txt") as file:
  for line in file.readlines():
    first = 0
    last = 0
    line = line.strip()

    first_found = False
    while not first_found:
      print(line)
      for num, n in digits.items():
        if line.startswith(num):
          print(n)
          first = n
          first_found = True
      line = line[1:]

    last_found = False
    while not last_found:
      print(line)
      for num, n in digits.items():
        if line.endswith(num):
          print(n)
          last = n
          last_found = True
        if line == "":
          last = first
          last_found = True
      line = line[:-1]
    
    value = first * 10 + last
    print(value)
    values.append(value)

answer = sum([val for val in values])
print(answer)






"""
- - - - -
_ _ _ _ _
0 1 2 3 4

  - - - - -
- - - - -
_ _ _ _ _ _
0 1 2 3 4 5

    - - - - -
  - - - - -
- - - - -
_ _ _ _ _ _ _
0 1 2 3 4 5 6

      - - - - -
    - - - - -
  - - - - -
- - - - -
_ _ _ _ _ _ _ _
0 1 2 3 4 5 6 7

        - - - - -
      - - - - -
    - - - - -
  - - - - -
- - - - -
_ _ _ _ _ _ _ _ _
0 1 2 3 4 5 6 7 8
"""