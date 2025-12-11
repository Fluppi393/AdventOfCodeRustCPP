#include "y2025/day10.h"

int main(int argc, char* argv[])
{
    aoc::assert_result(std::make_tuple(7ull, 33ull), aoc::day10::solve(true));
    aoc::assert_result(std::make_tuple(542ull, 0ull), aoc::day10::solve(false));

    return 0;
}
