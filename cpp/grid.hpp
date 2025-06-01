#pragma once
#include <array>
#include <functional>
using namespace std;

using Point = pair<int, int>;

constexpr array<Point, 4> NEIGHBOURS = {{
    {0, 1},
    {0, -1},
    {1, 0},
    {-1, 0},
}};

namespace std {
	template<>
	struct hash<Point> {
		int operator()(const Point &point) const noexcept {
			return point.first * 31 + point.second;
		}
	};
} // namespace std
