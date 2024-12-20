package day20

import (
	"testing"

	"github.com/sogard-dev/advent-of-code-2021/utils"
	"github.com/stretchr/testify/require"
)

func TestPart1(t *testing.T) {
	require.Equal(t, 35, part1(testInput))
	require.Equal(t, 5437, part1(utils.GetInput(t, "input.txt")))
}

func TestPart2(t *testing.T) {
	require.Equal(t, 3351, part2(testInput))
	require.Equal(t, 19340, part2(utils.GetInput(t, "input.txt")))
}

const testInput = `..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###`
