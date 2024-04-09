class Solution:
    def longestArithSeqLength(self, nums: List[int]) -> int:
        # This will hold the longest length for each difference at each index
        dp = [{} for _ in range(len(nums))]
        max_length = 1  # Since any single number is an arithmetic sequence of length 1

        for i in range(len(nums)):
            for j in range(i):
                diff = nums[i] - nums[j]  # Calculate the difference between the two numbers
                # If this difference already exists at j, extend that sequence. Otherwise, start a new one with length 2.
                dp[i][diff] = dp[j].get(diff, 1) + 1
                # Update the max length found so far
                max_length = max(max_length, dp[i][diff])

        return max_length
