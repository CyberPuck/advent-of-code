# Problem 08

There is a forest that was planted by a previous expedition.  Elfs want to
build a treehouse but need to find out what trees are visible from the outside
of the forest.

## Part 1

What number of trees are visible from the outside of the forest (called grid).

### Requirments

- Tree is visible when looking at only one direction: up, down, left, right
- Tree is visible if only shorter trees are between it and the edge
- Tree is invisible if from all directions there is a taller tree in front

### Sample

```BASH
30373
25512
65332
33549
35390
```

In this example there are *21* trees visible from the outside.

#### Invisible Trees

- Middle tree `3`
- Upper-right tree `1`
- Lower-left tree `3`
- Lower-right tree `4`