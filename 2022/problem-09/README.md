# Problem 09 Follow the Rope

Simulate a rope moving to distract self while crossing a bridge.

## Part One Where was the tail?

Need to track all of the unique locations of the tail.  The tail moves
differently from the rope head.

### Rope Movement

- Tail (T) must always be touching the Head (H)
- Touching involves:
    - Overrlapping (head on top of tail)
    - Adjacent (both orthogonal and diagonal)
- If the head is ever two steps orthogonal to the tail, the tail must move in that direction
- If the head and tail are not touching and are not in the same row and column, the tail moves diagonally

### Sample

```BASH
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
```

There are *13* unique locations the tail visits following these instructions.

Here is the sample results:
```text
..##..
...##.
.####.
....#.
s###..
```