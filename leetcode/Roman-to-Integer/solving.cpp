#include <iostream>
#include <string>
#include <map>

class Solution {
public:
    // iterate through this particular character 
    // and move its value to the vector
    // then return the sum of all integers 
    // from the vector and thats fucking it
    int romanToInt(std::string s) {
        // find the position of the map
        int to_return{};
        for (size_t i{}; i < s.size(); ++i) {
            if (auto ss = romans.find(s.at(i)); ss != romans.end()) {
                std::cout << to_return << "\n";
                if (i < s.size() - 1) {
                    if (s.at(i) == 'I' && (s.at(i + 1) == ('V' | 'X'))) {
                        to_return -= 1; 
                        continue;
                    }
                    else if (s.at(i) == 'X' && (s.at(i + 1) == ('L' | 'C'))) {
                        to_return -= 10;
                        continue;
                    }
                    else if (s.at(i) == 'C' && (s.at(i + 1) == ('D' | 'M'))) {
                        to_return -= 100;
                        continue;
                    }
                }
                to_return += (ss->second);
            }
        }
        return to_return; 
    }
private:
    const std::map<char, int> romans{
        {'I', 1}, {'V', 5}, {'X', 10}, {'L', 50}, 
        {'C', 100}, {'D', 500}, {'M', 1000}};
};

int main() {
    Solution s;
    std::cout << s.romanToInt("MCMXCIV") << "\n";

    return 0;
}
