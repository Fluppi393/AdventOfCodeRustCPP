#pragma once

#include "../parser.h"
#include "../views.h"

#include <ranges>

namespace aoc::day10
{
    inline auto count_presses(const std::vector<bool>& indicator_requirements, const std::vector<std::vector<uint64_t>>& buttons)
    {
        // Each button can either be pressed or not, order doesn't matter
        const auto num_combinations = 1ull << buttons.size();

        auto result = std::numeric_limits<uint64_t>::max();
        for (auto mask = 0ull; mask < num_combinations; ++mask)
        {
            auto indicators = std::vector(indicator_requirements.size(), false);

            // Push buttons
            auto num_presses = 0ull;
            for (auto button_idx = 0ull; button_idx < buttons.size(); ++button_idx)
            {
                if ((mask & 1 << button_idx) != 0)
                {
                    ++num_presses;

                    // Flips buttons
                    for (const auto indicator_idx : buttons[button_idx])
                    {
                        indicators[indicator_idx] = !indicators[indicator_idx];
                    }
                }
            }

            if (indicators == indicator_requirements)
            {
                result = std::min(result, num_presses);
            }
        }

        return result;
    }

    inline uint64_t count_joltages_rec(const std::vector<uint64_t>& joltage_requirements, const std::vector<std::vector<uint64_t>>& buttons, std::vector<uint64_t> state, const size_t index)
    {
        auto presses = 0ull;
        auto result = std::numeric_limits<uint64_t>::max();
        while (true)
        {
            // Recurse
            if (index < buttons.size() - 1)
            {
                const auto rec_result = count_joltages_rec(joltage_requirements, buttons, state, index + 1);
                if (rec_result != std::numeric_limits<uint64_t>::max())
                {
                    result = std::min(result, presses + rec_result);
                }
            }

            for (const auto i : buttons[index])
            {
                ++state[i];

                // Check if we exceeded the requirements
                if (state[i] > joltage_requirements[i])
                {
                    return result;
                }
            }

            ++presses;

            // Check if we are done
            if (state == joltage_requirements)
            {
                return presses;
            }
        }
    }

    inline auto count_joltages(const std::vector<uint64_t>& joltage_requirements, const std::vector<std::vector<uint64_t>>& buttons) { return count_joltages_rec(joltage_requirements, buttons, std::vector(joltage_requirements.size(), 0ull), 0ull); }

    inline auto solve(const bool is_test)
    {
        auto file = load_input_file(2025, 10, is_test);

        auto result_p1 = 0ull;
        auto result_p2 = 0ull;

        auto line = std::string();
        while (std::getline(file, line))
        {
            auto pos = line.begin();

            // Indicator lights
            consume_char(pos, line.end());
            const auto indicators = consume_string(pos, line.end(), [](const auto c) { return c == '.' || c == '#'; }) | std::views::transform([](const auto c) { return c == '#'; }) | std::ranges::to<std::vector>();
            consume_char(pos, line.end());

            // Space
            consume_whitespace(pos, line.end());

            // Button wirings
            auto button_wirings = std::vector<std::vector<uint64_t>>();
            while (consume_char(pos, line.end(), '('))
            {
                auto& wirings = button_wirings.emplace_back();
                while (const auto i = consume_int<uint64_t>(pos, line.end()))
                {
                    wirings.push_back(*i);

                    if (!consume_char(pos, line.end(), ','))
                    {
                        break;
                    }
                }

                // Closing bracket
                consume_char(pos, line.end());

                // Space
                consume_whitespace(pos, line.end());
            }

            // Joltage requirements
            consume_char(pos, line.end());
            auto joltage_reqs = std::vector<uint64_t>();
            while (const auto i = consume_int<uint64_t>(pos, line.end()))
            {
                joltage_reqs.push_back(*i);

                if (!consume_char(pos, line.end(), ','))
                {
                    break;
                }
            }
            consume_char(pos, line.end());

            result_p1 += count_presses(indicators, button_wirings);
            result_p2 += count_joltages(joltage_reqs, button_wirings);
        }

        return std::make_tuple(result_p1, result_p2);
    }
} // namespace aoc::day10
