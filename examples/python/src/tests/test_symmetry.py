from symmetry import symmetric

def test_happy_path():
    assert symmetric("abc") == "abcba"