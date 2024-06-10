import java.util.Stack;
/**
 * https://leetcode.com/problems/decode-string/
 * @date 2020-08-05
 */
public class DecodeString {
    private static String input;
    public static void main(String[] args) {
        Recursive recursive = new Recursive();
        Iterative iterative = new Iterative();

        input = "d2[c10[a]b]";
        assert recursive.decodeString(input).equals("dcaaaaaaaaaabcaaaaaaaaaab");
        assert iterative.decodeString(input).equals("dcaaaaaaaaaabcaaaaaaaaaab");

        input = "abc3[cd]xyz";
        assert recursive.decodeString(input).equals("abccdcdcdxyz");
        assert iterative.decodeString(input).equals("abccdcdcdxyz");
    }
}

class Iterative {
    public String decodeString(String string) {
        Stack<Character> stack = new Stack<>();
        for (char character: string.toCharArray()) {
            stack.add(character);
            if (character == ']') {
                Character ch = stack.pop();
                String sub = "";
                while (ch != '[') {
                    ch = stack.pop();
                    if (ch != '[') sub = ch + sub;
                }

                ch = stack.pop();
                String num = ch.toString();
                while (!stack.isEmpty() && Character.isDigit(stack.peek())) {
                    ch = stack.pop();
                    if (ch != '[') num = ch + num;
                    else stack.add(ch);
                }
                String repeat = sub.repeat(Integer.parseInt(num));
                for (int i = 0; i < repeat.length(); i++) stack.add(repeat.charAt(i));
            }
        }
        String result = "";
        while (!stack.isEmpty()) {
            Character ch = stack.pop();
            result = ch + result;
        }
        return result;
	}
}

class Recursive {
    public String decodeString(String string) {
		if (!string.contains("[") && !string.contains("]")) {
            return string;
        }

        int open = string.indexOf("[");
        int close = string.lastIndexOf("]");

        int total = 0;
        int index = open - 1;
        for (int multiplier = 1; index >= 0 && Character.isDigit(string.charAt(index)); index--, multiplier *= 10) {
            total += (string.charAt(index) - '0') * multiplier;
        }

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < total; i++) sb.append(decodeString(string.substring(open + 1, close)));

        StringBuilder result = new StringBuilder();
        result.append(string.substring(0, index + 1));
        result.append(sb.toString());
        result.append(string.substring(close + 1));
        return result.toString();
    }
}
