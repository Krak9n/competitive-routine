#include <iostream>

// if n is even -> n / 2
// if n is odd -> n * 3 + 1
// repeated until 1 is reached
int main() {
    unsigned int n{};
    std::cin >> n;
    std::cout << n << " ";

    while (!(n <= 1)) {
        if (!(n % 2 == 0)) { n = n * 3 + 1; }
        else { n >>= 1; }
        std::cout << n << " ";
    }

    return 0;
}
