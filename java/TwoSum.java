/**
 * https://leetcode.com/problems/two-sum/
 * @date 2020-08-05
 */
public class TwoSum {
    public static void main(String[] args) {
        int nums[] = {2, 0, 15, 7};
        int target = 9;
        System.out.printf("[%d, %d]\n", twoSum(nums, target)[0], twoSum(nums, target)[1]);
    }

    private static int[] twoSum(int[] nums, int target) {
        for (int i = 0; i < nums.length; i++) {
            for (int j = i + 1; j < nums.length; j++) {
                if (i != j && nums[i] + nums[j] == target) return new int[] {i , j};
            }
        }
        throw new IllegalArgumentException("No two sum solution");
    }
}
