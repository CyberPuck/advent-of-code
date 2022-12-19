# Problem 04: Elf Cleaning Detail

Elfs are cleaning their workspaces, there is overlap in area assignments.
We need to find how many elf pairs have overlapping assignments.

## Sample

```BASH
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
```

1. No overlap
2. No overlap
3. One section (7) overlaps
4. Complete overlap
5. Complete overlap
6. Three sections (4, 5, 6) overlaps

In this case 2 pairs fully overlap.

## Part One

Find out how many elf pairs *fully* overlap.

## Part Two

Find out how many elf pairs **partially** overlap.

### Scratch Pad for Logic

#### Leading First Pair

```BASH
10---40---
---20---50
```

> if 1 <= 2 start and 1 end >= 2 start

This logic should work with a fully overlapping leading first pair:
```BASH
10-------50
--12---45--
```

#### Trailing First Pair

--20---50--
10---30----

> if 1 >= 2 start and 1 start <= 2 end

This logic should work with a fully overlapping leading first pair:
```BASH
---12-----45---
10-----------50
```