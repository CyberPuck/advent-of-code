package main

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	fmt.Printf("Hello World\n")
}

func ParseGame(line string, red int32, green, int32, blue int32) (*Game, error) {
	var re = regexp.MustCompile("[[:digit:]]+")
	var indexStr = re.FindString(line)
	fmt.Printf("Expected index: %s", indexStr)
	index, err := strconv.Atoi(indexStr)
	if err != nil {
		return nil, errors.New("failed to parse game index")
	}
	var revealsRaw = strings.Split(line, ":")[1]

	reveals := [][3]int32{}

	for _, reveal := range strings.Split(revealsRaw, ";") {
		red := 0
		green := 0
		blue := 0

		parsedReveal := [3]int{}
		for _, colorStr := range strings.Split(reveal, ",") {

			if strings.Contains(colorStr, "red") {
				redNumber := re.FindString(colorStr)
				redNum, err := strconv.Atoi(redNumber)
				if err != nil {
					return nil, errors.New("Failed to parse red")
				}
				red = redNum
			} else if strings.Contains(colorStr, "green") {
				greenNumber := re.FindString(colorStr)
				greenNum, err := strconv.Atoi(greenNumber)
				if err != nil {
					return nil, errors.New("Failed to parse green")
				}
				green = greenNum
			} else if strings.Contains(colorStr, "blue") {
				blueNumber := re.FindString(colorStr)
				blueNum, err := strconv.Atoi(blueNumber)
				if err != nil {
					return nil, errors.New("Failed to parse blue")
				}
				blue = blueNum
			}
			parsedReveal[0] = red
			parsedReveal[1] = green
			parsedReveal[2] = blue
		}
		reveals = append(reveals, parsedReveal)
	}

	return &Game{
		index:      index,
		redTotal:   red,
		greenTotal: green,
		blueTotal:  blue,
		reveals:    reveals,
	}, nil
}
