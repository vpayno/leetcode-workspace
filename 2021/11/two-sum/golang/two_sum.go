package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

var out io.Writer = os.Stdout

func twoSum(nums []int, target int) []int {

	for num1 := 0; num1 < len(nums); num1++ {
		for num2 := num1 + 1; num2 < len(nums); num2++ {

			test_sum := nums[num1] + nums[num2]

			// fmt.Fprintln(os.Stderr, strconv.Itoa(test_sum) + " = " + strconv.Itoa(num1) + " + " + strconv.Itoa(num2) + " == " + strconv.Itoa(target))

			if test_sum == target {
				return []int{num1, num2}
			}
		}
	}

	return make([]int, 0)
} // two_sum()

func main() {
	args := os.Args[1:]

	if len(args) != 2 {
		fmt.Fprintln(os.Stderr, "Missing argument(s)")
		os.Exit(1)
	}

	var nums []int

	for _, i := range strings.Split(os.Args[1], " ") {
		num, error := strconv.Atoi(i)
		if error != nil {
			fmt.Fprintln(os.Stderr, "Error converting 1st argument to integer array.")
			os.Exit(1)
		}
		nums = append(nums, num)
	}

	target, error := strconv.Atoi(args[1])
	if error != nil {
		fmt.Fprintln(os.Stderr, "Error converting 2nd argument to integer.")
		os.Exit(1)
	}

	nums_bytes, _ := json.Marshal(nums)
	nums_str := string(nums_bytes)
	target_str := strconv.Itoa(target)
	fmt.Fprintln(os.Stderr, "nums := " + nums_str + "\ntarget := " + target_str)

	fmt.Fprintln(out, twoSum(nums, target))
} // main()
