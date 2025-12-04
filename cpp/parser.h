#pragma once

#include <assert.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string>
#include <optional>

#include "vector_2d.h"

namespace aoc
{
    class Parser
    {
        std::fstream file_;
        std::string line_;
        std::string::const_iterator line_it_;

    public:
        explicit Parser(const int year, const int day, const bool is_test)
        {
            const auto filename =
                std::filesystem::current_path().parent_path() /
                std::format("input/{}/day{}{}.txt", year, day, is_test ? "_test" : "");

            assert(std::filesystem::exists(filename));

            file_.open(filename);
        }

        ~Parser() { file_.close(); }

        bool next_line()
        {
            if (std::getline(file_, line_))
            {
                line_it_ = line_.begin();
                return true;
            }

            return false;
        }

        const auto& get_line() const { return line_; }

        bool next_char()
        {
            ++line_it_;
            return !line_end();
        }

        // Whether the entire file has been consumed
        bool file_end() const
        {
            return file_.eof();
        }

        // Whether the line has been consumed
        bool line_end() const { return line_it_ == line_.end(); }

        // Returns the index of the current character within the line
        auto char_index() const { return line_it_ - line_.begin(); }

        // Returns the number of characters left in the line
        auto num_chars_left() const { return line_.end() - line_it_; }

        // Consumes the next character and returns it if available
        std::optional<char> consume_char(auto&& predicate = default_predicate)
        {
            const auto c = peek_char();
            if (c && predicate(c.value()))
            {
                next_char();
                return c;
            }

            return std::nullopt;
        }

        auto peek_char() const
        {
            return line_end() ? std::nullopt : std::optional(*line_it_);
        }

        // Builds and consumes a string until the predicate returns false
        auto consume_string(auto&& predicate = default_predicate)
        {
            const auto start = line_it_;
            while (const auto c = consume_char(predicate))
            {
            }
            return std::string_view(start, line_it_);
        }

        // Builds a matrix by converting all characters
        template <typename T>
        auto build_matrix(auto&& transform)
        {
            next_line();

            auto m = std::vector<T>();

            auto max_coord = Vector2D<int>();
            do
            {
                m.reserve(m.size() + get_line().length());
                max_coord.x = 0;

                while (const auto c = consume_char(default_predicate))
                {
                    m.push_back(transform(c.value()));
                    ++max_coord.x;
                }

                ++max_coord.y;
            }
            while (next_line());

            return std::make_tuple(m, max_coord);
        }

    private:
        static bool default_predicate(const char)
        {
            return true;
        }
    };
}
