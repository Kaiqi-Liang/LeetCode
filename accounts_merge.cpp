/**
 * @file accounts_merge.cpp
 * @brief https://leetcode.com/problems/accounts-merge/
 * @date 2024-04-08
 */
#include <algorithm>
#include <cassert>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

vector<vector<string>> accountsMerge(vector<vector<string>> const &accounts) {
	unordered_multimap<string, unordered_set<string>> name_to_emails;
	for (auto &&account : accounts) {
		if (name_to_emails.contains(account[0])) {
			auto range = name_to_emails.equal_range(account[0]);
			auto iter = range.first;
			vector<unordered_multimap<string, unordered_set<string>>::iterator>
			    iters;
			for (; iter != range.second; ++iter) {
				auto email = next(account.cbegin());
				for (; email != account.cend(); ++email) {
					if (iter->second.contains(*email)) {
						iters.push_back(iter);
						break;
					}
				}
				if (email != account.cend()) {
					iter->second.insert(next(account.cbegin()), account.cend());
				}
			}
			if (iters.size() == 0) {
				name_to_emails.emplace(
				    account[0],
				    unordered_set<string>(
				        next(account.cbegin()),
				        account.cend()
				    )
				);
			} else {
				unordered_set<string> emails{};
				for (auto &&i : iters) {
					emails.insert(i->second.cbegin(), i->second.cend());
					name_to_emails.erase(i);
				}
				name_to_emails.emplace(account[0], emails);
			}
		} else {
			name_to_emails.emplace(
			    account[0],
			    unordered_set<string>(next(account.cbegin()), account.cend())
			);
		}
	}
	vector<vector<string>> res;
	for (auto &&[name, emails] : name_to_emails) {
		res.push_back(vector<string>(emails.cbegin(), emails.cend()));
		sort(res.back().begin(), res.back().end());
		res.back().insert(res.back().begin(), name);
	}
	return res;
}

int main() {
	assert(
	    accountsMerge(
	        {{"David", "David0@m.co", "David1@m.co"},
	         {"David", "David3@m.co", "David4@m.co"},
	         {"David", "David4@m.co", "David5@m.co"},
	         {"David", "David2@m.co", "David3@m.co"},
	         {"David", "David1@m.co", "David2@m.co"}}
	    ) ==
	    vector<vector<string>>(
	        {{"David",
	          "David0@m.co",
	          "David1@m.co",
	          "David2@m.co",
	          "David3@m.co",
	          "David4@m.co",
	          "David5@m.co"}}
	    )
	);
	assert(
	    accountsMerge(
	        {{"John", "johnsmith@mail.com", "john_newyork@mail.com"},
	         {"John", "johnsmith@mail.com", "john00@mail.com"},
	         {"Mary", "mary@mail.com"},
	         {"John", "johnnybravo@mail.com"}}
	    ) ==
	    vector<vector<string>>(
	        {{"Mary", "mary@mail.com"},
	         {"John",
	          "john00@mail.com",
	          "john_newyork@mail.com",
	          "johnsmith@mail.com"},
	         {"John", "johnnybravo@mail.com"}}
	    )
	);
	return 0;
}
