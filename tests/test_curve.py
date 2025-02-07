from client import Curve, Point

def test_curve_creation():
    curve = Curve(1, 1, 23)
    assert isinstance(curve, Curve)
    
def test_point_creation():
    curve = Curve(1, 1, 23)
    point = curve.point(3, 10)
    assert isinstance(point, Point)
    assert point.x == 3
    assert point.y == 10
