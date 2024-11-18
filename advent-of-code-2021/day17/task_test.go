package day17

import (
	"testing"

	"github.com/sogard-dev/advent-of-code-2021/utils"
	"github.com/stretchr/testify/require"
)

func TestPart1(t *testing.T) {
	require.Equal(t, 45, part1(`target area: x=20..30, y=-10..-5`))
	require.Equal(t, 8911, part1(utils.GetInput(t, "input.txt")))
}

func TestPart2(t *testing.T) {
	require.Equal(t, 112, part2(`target area: x=20..30, y=-10..-5`))
	require.Equal(t, 4748, part2(utils.GetInput(t, "input.txt")))
}
