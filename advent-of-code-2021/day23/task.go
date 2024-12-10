package day23

import (
	"math"
	"strings"
)

type state struct {
	positions map[pos]rune
}

type graph struct {
	nodes       map[pos]node
	connections map[pos]map[pos]bool
}

type node struct {
	canStopHere, isHallway bool
	usedFor                rune
}

type pos struct {
	x, y, d int
}

func part1(input string) int {
	g := graph{nodes: map[pos]node{}, connections: map[pos]map[pos]bool{}}
	animals := map[pos]rune{}
	for y, line := range strings.Split(input, "\n") {
		for x, r := range line {
			if r != '#' && r != ' ' {
				p := pos{x: x, y: y}
				n := node{
					isHallway:   y == 1,
					canStopHere: !(y == 1 && (x == 3 || x == 5 || x == 7 || x == 9)),
				}
				if y >= 2 {
					if x == 3 {
						n.usedFor = 'A'
					} else if x == 5 {
						n.usedFor = 'B'
					} else if x == 7 {
						n.usedFor = 'C'
					} else if x == 9 {
						n.usedFor = 'D'
					}
				}

				g.nodes[p] = n
				g.connections[p] = map[pos]bool{}

				for _, d := range []pos{{x: 0, y: -1}, {x: -1, y: 0}} {
					np := pos{x: x + d.x, y: y + d.y}
					if _, exists := g.nodes[np]; exists {
						g.connections[p][np] = true
						g.connections[np][p] = true
					}
				}
				if r >= 'A' && r <= 'D' {
					animals[p] = r
				}
			}
		}
	}

	return search(g, animals)
}

func bfs(g graph, start pos, isValid func(q node) bool, animals map[pos]rune) []pos {
	open := []pos{start}
	closed := map[pos]bool{start: true}
	ret := []pos{}
	for len(open) > 0 {
		o := open[0]
		p := pos{x: o.x, y: o.y}
		open = open[1:]

		n := g.nodes[p]
		if n.canStopHere {
			if isValid(n) {
				ret = append(ret, o)
			}
		}

		for np := range g.connections[p] {
			if _, isVisited := closed[np]; !isVisited {
				closed[np] = true
				if _, hasAnimal := animals[np]; !hasAnimal {
					open = append(open, pos{x: np.x, y: np.y, d: 1 + o.d})
				}
			}
		}
	}
	return ret
}

func search(g graph, animals map[pos]rune) int {
	lowestEffort := math.MaxInt
	sizeOfCaves := len(animals) / 4
	caveStart := 2

	isInRightSpot := func(p pos, r rune) bool {
		for i := caveStart + sizeOfCaves - 1; i >= p.y; i-- {
			if animals[pos{x: p.x, y: i}] != r {
				return false
			}
		}

		return true
	}

	isFirstFreeSpot := func(p pos, r rune) bool {
		for i := caveStart + sizeOfCaves - 1; i >= caveStart; i-- {
			l := animals[pos{x: p.x, y: i}]
			if l == 0 {
				return p.y == i
			} else if l != r {
				return false
			}
		}
		return false
	}

	var rec func(int) int
	rec = func(effortSoFar int) int {
		if effortSoFar > lowestEffort {
			return effortSoFar
		}

		animalsSafe := 0
		for p, r := range animals {
			c := g.nodes[p]
			if c.usedFor == r && isInRightSpot(p, r) {
				//Sweet, it is home and we are not blocking
				animalsSafe++
			} else {
				var f func(node) bool
				if c.isHallway {
					f = func(q node) bool { return q.usedFor == r }
				} else {
					f = func(q node) bool { return q.isHallway || q.usedFor == r }
				}

				options := bfs(g, p, f, animals)
				for _, option := range options {
					moveTo := pos{x: option.x, y: option.y}
					dest := g.nodes[moveTo]
					if dest.usedFor == r {
						if isFirstFreeSpot(moveTo, r) {
							options = []pos{option}
						}
					}
				}

				for _, option := range options {
					moveTo := pos{x: option.x, y: option.y}
					dest := g.nodes[moveTo]
					if dest.usedFor == r {
						if !isFirstFreeSpot(moveTo, r) {
							continue
						}
					}

					delete(animals, p)
					animals[moveTo] = r
					thisEffort := option.d * rToEnergy(r)
					rec(thisEffort + effortSoFar)

					delete(animals, moveTo)
					animals[p] = r
				}
			}
		}

		if animalsSafe == len(animals) {
			if effortSoFar < lowestEffort {
				lowestEffort = effortSoFar
				println("Lowest effort seen", lowestEffort)
			}
		}

		return effortSoFar
	}

	rec(0)

	return lowestEffort
}

func rToEnergy(r rune) int {
	if r == 'A' {
		return 1
	}
	if r == 'B' {
		return 10
	}
	if r == 'C' {
		return 100
	}
	if r == 'D' {
		return 1000
	}
	panic("Nope")
}
