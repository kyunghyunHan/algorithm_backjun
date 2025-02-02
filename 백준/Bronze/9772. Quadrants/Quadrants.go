package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	reader := bufio.NewScanner(os.Stdin)
	writer := bufio.NewWriter(os.Stdout)
	defer writer.Flush()

	for reader.Scan() {
		line := strings.TrimSpace(reader.Text())
		parts := strings.Fields(line)

		if len(parts) != 2 {
			continue
		}

		x, err1 := strconv.ParseFloat(parts[0], 32)
		y, err2 := strconv.ParseFloat(parts[1], 32)

		if err1 != nil || err2 != nil {
			continue
		}

		if x == 0.0 && y == 0.0 {
			fmt.Fprintln(writer, "AXIS")
			break
		}

		if x > 0.0 && y > 0.0 {
			fmt.Fprintln(writer, "Q1")
		} else if x < 0.0 && y > 0.0 {
			fmt.Fprintln(writer, "Q2")
		} else if x < 0.0 && y < 0.0 {
			fmt.Fprintln(writer, "Q3")
		} else if x > 0.0 && y < 0.0 {
			fmt.Fprintln(writer, "Q4")
		} else {
			fmt.Fprintln(writer, "AXIS")
		}
	}
}
