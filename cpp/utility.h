#pragma once

#include <iostream>

#include "vector_2d.h"

namespace aoc
{
    inline auto index_to_coord(const int index, const Vector2D<int>& size)
    {
        return Vector2D(index % size.x, index / size.x);
    }

    inline auto coord_to_index(const Vector2D<int>& coord, const Vector2D<int>& size)
    {
        return coord.x + coord.y * size.x;
    }

    inline bool is_valid_coord(const Vector2D<int>& coord, const Vector2D<int>& size)
    {
        return coord.x >= 0 && coord.x < size.x && coord.y >= 0 && coord.y < size.y;
    }

    template <typename T>
    void assert_result(const T& expected, const T& actual)
    {
        if (expected != actual)
        {
            std::cout << std::format("Expected {}, got {}\n", expected, actual);
        }
    }

    template <typename T1, typename T2>
    void assert_result(const std::tuple<T1, T2>& expected, const std::tuple<T1, T2>& actual)
    {
        if (expected != actual)
        {
            std::cout << std::format("Expected ({},{}), got ({},{})\n", std::get<0>(expected), std::get<1>(expected), std::get<0>(actual), std::get<1>(actual));
        }
    }
}
