#include <cstdint>
#include <string>
#include <iostream>
#include <vector>

class Solution {
public:
    // just traverse through the string and return 
    // the number of ways the balloon can be created
    int maxNumberOfBalloons(std::string text) {
        std::vector<std::string> found{};
        std::string *temp = &text; 
        for (size_t i{}; i < text.size(); ++i) {
            // if not found then clear it from the copy 
            if ((target.find(text[i]) == std::string::npos)) {
                temp->erase(text.begin() + (i));
            }
        }

        found.push_back(rearenge(*temp));

        //if (temp->size() >= 7 ) return (temp->size() + 1) / 7;
        //else 
        return 0;
    }

    std::string rearenge(std::string input) {
    }
private:
    std::string target = "balloon";
};

int main() {
    Solution s1;
    std::cout << s1.maxNumberOfBalloons("nlaebolko") << "\n";
    std::cout << s1.maxNumberOfBalloons("leetcode") << "\n";
    std::cout << s1.maxNumberOfBalloons("loonbalxballpoon") << "\n";

    return 0;
}
