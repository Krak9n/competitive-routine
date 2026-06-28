#include <iostream>
#include <unordered_map>
#include <sstream>
#include <string>

// output the missing number
int 
main() {
    unsigned int n{};
    std::string temp{};
    std::unordered_map<int, int> given; 
    // while getting the number:
    // - insert it in the map
    // - rearange the map so that it goes from the smallest to the biggest,
    // rearenge the value <key, value> -> key is the given, and value will be
    // the position
    // 

    std::cin >> n; // dimension

    while(std::getline(std::cin, temp)) {
        std::istringstream t(temp);
        t >> n >> std::ws;
    }

    return 0;
}
