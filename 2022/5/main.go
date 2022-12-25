package main

import (
	"fmt"
	"strconv"
	"strings"
)

func mustAtoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return i
}

func process_input(input string) {
	components := strings.Split(input, "\n\n")
	crate := strings.Split(components[0], "\n")

	stacks := [][]string{}
	for _, line := range crate[1:] {
		if string(line[1]) == "1" {
			break
		}
		j := 0
		for i := 1; i < len(line); i += 4 {
			if string(line[i]) == " " {
				j++
				continue
			}
			for len(stacks) <= j {
				stacks = append(stacks, []string{})
			}
			stacks[j] = append([]string{string(line[i])}, stacks[j]...)
			j++
		}
	}

	for i, stack := range stacks {
		fmt.Printf("[%d]  ", i)
		for _, stackItems := range stack {
			fmt.Printf("[%s] ", string(stackItems))
		}
		fmt.Printf("\n")
	}

	movements := strings.Split(components[1], "\n")
	for _, line := range movements {
		instructions := strings.Split(line, " ")
		moveCount := mustAtoi(string(instructions[1]))
		fromStack := mustAtoi(string(instructions[3])) - 1
		toStack := mustAtoi(string(instructions[5])) - 1
		for i := 0; i < moveCount; i++ {
			fromLen := len(stacks[fromStack]) - 1
			stacks[toStack] = append(stacks[toStack], stacks[fromStack][fromLen])
			stacks[fromStack] = stacks[fromStack][:fromLen]
		}
	}

	for i, stack := range stacks {
		fmt.Printf("[%d]  ", i)
		for _, stackItems := range stack {
			fmt.Printf("[%s] ", string(stackItems))
		}
		fmt.Printf("\n")
	}

	for _, stack := range stacks {
		fmt.Printf("%s", stack[len(stack)-1])
	}
	fmt.Println("")
	return
}

func process_input_9001(input string) {
	components := strings.Split(input, "\n\n")
	crate := strings.Split(components[0], "\n")

	stacks := [][]string{}
	for _, line := range crate[1:] {
		if string(line[1]) == "1" {
			break
		}
		j := 0
		for i := 1; i < len(line); i += 4 {
			if string(line[i]) == " " {
				j++
				continue
			}
			for len(stacks) <= j {
				stacks = append(stacks, []string{})
			}
			stacks[j] = append([]string{string(line[i])}, stacks[j]...)
			j++
		}
	}

	for i, stack := range stacks {
		fmt.Printf("[%d]  ", i)
		for _, stackItems := range stack {
			fmt.Printf("[%s] ", string(stackItems))
		}
		fmt.Printf("\n")
	}

	movements := strings.Split(components[1], "\n")
	for _, line := range movements {
		instructions := strings.Split(line, " ")
		moveCount := mustAtoi(string(instructions[1]))
		fromStack := mustAtoi(string(instructions[3])) - 1
		toStack := mustAtoi(string(instructions[5])) - 1

		fromLen := len(stacks[fromStack]) - moveCount
		stacks[toStack] = append(stacks[toStack], stacks[fromStack][fromLen:]...)
		stacks[fromStack] = stacks[fromStack][:fromLen]
	}

	for _, stack := range stacks {
		fmt.Printf("%s", stack[len(stack)-1])
	}
	fmt.Println("")
	return
}

func main() {
	// part 1
	process_input(testMovements)
	process_input(movements)
	// part 2
	process_input_9001(testMovements)
	process_input_9001(movements)
}

var testMovements = `
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`

