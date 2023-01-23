# Problem 10 - CPU Cycles

## Part 1 Calculate Register X Values

Given a set of "assembly-like" instructions, we need to find the given value of
the only register on the system (called `x`).

There are two instructions:

|Instruction|Cycles|Process|
|-----------|------|-------|
|noop|1|No value or state change|
|addx *i32*|2|Adds a signed integer to register `x` after 2 cycles|

We need to record the results of the 20th cycle then every 40th cycle after of
the `x` register given the instructions that will be processed.

### Sample 1

Given the sample 1 inputs, the results should produce the following:

- During the 20th cycle, register X has the value 21, so the signal strength is 20 * 21 = 420. (The 20th cycle occurs in the middle of the second addx -1, so the value of register X is the starting value, 1, plus all of the other addx values up to that point: 1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4 = 21.)
- During the 60th cycle, register X has the value 19, so the signal strength is 60 * 19 = 1140.
- During the 100th cycle, register X has the value 18, so the signal strength is 100 * 18 = 1800.
- During the 140th cycle, register X has the value 21, so the signal strength is 140 * 21 = 2940.
- During the 180th cycle, register X has the value 16, so the signal strength is 180 * 16 = 2880.
- During the 220th cycle, register X has the value 18, so the signal strength is 220 * 18 = 3960.

End result is: **13140**.

## Part 2 Drawing The Display

The display is 40 pixels wide by 6 pixels high.  It draws left to right, top to
bottom.  The left most position is 0, right most is 39.  A pixel is draw every
CPU cycle and each row has 40 CPU cycles to draw:

```TXT
Cycle   1 -> ######################################## <- Cycle  40
Cycle  41 -> ######################################## <- Cycle  80
Cycle  81 -> ######################################## <- Cycle 120
Cycle 121 -> ######################################## <- Cycle 160
Cycle 161 -> ######################################## <- Cycle 200
Cycle 201 -> ######################################## <- Cycle 240
```

- Lit pixels are defined by `#`
- Dark/un-lit pixels are defined by `.`

Every cycle the given pixel in the given row is drawn.  If the sprite is in
range, then a lit pixel is drawn.

## Sample 1 Output

The output should produce the following:

```TXT
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
```