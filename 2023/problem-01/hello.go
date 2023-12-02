package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"

	"regexp"
)

func main() {
	var filename = "part1.txt"
	fmt.Printf("Reading file: %s\n", filename)

	file, err := os.Open(filename)
	if err != nil {
		fmt.Println("Failed to read file")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	sum := 0
	for scanner.Scan() {
		partialSum, err := CalculateSumInLine(scanner.Text())
		if err != nil {
			fmt.Printf("Failed to parse line%s\n", err)
		} else {
			sum += partialSum
		}
	}

	fmt.Printf("Sum is: %d\n", sum)
}

// Calculate the first and last number sum in a string
// If there are < 2 digits return an error
func CalculateSumInLine(parseLine string) (int, error) {
	fmt.Printf("Entering function :)\n")
	re := regexp.MustCompile(`[[:digit:]]`)
	// Note: `-1` means match all instances in string
	matched := re.FindAll([]byte(parseLine), -1)
	totalString := ""
	if len(matched) < 1 {
		return 0, errors.New("failed to find any digits")
	} else if len(matched) == 1 {
		totalString = string(matched[0]) + string(matched[0])
	} else {
		totalString = string(matched[0]) + string(matched[len(matched)-1])
	}
	fmt.Printf("sum: %s\n", totalString)
	sum, err := strconv.Atoi(totalString)
	if err != nil {
		return 0, errors.New("failed to parse start sum")
	}
	return sum, nil
}
