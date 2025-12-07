#pragma once

#include <numeric>

#include "../parser.h"
#include "../views.h"

#include <xutility>
#include <ranges>

namespace aoc::day6
{
    auto do_math(auto& range, const char op)
    {
        switch (op)
        {
        case '+':
            return std::accumulate(range.begin(), range.end(), 0ull, std::plus());
        case '*':
            return std::accumulate(range.begin(), range.end(), 1ull, std::multiplies());
        default:
            assert(false);
            return 0ull;
        }
    }

    inline auto solve(const bool is_test)
    {
        auto file = load_input_file(2025, 6, is_test);

        // Build a matrix of operants and a vector of operators
        auto number_lines = std::vector<std::string>();
        auto ops_line = std::string();

        auto line = std::string();
        while (std::getline(file, line))
        {
            if (file.eof())
            {
                ops_line = line;
            }
            else
            {
                number_lines.push_back(line);
            }
        }

        // Calculate part 1
        auto result_p1 = 0ull;
        {
            // Get all operators from the last row
            const auto ops = ops_line | std::views::filter([](const auto c) { return !std::isspace(c); }) | std::ranges::to<std::vector>();

            // 1D matrix of the numbers in all rows and columns
            const auto numbers = number_lines | std::views::transform([](const auto& line)
            {
                return std::ranges::subrange(line.begin(), line.end())
                    | std::views::split(' ')
                    | std::views::filter([](auto&& s) { return s.begin() != s.end(); })
                    | std::views::transform([](auto&& s) { return std::stoull(std::string(s.begin(), s.end())); });
            }) | std::views::join | std::ranges::to<std::vector>();

            const auto num_columns = ops.size();

            // Calculate the results for all columns
            const auto column_results = std::views::iota(0ull, num_columns) | std::views::transform([&](const auto column)
            {
                auto columns = std::views::iota(0ull, number_lines.size()) | std::views::transform([&](const auto line_idx)
                {
                    const auto index = column + line_idx * num_columns;
                    return numbers[index];
                });

                return do_math(columns, ops[column]);
            });

            // Accumulate results
            result_p1 = std::accumulate(column_results.begin(), column_results.end(), 0ull);
        }

        // Calculate part 2
        auto result_p2 = 0ull;
        {
            for (auto op_it = ops_line.begin(); op_it < ops_line.end();)
            {
                // Find the next operator
                const auto next_op_it = std::find_if(std::next(op_it), ops_line.end(), [](const char c)
                {
                    return !std::isspace(c);
                });

                const auto num_columns = static_cast<unsigned long long>(next_op_it == ops_line.end() ? next_op_it - op_it : next_op_it - op_it - 1);

                // Generate numbers from columns
                const auto column_offset = op_it - ops_line.begin();
                const auto numbers = std::views::iota(0ull, num_columns) | std::views::transform([&](const auto column_idx)
                {
                    // Get a vertical slice from the lines and strip spaces
                    auto chars = number_lines | std::views::transform([&](const auto& s) { return s[column_idx + column_offset]; }) | std::views::filter([](const char c)
                    {
                        return !std::isspace(c);
                    });

                    // Convert to a string and then an integer
                    const auto number_str = std::string(chars.begin(), chars.end());
                    return std::stoull(number_str);
                });

                result_p2 += do_math(numbers, *op_it);

                op_it = next_op_it;
            }
        }

        return std::make_tuple(result_p1, result_p2);
    }
}