var movements = `
    [B]             [B] [S]
    [M]             [P] [L] [B] [J]
    [D]     [R]     [V] [D] [Q] [D]
    [T] [R] [Z]     [H] [H] [G] [C]
    [P] [W] [J] [B] [J] [F] [J] [S]
[N] [S] [Z] [V] [M] [N] [Z] [F] [M]
[W] [Z] [H] [D] [H] [G] [Q] [S] [W]
[B] [L] [Q] [W] [S] [L] [J] [W] [Z]
 1   2   3   4   5   6   7   8   9

move 3 from 5 to 2
move 5 from 3 to 1
move 4 from 4 to 9
move 6 from 1 to 4
move 6 from 8 to 7
move 5 from 2 to 7
move 1 from 5 to 4
move 11 from 9 to 7
move 1 from 1 to 9
move 6 from 4 to 6
move 12 from 6 to 7
move 1 from 9 to 2
move 2 from 4 to 6
move 1 from 8 to 9
move 1 from 9 to 4
move 1 from 6 to 1
move 2 from 7 to 5
move 2 from 6 to 7
move 2 from 1 to 6
move 2 from 4 to 7
move 1 from 5 to 4
move 1 from 5 to 6
move 1 from 6 to 1
move 1 from 1 to 3
move 1 from 4 to 1
move 1 from 1 to 4
move 1 from 4 to 5
move 1 from 3 to 9
move 1 from 5 to 1
move 4 from 2 to 1
move 20 from 7 to 8
move 24 from 7 to 3
move 3 from 6 to 4
move 1 from 1 to 9
move 1 from 9 to 3
move 2 from 1 to 2
move 2 from 4 to 1
move 2 from 2 to 1
move 14 from 3 to 6
move 6 from 1 to 6
move 10 from 3 to 2
move 1 from 2 to 3
move 6 from 6 to 5
move 2 from 3 to 4
move 13 from 8 to 4
move 1 from 9 to 7
move 1 from 6 to 3
move 10 from 4 to 2
move 1 from 3 to 6
move 2 from 8 to 7
move 1 from 7 to 2
move 11 from 6 to 8
move 2 from 6 to 1
move 2 from 1 to 3
move 1 from 8 to 6
move 1 from 3 to 9
move 3 from 8 to 2
move 1 from 3 to 6
move 2 from 6 to 4
move 1 from 6 to 5
move 11 from 2 to 9
move 2 from 4 to 6
move 1 from 6 to 1
move 1 from 1 to 5
move 11 from 2 to 7
move 12 from 7 to 5
move 1 from 6 to 2
move 10 from 8 to 7
move 6 from 5 to 3
move 4 from 5 to 4
move 11 from 9 to 7
move 7 from 4 to 9
move 4 from 9 to 6
move 12 from 7 to 3
move 1 from 8 to 9
move 1 from 5 to 1
move 1 from 1 to 2
move 1 from 6 to 9
move 3 from 4 to 1
move 1 from 9 to 7
move 8 from 7 to 2
move 3 from 6 to 1
move 8 from 2 to 3
move 1 from 7 to 4
move 2 from 7 to 2
move 1 from 5 to 2
move 8 from 5 to 1
move 3 from 9 to 6
move 1 from 6 to 2
move 1 from 4 to 5
move 1 from 5 to 4
move 2 from 9 to 3
move 1 from 8 to 6
move 1 from 4 to 5
move 1 from 5 to 1
move 1 from 6 to 8
move 1 from 8 to 1
move 7 from 1 to 5
move 11 from 3 to 7
move 1 from 1 to 9
move 4 from 2 to 1
move 5 from 1 to 3
move 1 from 5 to 9
move 1 from 6 to 3
move 6 from 2 to 1
move 5 from 7 to 3
move 1 from 6 to 8
move 1 from 8 to 4
move 6 from 7 to 9
move 4 from 9 to 8
move 2 from 8 to 9
move 2 from 5 to 8
move 13 from 3 to 7
move 1 from 3 to 8
move 2 from 1 to 9
move 3 from 1 to 5
move 1 from 4 to 1
move 6 from 5 to 9
move 8 from 9 to 8
move 2 from 7 to 3
move 1 from 9 to 7
move 1 from 5 to 2
move 5 from 9 to 8
move 1 from 8 to 7
move 1 from 2 to 9
move 7 from 1 to 2
move 4 from 7 to 5
move 6 from 2 to 3
move 1 from 2 to 1
move 10 from 8 to 9
move 3 from 8 to 9
move 4 from 5 to 1
move 2 from 8 to 6
move 9 from 9 to 8
move 1 from 9 to 6
move 8 from 8 to 4
move 12 from 3 to 5
move 1 from 4 to 2
move 3 from 8 to 1
move 3 from 9 to 7
move 1 from 3 to 2
move 1 from 6 to 9
move 8 from 3 to 8
move 6 from 4 to 5
move 1 from 7 to 6
move 1 from 8 to 1
move 6 from 8 to 7
move 1 from 3 to 6
move 7 from 1 to 5
move 1 from 4 to 9
move 4 from 6 to 5
move 13 from 7 to 5
move 1 from 8 to 2
move 2 from 9 to 3
move 4 from 7 to 2
move 1 from 3 to 8
move 1 from 3 to 4
move 4 from 1 to 2
move 1 from 5 to 7
move 23 from 5 to 6
move 1 from 8 to 6
move 1 from 9 to 4
move 5 from 2 to 6
move 1 from 4 to 9
move 1 from 9 to 3
move 1 from 7 to 8
move 1 from 4 to 3
move 1 from 3 to 7
move 1 from 7 to 5
move 1 from 8 to 7
move 12 from 6 to 1
move 1 from 2 to 5
move 1 from 3 to 1
move 20 from 5 to 2
move 14 from 2 to 4
move 11 from 2 to 6
move 1 from 7 to 8
move 13 from 1 to 8
move 9 from 8 to 4
move 3 from 8 to 6
move 10 from 6 to 8
move 6 from 6 to 4
move 4 from 8 to 5
move 26 from 4 to 2
move 2 from 5 to 2
move 5 from 8 to 1
move 1 from 8 to 3
move 2 from 1 to 3
move 2 from 3 to 7
move 27 from 2 to 7
move 2 from 8 to 1
move 1 from 3 to 7
move 6 from 6 to 2
move 4 from 6 to 1
move 4 from 6 to 4
move 2 from 5 to 4
move 4 from 2 to 1
move 3 from 1 to 8
move 1 from 2 to 8
move 8 from 4 to 3
move 1 from 2 to 8
move 5 from 8 to 6
move 1 from 4 to 2
move 1 from 2 to 1
move 6 from 3 to 1
move 13 from 7 to 1
move 1 from 2 to 8
move 1 from 8 to 2
move 1 from 6 to 2
move 1 from 2 to 8
move 1 from 8 to 2
move 14 from 7 to 1
move 5 from 6 to 3
move 2 from 3 to 1
move 3 from 3 to 2
move 3 from 7 to 4
move 1 from 4 to 9
move 1 from 9 to 7
move 2 from 3 to 6
move 5 from 2 to 7
move 1 from 7 to 6
move 5 from 7 to 6
move 2 from 6 to 7
move 1 from 6 to 8
move 1 from 4 to 7
move 4 from 6 to 9
move 35 from 1 to 8
move 3 from 7 to 2
move 1 from 2 to 5
move 24 from 8 to 3
move 1 from 5 to 8
move 13 from 3 to 6
move 2 from 2 to 6
move 6 from 6 to 4
move 11 from 1 to 6
move 12 from 6 to 1
move 1 from 8 to 1
move 2 from 1 to 3
move 5 from 4 to 1
move 1 from 6 to 4
move 1 from 8 to 3
move 13 from 3 to 9
move 3 from 8 to 2
move 3 from 2 to 7
move 1 from 3 to 6
move 3 from 7 to 8
move 14 from 1 to 3
move 1 from 1 to 9
move 6 from 3 to 8
move 17 from 8 to 6
move 1 from 3 to 7
move 1 from 7 to 8
move 26 from 6 to 7
move 1 from 1 to 9
move 3 from 4 to 1
move 2 from 3 to 8
move 1 from 8 to 4
move 14 from 9 to 7
move 12 from 7 to 3
move 2 from 1 to 4
move 2 from 7 to 8
move 2 from 8 to 3
move 4 from 9 to 8
move 1 from 4 to 7
move 1 from 1 to 3
move 2 from 4 to 2
move 24 from 7 to 6
move 1 from 8 to 1
move 1 from 7 to 2
move 1 from 7 to 9
move 3 from 2 to 9
move 1 from 1 to 6
move 5 from 8 to 2
move 5 from 3 to 4
move 1 from 2 to 5
move 3 from 9 to 8
move 2 from 4 to 9
move 16 from 6 to 3
move 14 from 3 to 8
move 1 from 7 to 9
move 8 from 6 to 9
move 4 from 8 to 5
move 8 from 8 to 3
move 1 from 5 to 8
move 1 from 2 to 4
move 4 from 8 to 7
move 1 from 5 to 6
move 12 from 9 to 5
move 15 from 5 to 8
move 1 from 6 to 1
move 2 from 2 to 6
move 3 from 4 to 2
move 4 from 2 to 7
move 8 from 7 to 3
move 1 from 1 to 4
move 3 from 6 to 9
move 16 from 8 to 3
move 3 from 9 to 4
move 1 from 8 to 9
move 2 from 9 to 4
move 24 from 3 to 8
move 19 from 8 to 7
move 2 from 8 to 7
move 7 from 4 to 5
move 13 from 7 to 5
move 4 from 7 to 8
move 7 from 8 to 1
move 3 from 5 to 3
move 3 from 7 to 2
move 1 from 1 to 4
move 1 from 7 to 2
move 3 from 2 to 4
move 8 from 3 to 1
move 11 from 1 to 3
move 12 from 3 to 4
move 1 from 2 to 5
move 18 from 3 to 8
move 3 from 1 to 9
move 1 from 3 to 5
move 15 from 5 to 4
move 4 from 5 to 1
move 23 from 4 to 6
move 3 from 1 to 6
move 13 from 8 to 3
move 25 from 6 to 2
move 1 from 9 to 5
move 5 from 3 to 8
move 17 from 2 to 8
move 4 from 4 to 1
move 1 from 9 to 7
move 5 from 2 to 6
move 2 from 2 to 4
move 1 from 9 to 4
move 6 from 3 to 9
move 16 from 8 to 3
move 2 from 1 to 8
move 1 from 7 to 4
move 5 from 4 to 7
move 1 from 5 to 3
move 2 from 7 to 1
move 9 from 8 to 4
move 3 from 7 to 2
move 2 from 8 to 3
move 10 from 4 to 1
move 1 from 2 to 3
move 5 from 3 to 7
move 2 from 8 to 9
move 2 from 9 to 8
move 1 from 2 to 1
move 3 from 9 to 6
move 2 from 2 to 8
move 4 from 7 to 3
move 4 from 8 to 6
move 1 from 7 to 1
move 1 from 4 to 8
move 4 from 3 to 4
move 4 from 4 to 2
move 6 from 1 to 2
move 1 from 4 to 3
move 5 from 3 to 8
move 6 from 3 to 8
move 2 from 2 to 8
move 3 from 2 to 9
move 8 from 1 to 6
move 3 from 2 to 7
move 2 from 7 to 2
move 13 from 6 to 5
move 7 from 5 to 9
move 3 from 2 to 7
move 1 from 2 to 9
move 2 from 5 to 2
move 3 from 8 to 5
move 5 from 3 to 4
move 2 from 2 to 1
move 9 from 8 to 7
move 1 from 1 to 8
move 6 from 5 to 2
move 4 from 2 to 8
move 4 from 7 to 1
move 1 from 2 to 6
move 5 from 1 to 6
move 1 from 8 to 2
move 1 from 2 to 9
move 13 from 6 to 5
move 2 from 7 to 2
move 1 from 8 to 7
move 4 from 4 to 7
move 1 from 4 to 1
move 4 from 8 to 4
move 6 from 5 to 9
move 2 from 1 to 4
move 1 from 8 to 6
move 11 from 9 to 5
move 1 from 7 to 8
move 1 from 8 to 1
move 1 from 1 to 3
move 6 from 4 to 8
move 1 from 8 to 4
move 1 from 1 to 6
move 6 from 9 to 7
move 1 from 4 to 5
move 3 from 2 to 1
move 1 from 8 to 2
move 1 from 3 to 2
move 20 from 5 to 6
move 3 from 1 to 6
move 2 from 2 to 9
move 3 from 8 to 3
move 5 from 3 to 8
move 1 from 1 to 6
move 2 from 8 to 9
move 7 from 9 to 5
move 3 from 5 to 4
move 3 from 8 to 3
move 9 from 7 to 9
move 1 from 8 to 5
move 7 from 7 to 9
move 2 from 5 to 2
move 9 from 9 to 2
move 1 from 7 to 3
move 2 from 9 to 1
move 2 from 5 to 9
move 2 from 1 to 4
move 2 from 3 to 7
move 18 from 6 to 7
move 7 from 9 to 1
move 7 from 6 to 8
move 4 from 4 to 9
move 4 from 8 to 3
move 2 from 8 to 2
move 1 from 8 to 5
move 1 from 4 to 7
move 1 from 5 to 1
move 2 from 9 to 3
move 12 from 2 to 5
move 6 from 5 to 6
move 5 from 7 to 2
move 3 from 6 to 4
move 1 from 4 to 7
move 1 from 4 to 1
move 2 from 5 to 8
move 1 from 8 to 2
move 2 from 9 to 7
move 8 from 1 to 8
move 11 from 7 to 1
move 5 from 8 to 2
move 7 from 7 to 5
move 1 from 9 to 4
move 1 from 7 to 5
move 7 from 5 to 7
move 2 from 6 to 1
move 1 from 8 to 2
move 12 from 1 to 7
move 2 from 1 to 2
move 3 from 8 to 5
move 3 from 5 to 2
move 8 from 7 to 3
move 1 from 3 to 1
move 3 from 6 to 4
move 4 from 5 to 6
move 14 from 2 to 9
move 3 from 6 to 9
move 3 from 4 to 2
move 1 from 1 to 7
move 1 from 7 to 1
move 3 from 3 to 5
move 8 from 7 to 4
move 1 from 5 to 9
move 3 from 2 to 4
move 1 from 3 to 4
move 4 from 2 to 6
move 2 from 6 to 7
move 3 from 5 to 4
move 16 from 4 to 1
move 7 from 9 to 8
move 1 from 5 to 1
move 3 from 7 to 9
move 3 from 9 to 4
move 7 from 1 to 7
move 6 from 7 to 1
move 5 from 3 to 1
move 11 from 9 to 2
move 3 from 4 to 6
move 9 from 2 to 8
move 6 from 3 to 5
move 2 from 8 to 6
move 5 from 5 to 3
move 2 from 7 to 1
move 3 from 3 to 9
move 1 from 2 to 4
move 1 from 5 to 1
move 13 from 1 to 2
move 5 from 8 to 6
move 2 from 3 to 9
move 2 from 4 to 7
move 5 from 6 to 9
move 7 from 9 to 1
move 3 from 7 to 2
move 6 from 8 to 6
move 5 from 6 to 2
move 2 from 8 to 3
move 2 from 9 to 4
move 6 from 2 to 5
move 1 from 3 to 7`
