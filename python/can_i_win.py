"""
https://leetcode.com/problems/can-i-win/
"""


def can_player_1_force_win(n: int, target: int) -> bool:
    options = tuple(i for i in range(1, n + 1))
    cache: dict[tuple[int, ...], bool] = {}

    def determine_win(options: tuple[int, ...], difference: int) -> bool:
        key = tuple(options)
        if key in cache:
            return cache[key]
        is_won = False
        for option in options:
            if option >= difference:
                cache[key] = True
                return True
            if not determine_win(tuple(o for o in options if o != option), difference - option):
                is_won = True
                break
        cache[key] = is_won
        return is_won
    return determine_win(options, target)


assert not can_player_1_force_win(2, 3)
assert can_player_1_force_win(3, 3)
assert can_player_1_force_win(3, 6)
assert can_player_1_force_win(15, 50)
assert can_player_1_force_win(20, 50)
assert not can_player_1_force_win(6, 21)
