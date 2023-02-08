# Problem 12 - Hill Climbing

## Part 1 - Shortest Path to `E`

The map is laid out with the starting position `S` and the ending position `E`.
There are alphabet characters in-between, you can advance to the next
letter in the alphabet.  So `S` then *a*, *b*... *z*, *E*.  Find the shortest
path from `S` to `E`.

### Movement

This is a little tricky, you can only advance one letter of the alpebet every
step.  That means going from *a* to *b*, but you can also travel to a space
that is equal to your space, *c* to *c*.  Please note there is a description
about:

> This also means that the elevation of the destination square can be much
lower than the elevation of your current square.

Yikes, so it is next step = current step + 1 || next step <= current_step.

### Map Coordinates

Positions are taken as (X, Y) format.  X is along the horizontal axis,
Y is along the vertical axis.  The origin is the upper, left corner of the map.
Positive X-axis offset are to the left, Positive Y-axis is down:

```
 Origin (0,0)
/
O---X AXIS---->
|
|
|
Y

A
X
I
S
|
|
|
V
```

Map struct vector access is: map_data[y_axis coordinate][x_axis coordinate].

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