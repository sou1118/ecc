///
/// # ECC (Elliptic Curve Cryptography)
///
/// このライブラリは、楕円曲線暗号 (ECC) の基本的な機能が実装されています。
///
/// 学習用として実装したため、実際のプロダクション環境で使用することはお控えください。
///
/// # Examples
///
/// ```rust
/// use ecc::{Curve, DiffieHellman};
///
/// // 楕円曲線を作成 (y² = x³ + 7)
/// let curve = Curve::new(0, 7, 223).unwrap();
/// let generator = curve.point(15, 86).unwrap();
///
/// // Diffie-Hellman鍵交換の例
/// let alice = DiffieHellman::new(curve.clone(), generator.clone()).unwrap();
/// let bob = DiffieHellman::new(curve.clone(), generator.clone()).unwrap();
///
/// // 共有鍵を計算
/// let alice_shared = alice.compute_shared_secret(bob.public_key()).unwrap();
/// let bob_shared = bob.compute_shared_secret(alice.public_key()).unwrap();
///
/// assert_eq!(alice_shared.x, bob_shared.x);
/// assert_eq!(alice_shared.y, bob_shared.y);
/// ```
use pyo3::prelude::*;

pub mod curve;
pub mod field;
pub mod point;
pub mod protocols;

use curve::Curve;
use point::Point;

#[pyclass]
#[derive(Clone)]
struct PyCurve {
    inner: Curve,
}

#[pyclass]
#[derive(Clone)]
struct PyPoint {
    inner: Point,
}

#[pymethods]
impl PyCurve {
    #[new]
    fn new(a: i64, b: i64, prime: i64) -> PyResult<Self> {
        Ok(PyCurve {
            inner: Curve::new(a, b, prime)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
        })
    }

    fn point(&self, x: i64, y: i64, py: Python<'_>) -> PyResult<Py<PyPoint>> {
        let point = PyPoint {
            inner: self
                .inner
                .point(x, y)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
        };
        Py::new(py, point)
    }

    fn __repr__(&self) -> String {
        format!(
            "Curve(a={}, b={}, p={})",
            self.inner.a.value(),
            self.inner.b.value(),
            self.inner.prime
        )
    }
}

#[pymethods]
impl PyPoint {
    #[getter]
    fn x(&self, py: Python<'_>) -> PyObject {
        match &self.inner.x {
            Some(x) => x.value().into_pyobject(py).unwrap().into(),
            None => py.None(),
        }
    }

    #[getter]
    fn y(&self, py: Python<'_>) -> PyObject {
        match &self.inner.y {
            Some(y) => y.value().into_pyobject(py).unwrap().into(),
            None => py.None(),
        }
    }

    fn __repr__(&self) -> String {
        match (self.inner.x, self.inner.y) {
            (Some(x), Some(y)) => format!("Point({}, {})", x.value(), y.value()),
            _ => "Point(infinity)".to_string(),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.__repr__())
    }

    fn __eq__(&self, other: PyRef<'_, Self>) -> bool {
        self.inner.x == other.inner.x && self.inner.y == other.inner.y
    }
}

#[pymodule]
fn _rust(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCurve>()?;
    m.add_class::<PyPoint>()?;
    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
