"""
ECC-Client - Rustで書かれたECCライブラリをFFIでPythonから利用するためのラッパーライブラリ
"""
from .core import Curve, Point

__version__ = "0.1.0"
__all__ = ["Curve", "Point"]
