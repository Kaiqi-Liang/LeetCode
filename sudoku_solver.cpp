/**
 * @file sudoku_solver.cpp
 * @brief https://leetcode.com/problems/sudoku-solver/
 * @date 2024-04-07
 */
#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

using Board = vector<vector<char>>;
constexpr size_t SIZE = 9;
constexpr size_t SUBGRID_SIZE = 3;

bool is_valid(Board const &board, size_t row, size_t col, size_t num) {
	// check the row
	if (find_if(board[row].cbegin(), board[row].cend(), [num](char cell) {
		    return cell != '.' and cell == num + '0';
	    }) != board[row].cend())
	{
		return false;
	}
	// check the column
	for (size_t i = 0; i < SIZE; ++i) {
		if (board[i][col] == num + '0') {
			return false;
		}
	}
	// check the subgrid
	for (size_t i = row / SUBGRID_SIZE * SUBGRID_SIZE;
	     i < (row / SUBGRID_SIZE + 1) * SUBGRID_SIZE;
	     ++i)
	{
		for (size_t j = col / SUBGRID_SIZE * SUBGRID_SIZE;
		     j < (col / SUBGRID_SIZE + 1) * SUBGRID_SIZE;
		     ++j)
		{
			if (board[i][j] == num + '0') {
				return false;
			}
		}
	}
	return true;
}

bool solveSudoku(Board &board) {
	for (size_t i = 0; i < SIZE; ++i) {
		for (size_t j = 0; j < SIZE; ++j) {
			if (board[i][j] == '.') {
				for (size_t k = 1; k <= SIZE; k++) {
					if (is_valid(board, i, j, k)) {
						board[i][j] = k + '0';
						if (not solveSudoku(board)) board[i][j] = '.';
                        else return true;
					}
				}
				return false;
			}
		}
	}
	return true;
}

int main() {
	Board board{
	    {'5', '3', '.', '.', '7', '.', '.', '.', '.'},
	    {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
	    {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
	    {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
	    {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
	    {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
	    {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
	    {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
	    {'.', '.', '.', '.', '8', '.', '.', '7', '9'}
	};
	assert(not is_valid(board, 0, 2, 6));
	assert(not is_valid(board, 3, 6, 6));
	assert(not is_valid(board, 8, 5, 8));
	assert(is_valid(board, 8, 0, 1));
	solveSudoku(board);
	assert(
	    board == Board(
	                 {{'5', '3', '4', '6', '7', '8', '9', '1', '2'},
	                  {'6', '7', '2', '1', '9', '5', '3', '4', '8'},
	                  {'1', '9', '8', '3', '4', '2', '5', '6', '7'},
	                  {'8', '5', '9', '7', '6', '1', '4', '2', '3'},
	                  {'4', '2', '6', '8', '5', '3', '7', '9', '1'},
	                  {'7', '1', '3', '9', '2', '4', '8', '5', '6'},
	                  {'9', '6', '1', '5', '3', '7', '2', '8', '4'},
	                  {'2', '8', '7', '4', '1', '9', '6', '3', '5'},
	                  {'3', '4', '5', '2', '8', '6', '1', '7', '9'}}
	             )
	);
	return 0;
}
