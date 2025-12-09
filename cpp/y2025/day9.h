#pragma once

#include "../parser.h"
#include "../views.h"

#include <ranges>

namespace aoc::day9
{
    inline auto solve(const bool is_test)
    {
        auto file = load_input_file(2025, 9, is_test);

        // Parse and fill red tiles
        auto red_tiles = std::vector<Vector2D<>>();
        auto max_tile = Vector2D(std::numeric_limits<int64_t>::min(), std::numeric_limits<int64_t>::min());
        auto min_tile = Vector2D(std::numeric_limits<int64_t>::max(), std::numeric_limits<int64_t>::max());

        auto line = std::string();
        while (std::getline(file, line))
        {
            auto pos = line.begin();

            const auto x = consume_int<int64_t>(pos, line.end());
            consume_char(pos, line.end());
            const auto y = consume_int<int64_t>(pos, line.end());
            const auto tile = red_tiles.emplace_back(x, y);

            max_tile = max_tile.max(tile);
            min_tile = min_tile.min(tile);
        }

        // Build a matrix with red and green tiles for part 2
        const auto matrix_size = max_tile - min_tile + Vector2D(1ll, 1ll);
        auto matrix = std::vector(matrix_size.x * matrix_size.y, '.');
        auto green_tiles = std::vector<Vector2D<>>();

        const auto get_tile = [&](const Vector2D<>& pos)
        {
            const auto index = coord_to_index(pos - min_tile, matrix_size);
            return matrix[index];
        };

        const auto set_tile = [&](const Vector2D<>& pos, const char v)
        {
            const auto index = coord_to_index(pos - min_tile, matrix_size);
            if (matrix[index] == '.')
            {
                matrix[index] = v;
                return true;
            }

            return false;
        };

        // Add red tiles and green tiles between them
        for (auto i = 0ull; i < red_tiles.size(); ++i)
        {
            const auto& tile = red_tiles[i];
            set_tile(tile, '#');

            const auto& next_tile = red_tiles[(i + 1) % red_tiles.size()];
            const auto delta = next_tile - tile;
            const auto dist = delta.abs_max();
            const auto dir = delta.normalized();
            for (auto d = 1ll; d < dist; ++d)
            {
                const auto green_tile = tile + dir * d;
                if (set_tile(green_tile, 'X'))
                {
                    green_tiles.push_back(green_tile);
                }
            }
        }

        // Fill space inside the loop with green tiles
        for (const auto& tile : green_tiles)
        {
            // Search for the next tile in the row
            auto next_tile_x = std::optional<int64_t>();
            for (auto x = tile.x + 2; x <= max_tile.x; ++x)
            {
                const auto& other_tile = get_tile(Vector2D(x, tile.y));
                if (other_tile != '.')
                {
                    next_tile_x = x;
                    break;
                }
            }

            if (!next_tile_x)
            {
                continue;
            }

            // Fill
            for (auto x = tile.x + 1; x < *next_tile_x; ++x)
            {
                set_tile(Vector2D(x, tile.y), 'X');
            }
        }

        const auto is_rect_inside = [&](const Vector2D<>& start, const Vector2D<>& end)
        {
            for (auto x = start.x + 1; x < end.x; ++x)
            {
                if (get_tile(Vector2D{x, start.y}) == '.' || get_tile(Vector2D{x, end.y}) == '.')
                {
                    return false;
                }
            }
            for (auto y = start.y + 1; y < end.y; ++y)
            {
                if (get_tile(Vector2D{start.x, y}) == '.' || get_tile(Vector2D{end.x, y}) == '.')
                {
                    return false;
                }
            }

            return true;
        };

        const auto find_largest_rect = [&](const bool allow_outside)
        {
            auto result = 0ull;
            for (auto i = 0ull; i < red_tiles.size() - 1; ++i)
            {
                const auto& tile = red_tiles[i];
                for (auto j = i + 1; j < red_tiles.size(); ++j)
                {
                    const auto& other_tile = red_tiles[j];

                    if (!allow_outside)
                    {
                        // Check if the entire rect is inside the loop
                        const auto start = tile.min(other_tile);
                        const auto end = tile.max(other_tile);
                        if (!is_rect_inside(start, end))
                        {
                            continue;
                        }
                    }

                    const auto rect_size = tile - other_tile;
                    const auto area = static_cast<uint64_t>(std::abs(rect_size.x) + 1) * static_cast<uint64_t>(std::abs(rect_size.y) + 1);
                    result = std::max(result, area);
                }
            }

            return result;
        };

        // Calculate results
        const auto result_p1 = find_largest_rect(true);
        const auto result_p2 = find_largest_rect(false);

        return std::make_tuple(result_p1, result_p2);
    }
} // namespace aoc::day9
