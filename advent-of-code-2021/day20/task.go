package day20

import (
	"math"
	"strings"
)

type pos struct {
	x, y int
}

type minMax struct {
	minX, maxX, minY, maxY int
}

func getMinMax(img map[pos]bool) minMax {
	minX, maxX, minY, maxY := math.MaxInt, 0, math.MaxInt, 0
	for p := range img {
		minX = min(minX, p.x)
		maxX = max(maxX, p.x)
		minY = min(minY, p.y)
		maxY = max(maxY, p.y)
	}
	return minMax{
		minX: minX,
		maxX: maxX,
		minY: minY,
		maxY: maxY,
	}
}

func iterate(img map[pos]bool, algo *string, run int, size minMax) (map[pos]bool, minMax) {
	newMinMax := minMax{
		minX: size.minX - 1,
		maxX: size.maxX + 1,
		minY: size.minY - 1,
		maxY: size.maxY + 1,
	}

	isEdge := func(p pos) bool {
		if (*algo)[0] == '#' && run%2 == 1 {
			if p.x <= newMinMax.minX || p.x >= newMinMax.maxX || p.y <= newMinMax.minY || p.y >= newMinMax.maxY {
				return true
			}
		}
		return false
	}

	newImage := map[pos]bool{}
	for y := newMinMax.minY; y <= newMinMax.maxY; y++ {
		for x := newMinMax.minX; x <= newMinMax.maxX; x++ {
			p := pos{
				x: x,
				y: y,
			}

			v := 256
			a := 0
			for dy := -1; dy <= 1; dy++ {
				for dx := -1; dx <= 1; dx++ {
					np := pos{x: p.x + dx, y: p.y + dy}
					if img[np] || isEdge(np) {
						a += v
					}

					v /= 2
				}
			}

			if (*algo)[a] == '#' {
				newImage[p] = true
			}
		}
	}

	return newImage, newMinMax
}

func part1(input string) int {
	return solve(input, 2)
}

func part2(input string) int {
	return solve(input, 50)
}

func solve(input string, runs int) int {
	blocks := strings.Split(input, "\n\n")
	algo := blocks[0]

	image := map[pos]bool{}

	for y, line := range strings.Split(blocks[1], "\n") {
		for x, r := range line {
			if r == '#' {
				image[pos{x: x, y: y}] = true
			}
		}
	}

	size := getMinMax(image)
	for run := range runs {
		image, size = iterate(image, &algo, run, size)
	}

	return len(image)
}
