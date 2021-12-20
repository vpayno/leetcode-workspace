package main

import (
	"bytes"
	"testing"
	"strings"
	"os"
)

func TestMain(t *testing.T) {

	tests := []struct {
		name   string
		args   []string
		expect string
	}{
		{
			name: "testcase1",
			args: []string{ "./two_sum", "1 2 3 4 5", "5" },
			expect: "[0 3]",
		}, {
			name: "testcase2",
			args: []string{ "./two_sum", "3 3", "6" },
			expect: "[0 1]",
		}, {
			name: "testcase3",
			args: []string{ "./two_sum", "1 2 3", "7" },
			expect: "[]",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			os.Args = tt.args
			out = bytes.NewBuffer(nil)
			main()

			if result := out.(*bytes.Buffer).String(); strings.TrimRight(result, "\r\n") != tt.expect {
					t.Errorf("%v: expected \"%v\", got \"%v\"", tt.name, tt.expect, result)
				}
		})
	}
} // testMain()

func TestTwoSum(t *testing.T) {
	type args struct {
		nums   []int
		target int
	}

	tests := []struct {
		name   string
		args   args
		expect []int
	}{
		{
			name: "testcase1",
			args: args{
				nums:   []int{1, 2, 3, 4, 5},
				target: 5,
			},
			expect: []int{0, 3},
		}, {
			name: "testcase2",
			args: args{
				nums:   []int{3, 3},
				target: 6,
			},
			expect: []int{0, 1},
		}, {
			name: "testcase3",
			args: args{
				nums:   []int{2, 7, 11, 15},
				target: 9,
			},
			expect: []int{0, 1},
		}, {
			name: "testcase4",
			args: args{
				nums:   []int{3, 2, 4},
				target: 6,
			},
			expect: []int{1, 2},
		}, {
			name: "testcase5",
			args: args{
				nums:   []int{},
				target: 6,
			},
			expect: []int{},
		}, {
			name: "testcase6",
			args: args{
				nums:   []int{1, 2, 3},
				target: 7,
			},
			expect: []int{},
		}, {
			name: "testcase7",
			args: args{
				nums:   []int{3, 4, 1, 7, 5},
				target: 11,
			},
			expect: []int{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := twoSum(tt.args.nums, tt.args.target)
			for i := range tt.expect {
				if result[i] != tt.expect[i] {
					t.Errorf("%v: expected %v, got %v", tt.name, tt.expect, result)
				}
			}
		})
	}

} // TestTwoSum()
