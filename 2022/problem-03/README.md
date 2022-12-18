# Part 3: Reorganizing The Bags

Elfs are organizing the bags, looks like some items were added to the wrong
compartment in each bag.  Every bag has two compartments.

## Bag Rules

- Each compartment holds the same amount of stuff
- All items added to a compartment must be of the same type
- Item type is IDes as an upper or lower case character (`A` vs `a`)

## Item Priority

Items are prioritized by:
1. Items ascii `a` through `z` are 1 - 26
2. Items ascii `A` through `Z` are 27 - 52

## Sample

```BASH
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
```

# Part 2, find total group priority

The elfs are split into groups of three, this means every three lines in the
file is an Elf Group (EG).  These EGs have a common item they carry, called
a badge.  We need to find out what each EGs badge is and then sum them up.

For this problem both total priority and total badge priority will be returned
at the same time.