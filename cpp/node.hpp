#pragma once
#include <vector>
using namespace std;
class Node {
public:
	int val;
	vector<Node *> children;

	Node() {}

	Node(int _val) {
		val = _val;
	}

	Node(int _val, vector<Node *> _children) {
		val = _val;
		children = _children;
	}
};
