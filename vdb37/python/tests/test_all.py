import pytest
import vdb37


def test_sum_as_string():
    assert vdb37.sum_as_string(1, 1) == "2"
