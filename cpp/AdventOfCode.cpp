#include "y2025/day4.h"

int main(int argc, char* argv[])
{
    aoc::assert_result(std::make_tuple(13ull, 43ull), aoc::day4::solve(true));
    aoc::assert_result(std::make_tuple(1445ull, 8317ull), aoc::day4::solve(false));
    return 0;
}
