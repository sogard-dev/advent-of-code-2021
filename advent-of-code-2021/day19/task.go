package day19

import (
	"strings"

	"github.com/sogard-dev/advent-of-code-2021/utils"
)

type point struct {
	x, y, z int
}

type distance struct {
	a, b point
	dist int
}

type scanner struct {
	beacons   []point
	translate point
}

func findPositions(input string) []scanner {
	scanners := []scanner{}
	for _, block := range strings.Split(input, "\n\n") {
		scanner := scanner{
			beacons: []point{},
		}

		for _, line := range strings.Split(block, "\n") {
			nums := utils.GetAllNumbers(line)
			if len(nums) == 3 {
				scanner.beacons = append(scanner.beacons, point{nums[0], nums[1], nums[2]})
			}
		}

		scanners = append(scanners, scanner)
	}

	distances := map[int][]distance{}
	for i, scanner := range scanners {
		d := []distance{}
		for i := 0; i < len(scanner.beacons); i++ {
			a := scanner.beacons[i]
			for j := i + 1; j < len(scanner.beacons); j++ {
				b := scanner.beacons[j]
				d = append(d, distance{
					a:    a,
					b:    b,
					dist: dist(a, b),
				})
			}
		}

		distances[i] = d
	}

	canFind := map[int][]int{}
	for i := 0; i < len(distances); i++ {
		canFind[i] = []int{}
		for j := 0; j < len(distances); j++ {
			if i != j {
				m := distanceMatches(distances[i], distances[j])
				if m >= 66 {
					canFind[i] = append(canFind[i], j)
				}
			}
		}
	}

	open := []int{0}
	closed := map[int]bool{0: true}

	for len(open) > 0 {
		s := open[0]
		open = open[1:]

		for _, next := range canFind[s] {
			if !closed[next] {
				if setOverlap(&scanners[s], &scanners[next]) {
					closed[next] = true
					open = append(open, next)
				}
			}
		}
	}
	return scanners
}

func setOverlap(origin *scanner, other *scanner) bool {
	rotateX := 0
	rotateY := 0
	rotateZ := 0

	for rotateZ != 4 {
		otherBeacons := []point{}
		for _, p := range other.beacons {
			for range rotateX {
				p = point{
					x: p.x,
					y: -p.z,
					z: p.y,
				}
			}
			for range rotateY {
				p = point{
					x: -p.z,
					y: p.y,
					z: p.x,
				}
			}
			for range rotateZ {
				p = point{
					x: -p.y,
					y: p.x,
					z: p.z,
				}
			}

			otherBeacons = append(otherBeacons, p)
		}

		translateX := map[int]int{}
		translateY := map[int]int{}
		translateZ := map[int]int{}
		for _, p1 := range origin.beacons {
			for _, p2 := range otherBeacons {
				translateX[p1.x-p2.x] += 1
				translateY[p1.y-p2.y] += 1
				translateZ[p1.z-p2.z] += 1
			}
		}

		for k, v := range translateX {
			if v < 12 {
				delete(translateX, k)
			}
		}
		for k, v := range translateY {
			if v < 12 {
				delete(translateY, k)
			}
		}
		for k, v := range translateZ {
			if v < 12 {
				delete(translateZ, k)
			}
		}

		if len(translateX) == 1 && len(translateY) == 1 && len(translateZ) == 1 {
			other.beacons = otherBeacons

			for x := range translateX {
				for y := range translateY {
					for z := range translateZ {
						other.translate = point{
							x: origin.translate.x + x,
							y: origin.translate.y + y,
							z: origin.translate.z + z,
						}
					}
				}
			}

			return true
		} else {
			rotateX += 1
			if rotateX == 4 {
				rotateX = 0
				rotateY += 1
				if rotateY == 4 {
					rotateY = 0
					rotateZ += 1
				}
			}
		}
	}

	return false
}

func distanceMatches(s1, s2 []distance) int {
	matches := 0

	for _, p1 := range s1 {
		for _, p2 := range s2 {
			if p1.dist == p2.dist {
				matches++
			}
		}
	}

	return matches
}

func dist(a, b point) int {
	return abs(a.x-b.x) + abs(a.y-b.y) + abs(a.z-b.z)
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func part1(input string) int {
	scanners := findPositions(input)

	globalBeacons := map[point]bool{}
	for _, scanner := range scanners {
		for _, p := range scanner.beacons {
			tp := point{
				x: p.x + scanner.translate.x,
				y: p.y + scanner.translate.y,
				z: p.z + scanner.translate.z,
			}
			globalBeacons[tp] = true
		}
	}

	return len(globalBeacons)
}

func part2(input string) int {
	scanners := findPositions(input)

	furthest := 0
	for i := 0; i < len(scanners); i++ {
		for j := i + 1; j < len(scanners); j++ {
			furthest = max(furthest, dist(scanners[i].translate, scanners[j].translate))
		}
	}

	return furthest
}
