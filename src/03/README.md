# [2025/03](https://adventofcode.com/2025/day/3) - Lobby

- Part 1 initially solved naively, iterate through each bank as a string finding the max value, and if its the last then it becomes the less significant value, and do the same again
- Part 2 had me realise a much easier (and faster) solution:
    - Go through bank as a string, removing any battery whose value is less than the previous until the length of the bank == the number of required batteries OR no value is removed, in which case truncate the bank to the required length 
