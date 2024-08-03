from kth_largest_element import Solution


def test():
    assert Solution().findKthLargest([3, 2, 1, 5, 6, 4], 2) == 5
    assert Solution().findKthLargest([3, 2, 3, 1, 2, 4, 5, 5, 6], 4) == 4
    assert Solution().findKthLargest([1], 1) == 1
    assert Solution().findKthLargest([-1, 2, 0], 2) == 0
    assert Solution().findKthLargest([5, 2, 4, 1, 3, 6, 0], 4) == 3
    assert (
        Solution().findKthLargest(
            [
                3,
                2,
                3,
                1,
                2,
                4,
                5,
                5,
                6,
                7,
                7,
                8,
                2,
                3,
                1,
                1,
                1,
                10,
                11,
                5,
                6,
                2,
                4,
                7,
                8,
                5,
                6,
            ],
            20,
        )
        == 2
    )

    assert Solution().findKthLargest([7, 6, 5, 4, 3, 2, 1], 5) == 3
    assert Solution().findKthLargest([3, 2, 3, 1, 2, 4, 5, 5, 6], 9) == 1
    