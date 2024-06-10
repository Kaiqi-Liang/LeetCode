/**
 * https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/
 * @date 2020-08-06
 */
public class NumberOfStudentsDoingHomeWorkAtAGivenTime {
    public static void main(String[] args) {
        int startTime[] = {1, 2 ,3};
        int endTime[] = {2, 3 ,7};
        int query = 4;
        assert busyStudent(startTime, endTime, query) == 1;
    }

    private static int busyStudent(int[] startTime, int[] endTime, int queryTime) {
        int count = 0;
        for (int i = 0; i < startTime.length; i++) {
            if (queryTime >= startTime[i] && queryTime <= endTime[i]) {
                count++;
            }
        }
        return count;
    }
}
