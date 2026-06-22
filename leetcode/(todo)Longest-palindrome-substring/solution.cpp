#include <string>
#include <iostream>

class Solution {
public:
    Solution() {}
// how to check for a longest substring
// go from the end. each time checking for it being the palindrome
// if it is then return since it is the longest if 
// else do the pop back until it is the length of 2, if two remaining arent 
// palindrome throw an error 
    std::string longestPalindrome(std::string s) {
        std::string temp = s;
        for (int i = 0; i < s.size(); ++i) {
            while (true) {
                if (temp.at(i) == temp.back()) {
                    break;
                }// palindrome 
                else {
                    temp.pop_back();
                } 
            }
        }
        return temp; // palindrome
    }
};

int main() {
    Solution s;
    std::cout << s.longestPalindrome("babad") << "\n";
    std::cout << s.longestPalindrome("cbbd") << "\n";
    return 0;
}
