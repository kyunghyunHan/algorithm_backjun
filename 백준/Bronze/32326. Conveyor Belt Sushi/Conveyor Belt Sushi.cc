#include <iostream>

int main() {
    int R, G, B;

    // Read the number of plates of each color
    std::cin >> R; // Red plates
    std::cin >> G; // Green plates
    std::cin >> B; // Blue plates

    // Calculate the total cost
    int cost = R * 3 + G * 4 + B * 5;

    // Output the total cost
    std::cout << cost << std::endl;

    return 0;
}
