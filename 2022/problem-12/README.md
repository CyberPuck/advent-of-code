# Problem 12 - Hill Climbing

## Part 1 - Shortest Path to `E`

The map is laid out with the starting position `S` and the ending position `E`.
There are alphabet characters in-between, you can only advance to the next
letter in the alphabet.  So `S` then *a*, *b*... *z*, *E*.  Find the shortest
path from `S` to `E`.

### Sample 1, Part 1 Solution

Based on the input map, the path should look like the following:
```
v..v<<<<
>v.vv<<^
.>vv>E^^
..v>>>^^
..>>>>>^
```

The final answer is **31** steps.