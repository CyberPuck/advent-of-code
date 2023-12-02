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
	//var filename = "part1.txt"
	var filename = "test2.txt"
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
	re := regexp.MustCompile(`[[:digit:]]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)`)
	// Note: `-1` means match all instances in string
	matched := re.FindAll([]byte(parseLine), -1)
	for index, m := range matched {
		fmt.Printf("%d, with %s\n", index, m)
	}
	totalString := ""
	if len(matched) < 1 {
		return 0, errors.New("failed to find any digits")
	} else if len(matched) == 1 {
		totalString = VerifyIntString(string(matched[0])) + VerifyIntString(string(matched[0]))
	} else {
		totalString = VerifyIntString(string(matched[0])) + VerifyIntString(string(matched[len(matched)-1]))
	}
	fmt.Printf("sum: %s\n", totalString)
	sum, err := strconv.Atoi(totalString)
	if err != nil {
		return 0, errors.New("failed to parse start sum")
	}
	return sum, nil
}

// Use the Atoi function if it is a Unicode integer
// Use the custom switch statement if it is a string
// If it can't be parsed return "0"
func VerifyIntString(line string) string {
	_, err := strconv.Atoi(line)
	if err != nil {
		switch line {
		case "one":
			return "1"
		case "two":
			return "2"
		case "three":
			return "3"
		case "four":
			return "4"
		case "five":
			return "5"
		case "six":
			return "6"
		case "seven":
			return "7"
		case "eight":
			return "8"
		case "nine":
			return "9"
		default:
			fmt.Printf("String unknown: %s\n", line)
			return "0"
		}
	}
	return line
}
