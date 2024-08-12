/**
 * @file kth_largest_element_in_a_stream.cpp
 * @brief https://leetcode.com/problems/kth-largest-element-in-a-stream/
 * @date 2024-08-12
 */
#include <cassert>
#include <iterator>
#include <set>
#include <vector>
using namespace std;
class KthLargest {
public:
	KthLargest(size_t k, vector<int> const &nums)
	: k_{k}
	, nums_{multiset<int, greater<int>>{nums.cbegin(), nums.cend()}} {}

	int add(int val) {
		nums_.insert(val);
		auto iter = nums_.cbegin();
		advance(iter, k_ - 1);
		return *iter;
	}

private:
	size_t k_;
	multiset<int, greater<int>> nums_;
};

int main() {
	KthLargest *obj = new KthLargest(3, {4, 5, 8, 2});
	assert(obj->add(3) == 4);
	assert(obj->add(5) == 5);
	assert(obj->add(10) == 5);
	assert(obj->add(9) == 8);
	assert(obj->add(4) == 8);
	return 0;
}
