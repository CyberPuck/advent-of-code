# The Quest to win Rock-Paper-Scissors (RPS)

The Elfs are setting up camp, everyone wants to be near the snacks.
To select who gets closest, everyone is participating in a game of RPS.

## Points 

The competition follows the basic rules of RPS, with the following points:

|Move|Points|
|----|------|
|Rock|1|
|Paper|2|
|Scissors|3|

|Round Result|Points|
|-----------|------|
|Loss|0|
|Draw|3|
|Win|6|

## Strategy Guide From Mysterious Elf

One of the elfs gives you a strategy guide (unknown if this is a ploy).

### The Guide Key

- Oppoents' move is the first column
   - `A` = Rock
   - `B` = Paper
   - `C` = Scissors
- Your move is the second colum?
   - `X` = Rock
   - `Y` = Paper
   - `Z` = Scissors

## Scoring

When playing a round of the game the total score is:
> Move played + round result

Your total score is the cumulative of all rounds played.

## Sample Guide

```BASH
A Y
B X
C Z
```

If this guide is followed:
1. Rock vs Paper, 2 paper + 6 win = 8
2. Paper vs Rock, 1 rock + 0 loss = 1
3. Scissors vs Scissors, 3 scissors + 3 draw = 6

Total score: 15

# Part 2: The elf is helping and the file is different

The elf gets back to you, turns out the second column is actually the final
state the match should end at.  The second column should translate to:

|Indicator|Round Result|
|---------|------------|
|`X`|loss|
|`Y`|draw|
|`Z`|win|

## Sample File Part 2

1. Opponent rock, draw `Y`, pick rock (1 + 3 = 4)
2. Opponent paper, lose `X`, pick rock (1 + 0 = 1)
3. Opponent scissors, win `Z`, pick rock (1 + 6 = 7)

Total: 12