import java.util.*;
import java.util.stream.*;

class Solution {
    public int[] twoSum(int[] nums, int target) {

		int num1;
		int num2;
		int testSum;

		for(int i = 0; i < nums.length; i++) {
			for(int j = i+1; j < nums.length; j++) {
				num1 = nums[i];
				num2 = nums[j];

				testSum = num1 + num2;

				if(testSum == target) {
					return new int[]{ i, j };
				}
			}
		}

		return new int[]{};
    }

	public static void main(String[] args) {
		int[] nums;
		int target;

		int[] result;

		System.err.println(Arrays.toString(args));

		nums = Arrays.stream(args[0].split(" ")).map(String::trim).mapToInt(Integer::parseInt).toArray();
		target = Integer.parseInt(args[1]);

		Solution solution = new Solution();

		result = solution.twoSum(nums, target);

		System.out.println(Arrays.toString(result));
	}
}
