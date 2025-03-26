"""
https://leetcode.com/problems/simplify-path/
"""


def simplifyPath(path: str) -> str:
    paths = [
        directory for directory in path.split("/") if directory and directory != "."
    ]
    stack: list[str] = []
    for directory in paths:
        if directory == ".." and len(stack):
            stack.pop()
        elif directory != "..":
            stack.append(directory)
    return "/" + "/".join(stack)


if __name__ == "__main__":
    assert simplifyPath("/home/") == "/home"
    assert simplifyPath("/home//foo/") == "/home/foo"
    assert simplifyPath("/home/user/Documents/../Pictures") == "/home/user/Pictures"
    assert simplifyPath("/...") == "/..."
    assert simplifyPath("/") == "/"
    assert simplifyPath("/../") == "/"
    assert simplifyPath("/.../a/../b/c/../d/./") == "/.../b/d"
    assert simplifyPath("/a//b////c/d//././/..") == "/a/b/c"
