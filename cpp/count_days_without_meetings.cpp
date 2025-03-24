/**
 * @file count_days_without_meetings.cpp
 * @brief https://leetcode.com/problems/count-days-without-meetings/
 * @date 2025-03-24
 */
#include <algorithm>
#include <cassert>
#include <map>
#include <vector>
using namespace std;

namespace line_sweep {
	int countDays(int days, vector<vector<int>> const &meetings) {
		map<int, int> line_sweep = ranges::fold_left(
		    meetings,
		    map<int, int>{},
		    [](map<int, int> a, vector<int> const &c) {
			    ++a[c[0]];
			    --a[c[1]];
			    return a;
		    }
		);
		int num_running_meetings = 0;
		int num_days = line_sweep.cbegin()->first - 1;
		int start_break_day = 0;
		for (auto &&[day, num_meetings] : line_sweep) {
			if (start_break_day > 0) {
				num_days += day - start_break_day - 1;
				start_break_day = 0;
			}
			num_running_meetings += num_meetings;
			if (num_running_meetings == 0) {
				start_break_day = day;
			}
		}
		return num_days + max(0, days - line_sweep.crbegin()->first);
	}
} // namespace line_sweep

namespace greedy {
	int countDays(int days, vector<vector<int>> &meetings) {
		ranges::sort(meetings);
		int num_days = meetings.front().front() - 1;
		int last_end_day = 0;
		for (size_t i = 1; i < meetings.size(); ++i) {
			last_end_day = max(last_end_day, meetings[i - 1].back());
			num_days += max(0, meetings[i].front() - last_end_day - 1);
		}
		return num_days +
		       max(0, days - max(last_end_day, meetings.back().back()));
	}
} // namespace greedy

int main() {
	assert(line_sweep::countDays(10, {{5, 7}, {1, 3}, {9, 10}}) == 2);
	assert(line_sweep::countDays(5, {{2, 4}, {1, 3}}) == 1);
	assert(line_sweep::countDays(6, {{1, 6}}) == 0);
	assert(line_sweep::countDays(8, {{2, 3}, {3, 5}, {8, 8}}) == 3);
	assert(
	    line_sweep::countDays(
	        57,
	        {{3, 49},
	         {23, 44},
	         {21, 56},
	         {26, 55},
	         {23, 52},
	         {2, 9},
	         {1, 48},
	         {3, 31}}
	    ) == 1
	);
	{
		vector<vector<int>> meetings{{5, 7}, {1, 3}, {9, 10}};
		assert(greedy::countDays(10, meetings) == 2);
	}
	{
		vector<vector<int>> meetings{{2, 4}, {1, 3}};
		assert(greedy::countDays(5, meetings) == 1);
	}
	{
		vector<vector<int>> meetings{{1, 6}};
		assert(greedy::countDays(6, meetings) == 0);
	}
	{
		vector<vector<int>> meetings{{2, 3}, {3, 5}, {8, 8}};
		assert(greedy::countDays(8, meetings) == 3);
	}
	{
		vector<vector<int>> meetings{
		    {3, 49},
		    {23, 44},
		    {21, 56},
		    {26, 55},
		    {23, 52},
		    {2, 9},
		    {1, 48},
		    {3, 31},
		};
		assert(greedy::countDays(57, meetings) == 1);
	}
	return 0;
}
