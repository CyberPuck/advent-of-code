package main

import (
	"fmt"
	"testing"
)

func TestParseBag(t *testing.T) {
	test1Line := "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
	fmt.Printf("Test me: %s\n", test1Line)
	//t.Fatalf("Failed, %s", test1Line)
}

func TestGameFunc(t *testing.T) {
	var reveal1 = [3][3]int32{{4, 0, 3}, {1, 2, 6}, {0, 2, 0}}
	var game1 = Game{
		redTotal:   12,
		greenTotal: 13,
		blueTotal:  14,
		reveals:    reveal1[:],
	}
	if !game1.IsGamePossible() {
		t.Fatalf("Game 1 failed")
	}

	var reveal2 = [3][3]int32{{20, 8, 6}, {4, 13, 5}, {1, 5, 0}}
	var game2 = Game{
		redTotal:   12,
		greenTotal: 13,
		blueTotal:  14,
		reveals:    reveal2[:],
	}
	if game2.IsGamePossible() {
		t.Fatalf("Game 2 did not failed")
	}
}
