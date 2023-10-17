// This problem is poorly explained
// This code is not optimized
// This code is a first version of a try out algorithm
// then end.
class Solution {
    // [1,3,-1,-1,-1], [-1,2,4,-1,-1]
    // todo here we can keep a map of parent_index to avoid doing the isAssociated again
    func isAssociated(_ n: Int, _ into: [Int], _ other: [Int]) -> Bool {
        print(n)

        if n == 0 && other[0] == 1 {
            return true
        }
        
        let parent_index = into.firstIndex(of: n)
        print(parent_index)
        if let parent_index {
            if parent_index == 0 {
        print("p")
                return true
            } else {
        print("d")
                return isAssociated(parent_index, other, into)
            }
        } 
        
        return false
    }
    
    func validateBinaryTreeNodes(_ n: Int, _ leftChild: [Int], _ rightChild: [Int]) -> Bool {

        var reverse = n - 1
        for i in 0...(n-1) {
            // Child as two parents
            
            if leftChild.contains(i) && rightChild.contains(i) {
                print("aie")
                return false
            }

            if !isAssociated(reverse, leftChild.contains(reverse) ? leftChild : rightChild, leftChild.contains(reverse) ? rightChild : leftChild) {
                return false
            }
            

            reverse -= 1
        }
        return true
    }
}   

let solution = Solution()
// let result = solution.validateBinaryTreeNodes(4, [1,-1, 3, 1], [2,-1,-1, -1])
let result = solution.validateBinaryTreeNodes(5, [1,3,-1,-1,-1], [-1,2,4,-1,-1])
// let result = solution.validateBinaryTreeNodes(2, [1, 0], [-1,-1])

print(result)