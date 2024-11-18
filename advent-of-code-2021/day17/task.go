package day17

import (
	"github.com/sogard-dev/advent-of-code-2021/utils"
)

func part1(input string) int {
	nums := utils.GetAllNumbers(input)
	minX, maxX, minY, maxY := nums[0], nums[1], nums[2], nums[3]

	possibleX := calcPossibleX(minX, maxX)
	possibleY := calcPossibleY(minY, maxY)

	highest := 0
	for _, x := range possibleX {
		for _, y := range possibleY {
			h, ok := simulate(x, y, minX, maxX, minY, maxY)
			if ok {
				highest = max(h, highest)
			}
		}
	}

	return highest
}

func part2(input string) int {
	nums := utils.GetAllNumbers(input)
	minX, maxX, minY, maxY := nums[0], nums[1], nums[2], nums[3]

	possibleX := calcPossibleX(minX, maxX)
	possibleY := calcPossibleY(minY, maxY)

	count := 0
	for _, x := range possibleX {
		for _, y := range possibleY {
			_, ok := simulate(x, y, minX, maxX, minY, maxY)
			if ok {
				count++
			}
		}
	}

	return count
}

func simulate(accX, accY, minX, maxX, minY, maxY int) (int, bool) {
	posX, posY := 0, 0
	maxHeight := 0
	for {
		posX += accX
		posY += accY
		maxHeight = max(maxHeight, posY)
		accX = max(0, accX-1)
		accY--
		targetX := posX >= minX && posX <= maxX
		targetY := posY >= minY && posY <= maxY
		if targetX && targetY {
			return maxHeight, true
		}
		if posY < minY || posX > maxX {
			return 0, false
		}
	}
}

func calcPossibleY(minY, maxY int) []int {
	ret := []int{}

	for o := minY; o <= -minY; o++ {
		canHit := false

		pos := 0
		acc := o
		for {
			pos += acc
			acc--
			if pos >= minY && pos <= maxY {
				canHit = true
				break
			} else if pos < minY {
				break
			}
		}

		if canHit {
			ret = append(ret, o)
		}
	}

	return ret
}

func calcPossibleX(minX, maxX int) []int {
	ret := []int{}

	for o := range maxX + 1 {
		canHit := false

		pos := 0
		acc := o
		for {
			pos += acc
			acc--
			if pos >= minX && pos <= maxX {
				canHit = true
				break
			} else if pos > maxX || acc <= 0 {
				break
			}
		}

		if canHit {
			ret = append(ret, o)
		}
	}

	return ret
}
