#pragma once

namespace aoc
{
    template <typename T = long long>
    struct Vector2D
    {
        T x = 0;
        T y = 0;

        explicit Vector2D(const T x = 0, const T y = 0) : x(x), y(y) {}

        bool operator==(const Vector2D& other) const { return x == other.x && y == other.y; }

        auto operator+(const Vector2D& other) const { return Vector2D{x + other.x, y + other.y}; }

        auto operator-(const Vector2D& other) const { return Vector2D{x - other.x, y - other.y}; }

        auto operator*(const Vector2D& other) const { return Vector2D{x * other.x, y * other.y}; }

        auto operator/(const Vector2D& other) const { return Vector2D{x / other.x, y / other.y}; }

        auto operator*(const T n) const { return Vector2D{x * n, y * n}; }

        auto abs_max() const { return std::max(std::abs(x), std::abs(y)); }

        auto normalized() const { return Vector2D{std::clamp(x, -1ll, 1ll), std::clamp(y, -1ll, 1ll)}; }

        auto min(const Vector2D& other) const { return Vector2D{std::min(x, other.x), std::min(y, other.y)}; }

        auto max(const Vector2D& other) const { return Vector2D{std::max(x, other.x), std::max(y, other.y)}; }

        auto manhatten(const Vector2D& other) const { return std::abs(x - other.x) + std::abs(y - other.y); }
    };
} // namespace aoc
