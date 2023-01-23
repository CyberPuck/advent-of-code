# Problem 10 - CPU Cycles

Given a set of "assembly-like" instructions, we need to find the given value of
the only register on the system (called `x`).

There are two instructions:

|Instruction|Cycles|Process|
|-----------|------|-------|
|noop|1|No value or state change|
|addx *i32*|2|Adds a signed integer to register `x` after 2 cycles|

We need to record the results of the 20th cycle then every 40th cycle after of
the `x` register given the instructions that will be processed.

## Sample 1

Given the sample 1 inputs, the results should produce the following:

- During the 20th cycle, register X has the value 21, so the signal strength is 20 * 21 = 420. (The 20th cycle occurs in the middle of the second addx -1, so the value of register X is the starting value, 1, plus all of the other addx values up to that point: 1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4 = 21.)
- During the 60th cycle, register X has the value 19, so the signal strength is 60 * 19 = 1140.
- During the 100th cycle, register X has the value 18, so the signal strength is 100 * 18 = 1800.
- During the 140th cycle, register X has the value 21, so the signal strength is 140 * 21 = 2940.
- During the 180th cycle, register X has the value 16, so the signal strength is 180 * 16 = 2880.
- During the 220th cycle, register X has the value 18, so the signal strength is 220 * 18 = 3960.

End result is: **13140**.