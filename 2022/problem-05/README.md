# Problem 05: Finding Stack Order

The Elfs are in the process of unloading supplies from the ship.
Crates holding the supplies need to be moved by a crane operator but the elds
do not have the final order listed out.  They want to be ready to start
unpacking supplies but can't ask the crane operator what the order will be.

Given the crane operation procedure (COP), find out the final crate order.

## Sample Input

```BASH
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

1. Crate `D` is moved to the second stack
2. Crates `D`, `N`, `Z` are moved to the third stack; in order from top `Z`, `N`, `D`, `P`
3. Crates `C`, `M` are moved to the first stack; in order from top `M`, `C`
4. Crate `M` is moved to the second stack

### Final Configuration

```BASH
        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3 
```

Final top config is `CMZ`.

## Part One

Once the procedure has been run through give the *top* crate order.

In the above sample it is `CMZ`

## Part Two

CoPs move all crates of a given stack at the same time and in the same order.

### Sample Results

This means that the end result from `sample1.txt` now reports:

> MCD