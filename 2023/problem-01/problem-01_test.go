package main

import (
	"testing"
)

func TestLineParser(t *testing.T) {
	test1 := "T1ist3st"
	var test1Sum = 13
	test2 := "6Compl1cat3d4"
	var test2Sum = 64
	test3 := "te1t"
	test3Sum := 11
	sum, err := CalculateSumInLine(test1)
	if test1Sum != sum || err != nil {
		t.Fatalf(`Test 1 failed: %d expected: %d`, sum, test1Sum)
	}

	sum, err = CalculateSumInLine(test2)
	if test2Sum != sum || err != nil {
		t.Fatalf(`Test 2 failed: %d expected: %d`, sum, test2Sum)
	}

	sum, err = CalculateSumInLine(test3)
	if test3Sum != sum || err != nil {
		t.Fatalf(`Test 3 failed: %d expected: %d`, sum, test2Sum)
	}
}

func TestLine2Parser(t *testing.T) {
	test1 := "two1nine"
	var test1Sum = 19
	test2 := "4nineeightseven2"
	var test2Sum = 42
	test3 := "7pqrstsixteen"
	test3Sum := 76
	sum, err := CalculateSumInLine(test1)
	if test1Sum != sum || err != nil {
		t.Fatalf(`Test 1 failed: %d expected: %d`, sum, test1Sum)
	}

	sum, err = CalculateSumInLine(test2)
	if test2Sum != sum || err != nil {
		t.Fatalf(`Test 2 failed: %d expected: %d`, sum, test2Sum)
	}

	sum, err = CalculateSumInLine(test3)
	if test3Sum != sum || err != nil {
		t.Fatalf(`Test 3 failed: %d expected: %d`, sum, test2Sum)
	}
}
