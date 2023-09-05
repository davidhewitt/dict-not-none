from dict_not_none import dict_not_none, dict_not_none_pyo3, dict_not_none_baremetal


def test_dict_not_none_py(benchmark):
    assert benchmark(dict_not_none, a=1, b=None, c=3, d=None, e=5, f=None) == {
        "a": 1,
        "c": 3,
        "e": 5,
    }


def test_dict_not_none_pyo3(benchmark):
    assert benchmark(dict_not_none_pyo3, a=1, b=None, c=3, d=None, e=5, f=None) == {
        "a": 1,
        "c": 3,
        "e": 5,
    }


def test_dict_not_none_baremetal(benchmark):
    assert benchmark(
        dict_not_none_baremetal, a=1, b=None, c=3, d=None, e=5, f=None
    ) == {
        "a": 1,
        "c": 3,
        "e": 5,
    }
