package day16

import (
	"testing"

	"github.com/sogard-dev/advent-of-code-2021/utils"
	"github.com/stretchr/testify/require"
)

func TestPart1(t *testing.T) {
	require.Equal(t, 31, part1(`A0016C880162017C3686B18A3D4780`))
	require.Equal(t, 23, part1(`C0015000016115A2E0802F182340`))
	require.Equal(t, 12, part1(`620080001611562C8802118E34`))
	require.Equal(t, 16, part1(`8A004A801A8002F478`))
	require.Equal(t, 960, part1(utils.GetInput(t, "input.txt")))
}

func TestPart2(t *testing.T) {
	require.Equal(t, 3, part2(`C200B40A82`))
	require.Equal(t, 54, part2(`04005AC33890`))
	require.Equal(t, 7, part2(`880086C3E88112`))
	require.Equal(t, 9, part2(`CE00C43D881120`))
	require.Equal(t, 1, part2(`D8005AC2A8F0`))
	require.Equal(t, 0, part2(`F600BC2D8F`))
	require.Equal(t, 0, part2(`9C005AC2F8F0`))
	require.Equal(t, 1, part2(`9C0141080250320F1802104A08`))
	require.Equal(t, 12301926782560, part2(utils.GetInput(t, "input.txt")))
}
