import java.util.Arrays;

import static org.junit.Assert.assertTrue;

import org.junit.Test;

public class TwoSumTest {

	public TwoSumTest() {
	}

	void testTwoSum(String name, int[] nums, int target, int[] expected) {
		int[] result;

		Solution solution = new Solution();

		result = solution.twoSum(nums, target);

		System.err.println("[" + name + "]   result: " + Arrays.toString(result));
		System.err.println("[" + name + "] expected: " + Arrays.toString(expected));

		assertTrue(Arrays.equals(result, expected));

		System.out.println();
	}

	@Test
	public void testTwoSum() {
		int[] nums;
		int target;
		int[] expected;
		String name;

		TwoSumTest test = new TwoSumTest();

		try {
			name = new String("testcase1");
			nums = new int[]{1, 2, 3};
			target = 5;
			expected = new int[]{1, 2};
			test.testTwoSum(name, nums, target, expected);

			name = new String("testcase2");
			nums = new int[]{1, 2, 3, 4, 5};
			target = 7;
			expected = new int[]{1, 4};
			test.testTwoSum(name, nums, target, expected);

			name = new String("testcase3");
			nums = new int[]{3, 3};
			target = 6;
			expected = new int[]{0, 1};
			test.testTwoSum(name, nums, target, expected);

			name = new String("testcase4");
			nums = new int[]{4, 5, 1, 3, 2};
			target = 4;
			expected = new int[]{2, 3};
			test.testTwoSum(name, nums, target, expected);
		}
		catch(Exception e) {
			System.err.println(e);
		}
	}
}
