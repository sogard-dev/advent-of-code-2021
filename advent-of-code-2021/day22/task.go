package day22

import (
	"strings"

	"github.com/sogard-dev/advent-of-code-2021/utils"
)

type pair struct {
	a, b int
}

type cuboid struct {
	x1, y1, z1 int
	x2, y2, z2 int
}

type step struct {
	turnOn bool
	qube   cuboid
}

func (q cuboid) intersect(o cuboid) bool {
	xNoOverlap := q.x1 > o.x2 || o.x1 > q.x2
	yNoOverlap := q.y1 > o.y2 || o.y1 > q.y2
	zNoOverlap := q.z1 > o.z2 || o.z1 > q.z2
	return !(xNoOverlap || yNoOverlap || zNoOverlap)
}

func (q cuboid) contains(o cuboid) bool {
	x := q.x1 <= o.x1 && q.x2 >= o.x2
	y := q.y1 <= o.y1 && q.y2 >= o.y2
	z := q.z1 <= o.z1 && q.z2 >= o.z2
	return x && y && z
}

func (q cuboid) isValid() bool {
	return q.x1 <= q.x2 && q.y1 <= q.y2 && q.z1 <= q.z2
}

func (q cuboid) area() int {
	return (q.x2 - q.x1 + 1) * (q.y2 - q.y1 + 1) * (q.z2 - q.z1 + 1)
}

func area(qubes []cuboid) int {
	sum := 0
	for _, pq := range qubes {
		sum += pq.area()
	}

	return sum
}

func turnOn(active []cuboid, q cuboid) []cuboid {
	active = splitIfNeeded(active, q)
	newActive := []cuboid{q}
	for _, pq := range active {
		if !q.contains(pq) {
			newActive = append(newActive, pq)
		}
	}

	return newActive
}

func turnOff(active []cuboid, q cuboid) []cuboid {
	active = splitIfNeeded(active, q)
	newActive := []cuboid{}
	for _, pq := range active {
		if !q.contains(pq) {
			newActive = append(newActive, pq)
		}
	}

	return newActive
}

func splitIfIntersect(k, splitter cuboid) []cuboid {
	if splitter.intersect(k) && !splitter.contains(k) && k.area() > 1 {
		intersection := cuboid{
			x1: max(k.x1, splitter.x1),
			x2: min(k.x2, splitter.x2),
			y1: max(k.y1, splitter.y1),
			y2: min(k.y2, splitter.y2),
			z1: max(k.z1, splitter.z1),
			z2: min(k.z2, splitter.z2),
		}

		options := []cuboid{}
		for _, x := range []pair{
			{a: k.x1, b: intersection.x1 - 1},
			{a: intersection.x1, b: intersection.x2},
			{a: intersection.x2 + 1, b: k.x2},
		} {
			for _, y := range []pair{
				{a: k.y1, b: intersection.y1 - 1},
				{a: intersection.y1, b: intersection.y2},
				{a: intersection.y2 + 1, b: k.y2},
			} {
				for _, z := range []pair{
					{a: k.z1, b: intersection.z1 - 1},
					{a: intersection.z1, b: intersection.z2},
					{a: intersection.z2 + 1, b: k.z2},
				} {
					options = append(options, cuboid{
						x1: x.a, x2: x.b,
						y1: y.a, y2: y.b,
						z1: z.a, z2: z.b,
					})
				}
			}
		}

		ret := []cuboid{}
		for _, q := range options {
			if q.isValid() {
				ret = append(ret, q)
			}
		}
		return ret
	} else {
		return []cuboid{k}
	}
}

func splitIfNeeded(qubes []cuboid, splitter cuboid) []cuboid {
	newQubes := []cuboid{}
	for _, q := range qubes {
		newQubes = append(newQubes, splitIfIntersect(q, splitter)...)
	}

	return newQubes
}

func solve(input string, maxsize int) int {
	steps := []step{}
	for _, line := range strings.Split(input, "\n") {
		nums := utils.GetAllNumbers(line)
		valid := true
		for _, n := range nums {
			if n < -maxsize || n > maxsize {
				valid = false
			}
		}
		if valid {
			q := cuboid{
				x1: nums[0],
				x2: nums[1],
				y1: nums[2],
				y2: nums[3],
				z1: nums[4],
				z2: nums[5],
			}
			steps = append(steps, step{
				turnOn: line[0:2] == "on",
				qube:   q,
			})
		}
	}

	active := []cuboid{}
	for _, step := range steps {
		if step.turnOn {
			active = turnOn(active, step.qube)
		} else {
			active = turnOff(active, step.qube)
		}
	}

	return area(active)
}

func part1(input string) int {
	return solve(input, 50)
}

func part2(input string) int {
	return solve(input, 1000000)
}
