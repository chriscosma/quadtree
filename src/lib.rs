use pyo3::prelude::*;

#[pyclass]
struct Point {
    x: f32,
    y: f32,
    data: PyObject
}

#[pymethods]
impl Point {
    #[new]
    fn __new__(x: f32, y: f32, data: PyObject) -> Self {
        return Self {
            x, y, data
        }
    }

    fn __repr__(&self) -> String {
        std::format!("Point(x={}, y={}, data={})", self.x, self.y, self.data)
    }

    fn __str__(&self) -> String {
        return self.__repr__()
    }

    #[getter]
    fn x(&self) -> f32 {
        self.x
    }

    #[getter]
    fn y(&self) -> f32 {
        self.y
    }

    #[getter]
    fn data(&self) -> &PyObject {
        &self.data
    }

    fn distance_to(&self, other: &Point) -> f32 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;

        f32::sqrt(x_diff*x_diff + y_diff*y_diff)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn quadtree(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    Ok(())
}