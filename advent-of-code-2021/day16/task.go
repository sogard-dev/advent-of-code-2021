package day16

import (
	"fmt"
	"math"
	"strconv"
)

func binToInt(str string) int {
	i, _ := strconv.ParseInt(str, 2, 64)
	return int(i)
}

func strToBin(str string) string {
	ret := []rune{}
	for _, s := range str {
		num, _ := strconv.ParseInt(string(s), 16, 16)

		b := fmt.Sprintf("%04b", num)
		for _, c := range b {
			ret = append(ret, c)
		}
	}
	return string(ret)
}

type literal struct {
	packetVersion int
	packetType    int
	literal       int
}

type operator struct {
	packetVersion int
	packetType    int
	packets       []packet
}

func (l literal) getPacketVersion() int {
	return l.packetVersion
}

func (l literal) value() int {
	return l.literal
}

func (o operator) getPacketVersion() int {
	return o.packetVersion
}

func (o operator) value() int {
	switch o.packetType {
	case 0:
		s := 0
		for _, c := range o.packets {
			s += c.value()
		}
		return s
	case 1:
		s := 1
		for _, c := range o.packets {
			s *= c.value()
		}
		return s
	case 2:
		s := math.MaxInt
		for _, c := range o.packets {
			s = min(s, c.value())
		}
		return s
	case 3:
		s := -1
		for _, c := range o.packets {
			s = max(s, c.value())
		}
		return s
	case 5:
		if o.packets[0].value() > o.packets[1].value() {
			return 1
		}
		return 0
	case 6:
		if o.packets[0].value() < o.packets[1].value() {
			return 1
		}
		return 0
	case 7:
		if o.packets[0].value() == o.packets[1].value() {
			return 1
		}
		return 0
	}

	panic("Unknown package")
}

type packet interface {
	getPacketVersion() int
	value() int
}

func readLiteral(msg string, ptr int) (int, literal) {
	v := binToInt(msg[ptr : ptr+3])
	t := binToInt(msg[ptr+3 : ptr+6])

	ptr = ptr + 6
	b := ""
	for {
		b = b + msg[ptr+1:ptr+5]
		ptr = ptr + 5
		if msg[ptr-5] == '0' {
			break
		}
	}

	return ptr, literal{
		packetVersion: v,
		packetType:    t,
		literal:       binToInt(b),
	}
}

func readOperator(msg string, ptr int) (int, operator) {
	v := binToInt(msg[ptr : ptr+3])
	t := binToInt(msg[ptr+3 : ptr+6])
	i := binToInt(msg[ptr+6 : ptr+7])
	ptr = ptr + 7

	op := operator{
		packetVersion: v,
		packetType:    t,
		packets:       []packet{},
	}

	if i == 0 {
		toRead := binToInt(msg[ptr : ptr+15])
		ptr = ptr + 15
		stopWhen := toRead + ptr
		for ptr < stopWhen {
			ptr = readPacket(msg, &op, ptr)
		}
	} else {
		toRead := binToInt(msg[ptr : ptr+11])
		ptr = ptr + 11
		for len(op.packets) < toRead {
			ptr = readPacket(msg, &op, ptr)
		}
	}

	return ptr, op
}

func readPacket(msg string, parent *operator, ptr int) int {
	t := binToInt(msg[ptr+3 : ptr+6])

	if t == 4 {
		p, l := readLiteral(msg, ptr)
		parent.packets = append(parent.packets, l)
		return p
	} else {
		p, l := readOperator(msg, ptr)
		parent.packets = append(parent.packets, l)
		return p
	}
}

func readPackets(msg string) packet {
	rootPacket := operator{
		packets: []packet{},
	}

	ptr := 0
	for ptr+10 < len(msg) {
		ptr = readPacket(msg, &rootPacket, ptr)
	}

	return rootPacket
}

func sumPacketVersions(p packet) int {
	s := p.getPacketVersion()

	switch v := p.(type) {
	case operator:
		for _, c := range v.packets {
			s += sumPacketVersions(c)
		}
	}

	return s
}

func part1(input string) int {
	bin := strToBin(input)
	packets := readPackets(bin)
	return sumPacketVersions(packets)
}

func part2(input string) int {
	bin := strToBin(input)
	packets := readPackets(bin)
	return packets.value()
}
