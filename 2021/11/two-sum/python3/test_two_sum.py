#!/usr/bin/env python3
"""
https://leetcode.com/problems/two-sum/

Author: Victor Payno (https://github.com/vpayno)

Tests
"""

import os.path
import subprocess
import sys
from typing import List

import pytest

# Our Project
from two_sum import two_sum

integration_test_data = [
    ("0 1 2 3 4 5", "7", "2 + 5 = 7"),
    ("0 1 2 3 4 5", "11", "No solution found."),
]

unit_test_data = [
    ([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 11, [2, 9]),
    ([4, 7, 9, 14, 18, 23], 21, [1, 3]),
    ([3, 3], 6, [0, 1]),
    ([1, 2, 3], 5, [1, 2]),
    ([1, 3, 5], 2, []),
    ([], 0, []),
]


def test_class_init():
    """ the things we test for 100% coverage """
    program: two_sum.Solution = two_sum.Solution(debug=True)
    assert program.two_sum([1, 2, 3, 4], 5) == [0, 3]
    assert program.two_sum([], 0) == []


def test_class_str():
    """ test for the __str__ method that was added because pylint loves to complain """
    assert two_sum.Solution().__str__() == "Solution"


@pytest.mark.parametrize("nums,target,expected", unit_test_data)
def test_method(nums: List[int], target: int, expected: List[int]):
    """ Runs the two_sum class method against all of our test data. """
    assert two_sum.Solution().two_sum(nums, target) == expected


@pytest.mark.parametrize("nums,target,expected", integration_test_data)
def test_script(nums: str, target: str, expected: str):
    """ Runs the two_sum script against all of our test data. """
    process = subprocess.run(
        [
            sys.executable,
            os.path.join(os.path.dirname("two_sum/two_sum.py"), "two_sum.py"),
            nums,
            target,
        ],
        check=False,
        stdout=subprocess.PIPE,
    )
    print(f"{process.stdout.decode('utf-8')} == {expected}")
    assert expected in process.stdout.decode("utf-8")
