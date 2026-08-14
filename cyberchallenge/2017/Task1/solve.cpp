#include <algorithm>
#include <iostream>
#include <string>
#include <sstream>
#include <algorithm>
#include <unordered_map>

using u16 = unsigned int;

std::unordered_map<u16, u16> sort(std::unordered_map<u16, u16> to_sort);
// remove N number of sandwiches so that we 
// get a decreasing sequence. and so if the biggest is at the end 
// get just the biggest. fuck them all.
// to check:
// - no one is equal in the hash map
int main() {
    std::unordered_map<u16, u16> inputs{};
    int index{}, temp{}, size{};
    std::string t;
    while(std::getline(std::cin, t)) { // eventually will break
        std::istringstream tmp(t);
        tmp >> temp >> std::ws;
        if (inputs.size() < 1 && size < 1) { size = temp; }
        else {
            inputs.insert({index, temp});
            ++index;
        }
        if (size == inputs.size()) break;
    }

    std::unordered_map<u16, u16> sorted = sort(inputs);
    // solution
    std::cout << "the size is: " << sorted.size() << "\n";
}

// iterate through every element
std::unordered_map<u16, u16> sort(
        std::unordered_map<u16, u16> to_sort) {
    std::unordered_map<u16, u16> copy{};
    std::for_each(
            to_sort.begin(), to_sort.end(),
            [to_sort](std::pair<u16, u16> v) {
                for (size_t i = 0; i < to_sort.size(); ++i) {
                    std::cout << v.first << " -> " << v.second << "\n";
                }
            });
    return copy;
}
