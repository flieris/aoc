package main

import (
	"bufio"
	"log"
	"math"
	"os"
	"regexp"
	"strconv"
)

func getInputs() ([]int, error) {
	fd, err := os.Open("./2025/day1/input.txt")
	if err != nil {
		return nil, err
	}
	defer fd.Close()
	var slice []int
	re := regexp.MustCompile(`([a-zA-Z]+)(\d+)`)
	scanner := bufio.NewScanner(fd)
	for scanner.Scan() {
		line := scanner.Text()
		match := re.FindStringSubmatch(line)
		num, err := strconv.Atoi(match[2])
		if err != nil {
			log.Println(err)
			continue
		}
		val := num
		if match[1] == "L" {
			val = -(num)
		}
		slice = append(slice, val)
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return slice, nil
}

func solution1(inputs []int) (int, error) {
	var result int
	result = 0
	pos := 50
	for i := 0; i < len(inputs); i++ {
		val := pos + inputs[i]
		pos = intMod(val, 100)
		if pos == 0 {
			result++
		}
	}

	return result, nil
}

func solution2(inputs []int) int {
	var result int
	result = 0
	pos := 50
	for i := 0; i < len(inputs); i++ {
		val := pos + inputs[i]
		// handle left movement
		if inputs[i] < 0 && pos != 0 && intMod(pos, 100) < intMod(Abs(inputs[i]), 100) {
			result++
		} else { // handle right movement
			if inputs[i] > 0 && pos != 0 && (intMod(pos, 100)+intMod(inputs[i], 100)) > 100 {
				result++
			}
		}
		rotations := Abs(int(math.Floor(float64(Abs(inputs[i])) / 100.0)))
		result += rotations
		pos = intMod(val, 100)
		if pos == 0 {
			result++
		}
	}

	return result
}

func intMod(a, b int) int {
	return (a%b + b) % b
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	inputs, err := getInputs()
	if err != nil {
		log.Fatal(err)
	}
	sol1, _ := solution1(inputs)
	log.Printf("Solution for part 1: %v", sol1)
	sol2 := solution2(inputs)
	log.Printf("Solution for part 2: %v", sol2)
}
