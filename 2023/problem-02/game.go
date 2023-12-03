package main

import (
	"errors"
	"fmt"
)

type Game struct {
	index      int
	redTotal   int32
	greenTotal int32
	blueTotal  int32
	reveals    [][3]int32
}

// Check if the given reveal is possible given total gems
func (game Game) IsRevealPossible(index int32) (bool, error) {
	if index >= int32(len(game.reveals)) {
		return false, errors.New("reveal index out of bounds")
	}
	if game.reveals[index][0] <= game.redTotal && game.reveals[index][1] <= game.greenTotal && game.reveals[index][2] <= game.blueTotal {
		return true, nil
	}
	return false, nil
}

func (game Game) IsGamePossible() bool {
	for index, reveal := range game.reveals {
		result, err := game.IsRevealPossible(int32(index))
		if err != nil {
			fmt.Printf("Error testing reveal, index: %d, reveal: %v", index, reveal)
		}
		if !result {
			return false
		}
	}
	return true
}

func (game Game) GetIndex() int {
	return game.index
}
