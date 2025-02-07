"""Core functionality for ECC operations"""
from typing import Optional

try:
    from ._rust import PyCurve as _PyCurve, PyPoint as _PyPoint  # type: ignore
except ImportError:
    from warnings import warn
    warn("Rust module not found. Using mock objects for development.")
    
    class PyCurve:
        def __init__(self, *args, **kwargs): pass
        def point(self, *args, **kwargs): return _PyPoint()
    
    class PyPoint:
        def __init__(self, *args, **kwargs): pass
        @property
        def x(self): return 0
        @property
        def y(self): return 0

class Curve:
    """Elliptic curve over a finite field"""
    def __init__(self, a: int, b: int, prime: int):
        self._curve = _PyCurve(a, b, prime)

    def point(self, x: int, y: int) -> 'Point':
        return Point(self._curve.point(x, y))

    def __repr__(self) -> str:
        return f"Curve(a={self._curve.a}, b={self._curve.b}, p={self._curve.prime})"

class Point:
    """Point on an elliptic curve"""
    def __init__(self, point: _PyPoint):
        self._point = point

    @property
    def x(self) -> Optional[int]:
        return self._point.x

    @property
    def y(self) -> Optional[int]:
        return self._point.y

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Point):
            return NotImplemented
        return self._point == other._point
