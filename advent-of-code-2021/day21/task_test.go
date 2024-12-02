package day21

import (
	"testing"

	"github.com/sogard-dev/advent-of-code-2021/utils"
	"github.com/stretchr/testify/require"
)

func TestPart1(t *testing.T) {
	require.Equal(t, 739785, part1(testInput))
	require.Equal(t, 903630, part1(utils.GetInput(t, "input.txt")))
}

func TestPart2(t *testing.T) {
	require.Equal(t, int64(444356092776315), part2(testInput))
	require.Equal(t, int64(303121579983974), part2(utils.GetInput(t, "input.txt")))
}

const testInput = `Player 1 starting position: 4
Player 2 starting position: 8`
