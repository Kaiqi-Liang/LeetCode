/**
 * https://leetcode.com/problems/trapping-rain-water/
 * @date 2020-08-05
 */
public class TrappingRainWater {
    private static final int heights[] = {1,3,2,4,1,3,1,4,5,2,2,1,4,2,2};
    public static void main(String[] args) {
        assert totalVolume() == 15;
    }

    private static int totalVolume() {
        int result = 0;
        for (int i = 0; i < heights.length; i++) result += columnVolume(i);
        return result;
    }

    private static int columnVolume(int i) {
        int height = heights[i];
        int left = height;
        for (int j = i - 1; j >= 0; j--) { // Find the left highest
            if (heights[j] > left) left = heights[j];
        }
        int right = height;
        for (int j = i + 1; j < heights.length; j++) { // Find the right highest
            if (heights[j] > right) right = heights[j];
        }
        return left < right ? left - height : right - height;
    }
}
