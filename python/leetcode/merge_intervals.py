class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        if not intervals:
            return []
            
        intervals = sorted(intervals, key=lambda x: x[0])
        ret = [intervals[0]]
        idx = 0
        for interval in intervals[1::]:
            if ret[idx][1] >= interval[0]:
                ret[idx][1] = max(ret[idx][1], interval[1])
                continue           
            ret.append(interval)
            idx += 1

        return ret

if __name__ == '__main__':
    s = Solution()
    assert s.merge([[1,3],[2,6],[8,10],[15,18]]) == [[1,6],[8,10],[15,18]]
    assert s.merge([[1,4],[4,5]]) == [[1,5]]
    assert s.merge([[4,7],[1,4]]) == [[1,7]]
    assert s.merge([[1,4],[2,3]]) == [[1,4]]
