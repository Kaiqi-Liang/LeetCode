/**
 * https://leetcode.com/problems/subarray-sum-equals-k/
 * @date 2020-08-05
 */
public class SubarraySumEqualsK {
    public static void main(String[] args) {
        int nums[] = {0, 0, 0, 0};
        int k = 0;
        assert subarraySum(nums, k) == 10;
    }

    private static int subarraySum(int[] nums, int k) {
        int count = 0;
        for (int i = 0; i < nums.length; i++) {
            int sum = nums[i];
            boolean flag = false;
            if (sum == k) {
                count++;
                flag = true;
            }
            for (int j = i + 1; j < nums.length; j++) {
                sum += nums[j];
                if (sum == k) {
                    count++;
                    flag = true;
                }
            }
            if (!flag && sum == k) {
                count++;
            }
        }
        return count;
    }
}
