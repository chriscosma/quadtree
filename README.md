# quadtree

This is a Rust implementation of a quadtree data structure, which can be used to partition two-dimensional space and efficiently perform spatial queries such as searching for all points within a certain area.

This project includes Python bindings using the PyO3 library, allowing the quadtree to be fast and used from Python.
## Usage

To install in the activated Python environment run `pip install quadtree-fast` or run:
```
pip install maturin
maturin develop --release
```

Python usage:
```
from quadtree import Point, QuadTree, Rect

# Create a new region centered around (0,0) with width and height of 100
boundary = Rect(0.0, 0.0, 100.0, 100.0)
# Create quadtree out of that region with 100 maximum points and 0 starting depth
quadtree = QuadTree(boundary, 100, 0)

# Insert some points into the quadtree
point1 = Point(10.0, 10.0)
point2 = Point(20.0, 20.0)
point3 = Point(30.0, 30.0)
point4 = Point(40.0, 40.0)
quadtree.insert(point1)
quadtree.insert(point2)
quadtree.insert(point3)
quadtree.insert(point4)

# Search for all points within rectangle centered at (15, 15) with height and width of 20
search_rect = Rect(15.0, 15.0, 20.0, 20.0)
result = quadtree.query_rect(search_rect)

print(result)  # Output: [Point(x=10.0, y=10.0), Point(x=20.0, y=20.0)]

# Search for all points with circle of radius 20 centered at (0, 0)
result = quadtree.query_radius(0, 0, 20.0)
print(result) # Output: [Point(x=10.0, y=10.0)]
```
