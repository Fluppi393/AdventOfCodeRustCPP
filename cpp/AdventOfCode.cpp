#include "y2025/day6.h"

int main(int argc, char* argv[])
{
    aoc::assert_result(std::make_tuple(4277556ull, 3263827ull), aoc::day6::solve(true));
    aoc::assert_result(std::make_tuple(4878670269096ull, 8674740488592ull), aoc::day6::solve(false));
    return 0;
}
