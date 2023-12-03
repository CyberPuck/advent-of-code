package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	var filename = "part1.txt"
	//var filename = "test2.txt"
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
	// Part one regex
	// re := regexp.MustCompile(`[[:digit:]]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)`)
	var expressionList = [10]string{"[[:digit:]]", "(one)", "(two)", "(three)", "(four)", "(five)", "(six)", "(seven)", "(eight)", "(nine)"}
	// Part two, need to run a regex for all text digits independently
	// Then need to compare which one starts first and which one ends last
	var startIndex = len(parseLine)
	var startNumber = ""
	var endIndex = 0
	var endNumber = ""

	// Try all regex indpendently to preserve individual words
	for _, expression := range expressionList {
		re := regexp.MustCompile(expression)
		matches := re.FindAllIndex([]byte(parseLine), -1)
		var isDigit = expression == "[[:digit:]]"
		// Run through all matches checking if they are better
		for _, match := range matches {
			// convert `[[:digit:]]` to number if needed
			if isDigit {
				expression = string(parseLine[match[0]])
			}

			if match[0] < startIndex || (match[0] == startIndex && startNumber == "") {
				startIndex = match[0]
				startNumber = expression
			}
			if match[0] > endIndex || (match[0] == endIndex && endNumber == "") {
				endIndex = match[0]
				endNumber = expression
			}
		}
	}

	// Create the number to return
	totalString := VerifyIntString(startNumber) + VerifyIntString(endNumber)
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
		case "(one)":
			return "1"
		case "(two)":
			return "2"
		case "(three)":
			return "3"
		case "(four)":
			return "4"
		case "(five)":
			return "5"
		case "(six)":
			return "6"
		case "(seven)":
			return "7"
		case "(eight)":
			return "8"
		case "(nine)":
			return "9"
		default:
			fmt.Printf("String unknown: %s\n", line)
			return "0"
		}
	}
	return line
}
