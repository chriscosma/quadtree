from quadtree import *
import numpy as np
import time

points = np.random.random((70000, 2))

t0 = time.time()

boundary = Rect(0, 0, 360, 180)
qt = QuadTree(
    boundary,
    100,
    0
)

for x, y in points:
    x *= 5
    x -= 2.5
    y *= 5
    y -= 2.5
    point = Point(x, y)
    qt.insert(point)

len(qt)

query_bound = Rect(2, 2, 1, 1)
qt.query(query_bound)

print("%.3f seconds" % (time.time() - t0))