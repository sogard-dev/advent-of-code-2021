package day18

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/sogard-dev/advent-of-code-2021/utils"
)

type snailfishNumber struct {
	left   *snailfishNumber
	right  *snailfishNumber
	parent *snailfishNumber
	value  int
}

func (s snailfishNumber) magnitude() int {
	if s.value >= 0 {
		return s.value
	} else {
		return 3*s.left.magnitude() + 2*s.right.magnitude()
	}
}
func (s snailfishNumber) toText() string {
	if s.value >= 0 {
		return strconv.Itoa(s.value)
	} else {
		return fmt.Sprintf("[%s,%s]", (*s.left).toText(), (*s.right).toText())
	}
}

func visitForSplit(node *snailfishNumber) bool {
	if node.value > 9 {
		num := node.value / 2
		rem := node.value - num*2

		l := snailfishNumber{value: num}
		r := snailfishNumber{value: num + rem}

		sp := snailfishNumber{
			left:   &l,
			right:  &r,
			parent: node.parent,
			value:  -1,
		}

		l.parent = &sp
		r.parent = &sp

		if node == node.parent.left {
			node.parent.left = &sp
		} else {
			node.parent.right = &sp
		}
		return true
	}
	return false
}

func reduceForSplit(sp *snailfishNumber) bool {
	changed := false

	if sp.left.value >= 0 {
		changed = changed || visitForSplit(sp.left)
	} else {
		changed = changed || reduceForSplit(sp.left)
	}

	if sp.right.value >= 0 {
		changed = changed || visitForSplit(sp.right)
	} else {
		changed = changed || reduceForSplit(sp.right)
	}

	return changed
}

func reduceForExplode(sp *snailfishNumber, depth int) bool {
	if depth == 5 {
		//Try to add left value
		if sp == sp.parent.right {
			//If we are right node, go up, left, then right until end
			current := sp.parent.left
			for current.value < 0 {
				current = current.right
			}
			current.value += sp.left.value
		} else {
			//If we are left node, go up until we can go left, then right until end
			current := sp
			for current.parent != nil && current == current.parent.left {
				current = current.parent
			}
			if current != nil && current.parent != nil {
				current = current.parent.left
				for current.value < 0 {
					current = current.right
				}
				current.value += sp.left.value
			}
		}

		//Try to add right value
		if sp == sp.parent.left {
			//If we are left node, go up, right, then left until end
			current := sp.parent.right
			for current.value < 0 {
				current = current.left
			}
			current.value += sp.right.value
		} else {
			//If we are right node, go up until we can go right, then left until end
			current := sp
			for current.parent != nil && current == current.parent.right {
				current = current.parent
			}
			if current != nil && current.parent != nil {
				current = current.parent.right
				for current.value < 0 {
					current = current.left
				}
				current.value += sp.right.value
			}
		}

		t := snailfishNumber{
			value:  0,
			parent: sp.parent,
		}
		if sp.parent.left == sp {
			sp.parent.left = &t
		} else {
			sp.parent.right = &t
		}

		return true
	} else {
		changed := false
		if sp.left.value < 0 {
			changed = changed || reduceForExplode(sp.left, depth+1)
		}
		if sp.right.value < 0 {
			changed = changed || reduceForExplode(sp.right, depth+1)
		}
		return changed
	}
}

func add(a, b *snailfishNumber) *snailfishNumber {
	sp := &snailfishNumber{
		left:  a,
		right: b,
		value: -1,
	}
	a.parent = sp
	b.parent = sp

	changed := true
	for changed {
		changed = false
		changed = changed || reduceForExplode(sp, 1)
		changed = changed || reduceForSplit(sp)
	}

	return sp
}

func parse(str string) *snailfishNumber {
	sp, _ := parseInternal(str, nil)
	return sp
}

func parseInternal(str string, parent *snailfishNumber) (*snailfishNumber, string) {
	switch str[0] {
	case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
		return &snailfishNumber{
			value:  utils.GetAllNumbers(string(str[0]))[0],
			parent: parent,
		}, str[1:]
	}

	sp := snailfishNumber{
		value:  -1,
		parent: parent,
	}
	sp.left, str = parseInternal(str[1:], &sp)
	sp.right, str = parseInternal(str[1:], &sp)

	return &sp, str[1:]
}

func part1(input string) int {
	var num *snailfishNumber
	for _, line := range strings.Split(input, "\n") {
		s := parse(line)
		if num == nil {
			num = s
		} else {
			res := add(num, s)
			num = res
		}
	}

	return num.magnitude()
}

func part2(input string) int {
	high := 0
	for _, line1 := range strings.Split(input, "\n") {
		for _, line2 := range strings.Split(input, "\n") {
			if line1 != line2 {
				a := parse(line1)
				b := parse(line2)
				s := add(a, b)
				m := s.magnitude()
				high = max(m, high)
			}
		}
	}

	return high
}
