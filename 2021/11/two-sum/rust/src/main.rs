use std::env;
use std::process;

const DEBUG: bool = false;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num1: i32;
        let mut num2: i32;
        let mut test_sum: i32;

        for index1 in 0..nums.len() {
            //index = nums.iter().position(|&r| r == num1).unwrap();

            //for num2 in nums[(index + (1 as usize))..] {
            for index2 in (index1 + 1)..nums.len() {
                num1 = nums[index1];
                num2 = nums[index2];
                if DEBUG {
                    println!(
                        "{:?} + {:?} = {:?} == {:?}",
                        num1,
                        num2,
                        (num1 + num2),
                        target
                    );
                }

                test_sum = num1 + num2;

                if test_sum == target {
                    if DEBUG {
                        println!("Found solution.");
                    }
                    //index1 = nums.iter().position(|&r| r == (num1 as &i32)).unwrap();
                    //index2 = nums[(index1 + 1)..] .iter() .position(|&r| r == num2) .unwrap();
                    //return [index1, index2 + index1 + 1];
                    return vec![index1 as i32, index2 as i32];
                }
            }
        }
        if DEBUG {
            println!("No solution.");
        }

        return vec![];
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // +1 since arg 0 is the program path
    if args.len() != 3 {
        eprintln!("Missing argument(s)");
        eprintln!("Arguments: {:?}", args);
        process::exit(1);
    }

    println!("args[1]: {:?}", args[1]);
    println!("args[2]: {:?}", args[2]);

    let nums: Vec<i32> = args[1]
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let target: i32 = args[2].parse().unwrap();

    // let solution = Solution::two_sum(vec![1, 2, 3, 4, 5], 7);
    let solution = Solution::two_sum(nums, target);
    println!("{:?}", solution);
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[rstest]
    #[case(vec![2, 7, 11, 15], 9, vec![0, 1])]
    #[case(vec![3, 3], 6, vec![0, 1])]
    #[case(vec![3, 2, 4], 6, vec![1, 2])]
    #[case(vec![1, 2, 3, 4, 5], 7, vec![1, 4])]
    #[case(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 11, vec![2, 9])]
    #[case(vec![4, 7, 9, 14, 18, 23], 21, vec![1, 3])]
    #[case(vec![1, 2, 3], 5, vec![1, 2])]
    #[case(vec![1, 2, 3, 4, 5], 11, vec![])]
    #[case(vec![], 0, vec![])]
    fn test_two_sum_parameterized(
        #[case] nums: Vec<i32>,
        #[case] target: i32,
        #[case] expected: Vec<i32>,
    ) {
        assert_eq!(expected, super::Solution::two_sum(nums, target));
    }

    #[test]
    fn test_two_sum_monolith() {
        assert_eq!(vec![0, 1], super::Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![0, 1], super::Solution::two_sum(vec![3, 3], 6));
        assert_eq!(vec![1, 4], super::Solution::two_sum(vec![1, 2, 3, 4, 5], 7));
        assert_eq!(
            vec![2, 9],
            super::Solution::two_sum(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 11)
        );
        assert_eq!(
            vec![1, 3],
            super::Solution::two_sum(vec![4, 7, 9, 14, 18, 23], 21)
        );
        assert_eq!(vec![1, 2], super::Solution::two_sum(vec![1, 2, 3], 5));
        //assert_eq!(vec![], super::Solution::two_sum(vec![1, 2, 3, 4, 5], 11));
        //assert_eq!(vec![], super::Solution::two_sum(vec![], 0));
    }
} // mod test
