/**
 * @file lru_cache.cpp
 * @brief https://leetcode.com/problems/lru-cache/
 * @date 2024-03-31
 */
#include <cassert>
#include <list>
#include <unordered_map>
#include <utility>
using namespace std;

class LRUCache {
private:
	unordered_map<int, pair<int, list<int>::iterator>> cache_;
	list<int> last_accessed_;
	int capacity_;

public:
	LRUCache(int capacity)
	: capacity_{capacity} {}

	int get(int key) {
		if (cache_.contains(key)) {
			auto &[value, iter] = cache_[key];
			last_accessed_.erase(iter);
			last_accessed_.push_back(key);
            iter = prev(last_accessed_.end());
			return value;
		} else {
			return -1;
		}
	}

	void put(int key, int value) {
        if (cache_.contains(key)) {
            last_accessed_.erase(cache_[key].second);
			cache_.erase(key);
        } else if (cache_.size() == capacity_) {
			cache_.erase(last_accessed_.front());
			last_accessed_.pop_front();
		}
		last_accessed_.push_back(key);
		pair<int, list<int>::iterator> val =
		    make_pair(value, prev(last_accessed_.end()));
		cache_.emplace(key, val);
	}
};

int main() {
	{
		LRUCache *obj = new LRUCache(2);
		obj->put(1, 1);
		obj->put(2, 2);
		assert(obj->get(1) == 1);
		obj->put(3, 3);
		assert(obj->get(2) == -1);
		obj->put(4, 4);
		assert(obj->get(1) == -1);
		assert(obj->get(3) == 3);
		assert(obj->get(4) == 4);
		delete obj;
	}
	{
		LRUCache *obj = new LRUCache(2);
		obj->put(2, 1);
		obj->put(2, 2);
		assert(obj->get(2) == 2);
		obj->put(1, 1);
		obj->put(4, 1);
		assert(obj->get(2) == -1);
		delete obj;
	}
	{
		LRUCache *obj = new LRUCache(2);
		obj->put(2, 1);
		obj->put(1, 1);
		assert(obj->get(2) == 1);
		obj->put(4, 1);
		assert(obj->get(1) == -1);
		assert(obj->get(2) == 1);
		delete obj;
	}
	return 0;
}
