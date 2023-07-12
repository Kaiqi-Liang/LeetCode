/**
 * @file longest_increasing_subsequence.cpp
 * @brief https://leetcode.com/problems/longest-increasing-subsequence/
 * @date 2023-07-12
 */
#include <algorithm>
#include <iostream>
#include <vector>

namespace dynamic_programming {
	auto lengthOfLIS(std::vector<int> const &nums) -> std::size_t {
		auto opt = std::vector<std::size_t>(nums.size(), 1);
		for (auto i = std::size_t{1}; i < nums.size(); ++i) {
			auto prev = std::vector<std::size_t>{};
			for (auto j = std::size_t{0}; j < i; ++j) {
				if (nums[j] < nums[i]) {
					prev.push_back(opt[j]);
				}
			}
			auto max = std::max_element(prev.cbegin(), prev.cend());
			if (max != prev.cend()) opt[i] = *max + 1;
		}
		return *std::max_element(opt.cbegin(), opt.cend());
	}
}  // namespace dynamic_programming

namespace range_tree {
	using namespace std;

	// add base to each number to convert them into the range [0, 20000] from [-10000, 10000]
	constexpr int base = 10000;
	constexpr int N = base * 2;
	constexpr int TREE_SIZE = N * 4;

	// instead of explicitly storing each node's range of responsibility [cL,cR), we calculate it on the way down
	// the root node is responsible for [0, N) not [0,n)
	int tree[TREE_SIZE]; // range max tree over array values (not indices)

	int n;  // actual length of underlying array

	/**
	 * @brief Query the sum over [qL, qR) (0-based)
	 * @param i the index in the tree, rooted at 1 so children are 2i and 2i+1
	 */
	int query(int qL, int qR, int i = 1, int cL = 0, int cR = N) {
		if (cL == qL && cR == qR) return tree[i];
		int mid = (cL + cR) / 2;
		int ans = 0;
		if (qL < mid) ans = max(0, query(qL, min(qR, mid), i * 2, cL, mid));
		if (qR > mid) ans = max(ans, query(max(qL, mid), qR, i * 2 + 1, mid, cR));
		return ans;
	}

	/**
	 * @brief Point update a single value in the range tree
	 * @param p the index in the array (0-based)
	 * @param v the value that the p-th element will be updated to
	 * @param i the index in the tree, rooted at 1 so children are 2i and 2i+1
	 */
	void update(int p, int v, int i = 1, int cL = 0, int cR = N) {
		if (cR - cL == 1) { // this node is a leaf, so apply the update
			tree[i] = v;
			return;
		}
		int mid = (cL + cR) / 2;
		if (p < mid) update(p, v, i * 2, cL, mid);
		else update(p, v, i * 2 + 1, mid, cR);
		// once we have updated the correct child, recalculate our stored value.
		tree[i] = max(tree[i * 2], tree[i * 2 + 1]);
	}

	auto lengthOfLIS(std::vector<int> const& nums) -> int {
		n = nums.size();
		fill_n(tree, TREE_SIZE, 0);
		for (auto && num: nums) {
			int best = 1 + query(0, num + base);
			update(num + base, best);
		}
		return query(0, N);
	}
} // namespace range_tree

auto main() -> int {
	assert(dynamic_programming::lengthOfLIS({-2, -1}) == range_tree::lengthOfLIS({-2, -1}));
	assert(dynamic_programming::lengthOfLIS({10, 9, 2, 5, 3, 7, 101, 18}) == range_tree::lengthOfLIS({10, 9, 2, 5, 3, 7, 101, 18}));
	assert(dynamic_programming::lengthOfLIS({0, 1, 0, 3, 2, 3}) == range_tree::lengthOfLIS({0, 1, 0, 3, 2, 3}));
	assert(dynamic_programming::lengthOfLIS({7, 7, 7, 7, 7, 7, 7}) == range_tree::lengthOfLIS({7, 7, 7, 7, 7, 7, 7}));
	assert(dynamic_programming::lengthOfLIS({1, 3, 4, 5, 6, 2, 8, 7}) == range_tree::lengthOfLIS({1, 3, 4, 5, 6, 2, 8, 7}));
	return 0;
}
