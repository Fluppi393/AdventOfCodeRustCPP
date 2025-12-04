#pragma once

#include <vector>
#include <ranges>

#include "utility.h"
#include "vector_2d.h"

namespace aoc::views
{
    // Builds a view for the <= 8 neighbours of an element in a matrix
    template <class T>
    auto neighbours(const std::vector<T>& matrix, const Vector2D<int>& size, const Vector2D<int>& origin)
    {
        return std::views::iota(-1, 2)
            | std::views::transform([](const int dx)
            {
                return std::views::iota(-1, 2)
                    | std::views::transform([dx](const int dy)
                    {
                        return Vector2D{dx, dy};
                    });
            })
            | std::views::join
            | std::views::filter([](const Vector2D<int>& off)
            {
                return !(off.x == 0 && off.y == 0);
            })
            | std::views::transform([&](const Vector2D<int>& off)
            {
                return origin + off;
            })
            | std::views::filter([&](const auto& coord)
            {
                return is_valid_coord(coord, size);
            })
            | std::views::transform([&](const auto& coord)
            {
                const auto index = coord_to_index(coord, size);
                return matrix[index];
            });
    }
};
