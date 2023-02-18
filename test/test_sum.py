from maturin_handson import sum_as_string, axpy_py
import numpy as np
def test_add():
    a, b = 1, 2
    c = sum_as_string(a, b)
    assert c == "3"

def test_axpy():
    a = 2
    x = np.array([[1., 1., 1.],
                  [1., 1., 1.],
                  [1., 1., 1.]])

    y = np.array([[1., 1., 1.],
                  [1., 1., 1.],
                  [1., 1., 1.]])

    res = axpy_py(a, x, y)
    assert (res == a*x+y).all()
