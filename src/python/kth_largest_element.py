from math import ceil, floor


def swap(a, b, nums):
    swap = nums[a]
    nums[a] = nums[b]
    nums[b] = swap


def heapify(k: int, nums: list[int]):
    if k > (len(nums) - 1) // 2:
        return
    left_node = k * 2 + 1
    right_node = k * 2 + 2
    if nums[left_node] > nums[k]:
        next = left_node
    elif nums[right_node] > nums[k]:
        next = right_node
    else:
        next = None

    if next:
        heapify(next, nums)


def make_heap(nums):
    for i in range(len(nums) // 2):
        heapify(i, nums)


class MyHeapQ:
    def __init__(self, nums: list[int]):
        self.heap = []
        for i in nums:
            self.heappush(i)

    def heappush(self, n: int):
        self.heap.insert(0, n)
        self.heapify(0)

    def heappop(self) -> int:
        n = self.heap[0]
        self.heap[0] = self.heap[-1]
        self.heap.pop()
        self.heapify(0)
        return n

    def heapify(self, k: int):
        left_node = 2 * k + 1
        right_node = 2 * k + 2
        if left_node < len(self.heap) and self.heap[k] > self.heap[left_node]:
            self.swap(k, left_node)
            self.heapify(left_node)
        elif right_node < len(self.heap) and self.heap[k] > self.heap[right_node]:
            self.swap(k, right_node)
            self.heapify(right_node)

    def swap(self, x: int, y: int):
        s = self.heap[x]
        self.heap[x] = self.heap[y]
        self.heap[y] = s


class Solution:
    def findKthLargest(self, nums: list[int], k: int) -> int:
        heap = nums[:k]
        hq = MyHeapQ(heap)
        for n in nums[k:]:
            if n > hq.heap[0]:
                hq.heappop()
                hq.heappush(n)
        return hq.heappop()
