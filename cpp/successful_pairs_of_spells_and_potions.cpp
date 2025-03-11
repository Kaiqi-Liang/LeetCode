/**
 * @file successful_pairs_of_spells_and_potions.cpp
 * @brief https://leetcode.com/problems/successful-pairs-of-spells-and-potions/?envType=study-plan-v2&envId=leetcode-75
 * @date 2025-03-12
 */
#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

vector<int> successfulPairs(
    vector<int> const &spells,
    vector<int> &potions,
    long long success
) {
	vector<int> pairs;
	sort(potions.begin(), potions.end());
	for (long long spell : spells) {
		long double minimum_potion_required =
		    ceil(static_cast<long double>(success) / spell);
		auto iter = lower_bound(
		    potions.begin(),
		    potions.end(),
		    minimum_potion_required
		);
		pairs.emplace_back(distance(iter, potions.end()));
	}
	return pairs;
}

int main() {
	{
		vector<int> spells{5, 1, 3};
		vector<int> potions{1, 2, 3, 4, 5};
		assert(successfulPairs(spells, potions, 7) == vector({4, 0, 3}));
	}
	{
		vector<int> spells{3, 1, 2};
		vector<int> potions{8, 5, 8};
		assert(successfulPairs(spells, potions, 16) == vector({2, 0, 2}));
	}
	return 0;
}
