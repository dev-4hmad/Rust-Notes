# Struct
- Used to group different fields of related data together.
- All fields exist at the same time.

struct Point {
    x: f64,
    y: f64,
}
# Nested struct:
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
