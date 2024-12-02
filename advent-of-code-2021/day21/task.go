package day21

import (
	"github.com/sogard-dev/advent-of-code-2021/utils"
)

func part1(input string) int {
	nums := utils.GetAllNumbers(input)
	p := [2]int{nums[1], nums[3]}
	s := [2]int{0, 0}

	dice := 1

	for {
		for i, l := range p {
			d := 3 * (dice + 1)
			dice += 3
			p[i] = ((l + d - 1) % 10) + 1
			s[i] += p[i]
			if s[i] >= 1000 {
				return s[(i+1)%2] * (dice - 1)
			}
		}
	}
}

type game struct {
	p1, p2, s1, s2 int
	p1Turn         bool
}

type score struct {
	s1, s2 int64
}

func play(orig game, cache map[game]score) score {
	if prev, exists := cache[orig]; exists {
		return prev
	} else {
		thisScores := score{}

		for d1 := 1; d1 <= 3; d1++ {
			for d2 := 1; d2 <= 3; d2++ {
				for d3 := 1; d3 <= 3; d3++ {
					p := d1 + d2 + d3
					ng := game(orig)
					if ng.p1Turn {
						ng.p1 = ((ng.p1 + p - 1) % 10) + 1
						ng.s1 += ng.p1

					} else {
						ng.p2 = ((ng.p2 + p - 1) % 10) + 1
						ng.s2 += ng.p2
					}

					if ng.s1 >= 21 {
						thisScores.s1 += 1
					} else if ng.s2 >= 21 {
						thisScores.s2 += 1
					} else {
						ng.p1Turn = !ng.p1Turn
						sg := play(ng, cache)
						thisScores.s1 += sg.s1
						thisScores.s2 += sg.s2
					}
				}
			}
		}

		cache[orig] = thisScores
		return thisScores
	}
}

func part2(input string) int64 {
	nums := utils.GetAllNumbers(input)
	c := map[game]score{}
	s := play(game{
		p1Turn: true,
		p1:     nums[1],
		p2:     nums[3],
	}, c)

	if s.s1 > s.s2 {
		return s.s1
	} else {
		return s.s2
	}
}
