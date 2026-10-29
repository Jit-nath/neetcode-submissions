class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        islands = 0
        max_area = 0

        row_boundary = len(grid)
        col_boundary = len(grid[0])

        def areaFinder(row: int, col: int) -> int:

            # Check boundaries
            if row < 0 or row >= row_boundary:
                return 0

            if col < 0 or col >= col_boundary:
                return 0

            # Water or already visited
            if grid[row][col] == 0:
                return 0

            # Mark current cell as visited
            grid[row][col] = 0

            # Count current cell
            area = 1

            # Explore four directions
            area += areaFinder(row - 1, col)  # Up
            area += areaFinder(row + 1, col)  # Down
            area += areaFinder(row, col - 1)  # Left
            area += areaFinder(row, col + 1)  # Right

            return area

        if not grid or not grid[0]:
            return 0

        for row in range(len(grid)):
            for col in range(len(grid[row])):
                if grid[row][col] == 1:
                    islands += 1

                    area = areaFinder(row, col)

                    if max_area < area:
                        max_area = area

        return max_area
