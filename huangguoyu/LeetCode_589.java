/*
// Definition for a Node.
class Node {
    public int val;
    public List<Node> children;

    public Node() {}

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, List<Node> _children) {
        val = _val;
        children = _children;
    }
};
*/
class Solution {
    List<Integer> res = new ArrayList<>();
    public List<Integer> preorder(Node root) {
        if (root == null) return res;
        if (root.children == null || root.children.size() == 0) {
            res.add(root.val);
            return res;
        }
        res.add(root.val);
        for(Node item : root.children) {
            preorder(item);
        }
        return res;
    }
}