/**
 * @file h_index.cpp
 * @brief https://leetcode.com/problems/h-index/
 * @date 2024-12-21
 */
#include <algorithm>
#include <cassert>
#include <iterator>
#include <vector>
using namespace std;
int hIndex(vector<size_t> &citations) {
	sort(citations.begin(), citations.end());
	size_t low = 0;
	size_t high = min(citations.back(), citations.size());
	size_t h_index = 0;
	while (low <= high) {
		size_t mid = (low + high) / 2;
		size_t num_citations_at_least_mid = distance(
		    lower_bound(citations.cbegin(), citations.cend(), mid),
		    citations.cend()
		);
		if (num_citations_at_least_mid >= mid) {
			h_index = max(h_index, mid);
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}
	return h_index;
}

int main() {
	{
		auto citations = vector<size_t>{3, 0, 6, 1, 5};
		assert(hIndex(citations) == 3);
	}
	{
		auto citations = vector<size_t>{1, 3, 1};
		assert(hIndex(citations) == 1);
	}
	return 0;
}
