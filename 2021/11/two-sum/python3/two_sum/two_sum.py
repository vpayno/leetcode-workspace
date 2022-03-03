#!/usr/bin/env python3
"""
https://leetcode.com/problems/two-sum/

Author: Victor Payno (https://github.com/vpayno)
"""

import sys
from typing import List


class Solution:
    """TwoSum Solution (https://leetcode.com/problems/two-sum/)

    Given an array of integers nums and an integer target, return indices of the two numbers such
    that they add up to target.
    You may assume that each input would have exactly one solution, and you may not use the same
    element twice.
    You can return the answer in any order.
    Only one valid answer exists.
    """

    def __init__(self, debug: bool = False):
        self.debug = debug

    def two_sum(self, nums: List[int], target: int) -> List[int]:
        """Finds two numbers in a list who's sum equals target."""

        compliment: int
        key: int
        value: int

        hashmap = {}

        for key, value in enumerate(nums):
            hashmap[nums[key]] = key

        for key, value in enumerate(nums):
            compliment = target - value

            if compliment in hashmap and hashmap[compliment] != key:
                print("Found solution.")
                return [key, hashmap[compliment]]

        if self.debug:
            print("No solution.")

        return []

    def __str__(self):
        return self.__class__.__name__


if __name__ == "__main__":  # pragma: no cover
    Exercise = Solution()

    search_candidates: List[int] = ([] if sys.argv[1] is None else
                                    [int(_) for _ in sys.argv[1].split(" ")])

    search_target: int = 0 if sys.argv[2] is None else int(sys.argv[2])

    result: List[int] = []

    if len(search_candidates) < 2:
        print(
            "ERROR: the search candidate list needs at least two numbers in it."
        )

    print(f"Search Candidates: {search_candidates}")
    print(f"Searching for sum of {search_target}.")
    print()

    if search_candidates and search_target:
        result = Exercise.two_sum(search_candidates, search_target)

    if len(result) == 2:
        print(f"{result[0]} + {result[1]} = {search_target}\n")
    else:
        print("No solution found.\n")
