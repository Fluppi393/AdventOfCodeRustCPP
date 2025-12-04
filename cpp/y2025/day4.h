#pragma once

#include "../parser.h"
#include "../views.h"

#include <xutility>
#include <ranges>

namespace aoc::day4
{
    inline auto solve(const bool is_test)
    {
        auto parser = Parser(2025, 4, is_test);

        // Build the matrix
        auto [matrix, size] = parser.build_matrix<bool>([](const char c)
        {
            return c == '@';
        });

        auto result_p1 = 0ull;
        auto result_p2 = 0ull;

        // Keep iterating and removing rolls of paper
        for (auto i = 0ull; ; ++i)
        {
            const auto indices_to_remove = std::views::iota(0u, matrix.size()) | std::views::filter([&](const auto index)
            {
                const auto coord = index_to_coord(index, size);
                return matrix[index] && std::ranges::distance(views::neighbours(matrix, size, coord) | std::views::filter(std::identity())) < 4;
            }) | std::ranges::to<std::vector>();

            const auto num_removed = indices_to_remove.size();
            if (num_removed == 0)
            {
                // Couldn't remove more rolls, abort
                break;
            }

            // Mark rolls as removed
            for (const auto index : indices_to_remove)
            {
                matrix[index] = false;
            }

            // Add to the results
            result_p1 += i == 0 ? num_removed : 0;
            result_p2 += num_removed;
        }

        return std::make_tuple(result_p1, result_p2);
    }
}
