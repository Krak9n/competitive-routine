#include <iostream>
#include <vector>
#include <string>

using u16 = unsigned int;

// recurse the function with the loop until every 
// single instancce has been found
// 1. convert into ascii
// 2. push into string
// 3. traverse through the string
class Solution {
public:
    std::string countAndSay(int n) {
        std::string stores = "11"; // .insert(pos, what to)
        char previous = *stores.begin();
        int occurences{1};
        for (auto i{1uz}; i <= n; ++i) {
            if (stores.at(i) == previous) {
                ++occurences;
            }
            else {
                stores += std::to_string(occurences);
                occurences = 1;
            }
            previous = stores.at(i);

        }
        // then just convert that vector into a big integer, 
        // and then to string, and then reverse the string
        std::reverse(stores.begin(), stores.end());
        return stores;
    }
};

int main() {
    Solution sol;
    sol.countAndSay(5);

    return 0;
}
