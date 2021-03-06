#!/usr/bin/env ruby
# typed: true
# frozen_string_literal: true

# https://leetcode.com/problems/two-sum/
#
# Author: Victor Payno (https://github.com/vpayno)

class Solution
  # @return {void}
  def initialize()
    @debug = false
  end

  # @param {::Array[Integer]} nums
  # @param {Integer} target
  # @return {::Array[Integer]}
  def two_sum(nums, target)
    nums_length = nums.length

    hashmap = {}

    for i in 0..(nums_length - 1)
      hashmap[nums[i]] = i
    end

    for i in 0..(nums_length - 1)
      compliment = target - nums[i]

      if hashmap.key?(compliment) && hashmap[compliment] != i
        return [
                 i,
                 hashmap[compliment],
               ]
      end
    end

    []
  end # two_sum()

  # @param {::Array[String]} args
  # @return {Integer}
  def main(args)
    search_candidates = if args[0].nil? || args[0].empty?
        []
      else
        args[0].split(" ").reject(&:empty?).map(&:to_i)
      end

    search_target = if args[1].nil? || args[1].empty?
        0
      else
        args[1].to_i
      end

    # uncovered
    $stderr.puts("ERROR: the search candidate list needs at least two numbers in it.\n") if search_candidates.length < 2

    puts format("Search Candidates: %s\n", search_candidates)
    puts format("Searching for sum of %s.\n", search_target)
    puts

    # uncovered
    result = self.two_sum(search_candidates, search_target) if search_candidates && search_target

    if result.length == 2
      puts format("%<num1>s + %<num2>s = %<sum>s\n", num1: search_candidates[result[0]], num2: search_candidates[result[1]], sum: search_target)
      return 0
    else
      print("No solution found.\n")
      return 1
    end
  end # main()
end # class Solution

if $PROGRAM_NAME == __FILE__
  app = Solution.new()  # uncovered
  status = app.main(ARGV)  # uncovered
  exit(status)  # uncovered
end
