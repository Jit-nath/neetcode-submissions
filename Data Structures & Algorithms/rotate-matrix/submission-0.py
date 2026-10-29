class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:

        if not matrix:
            return
        
        n = len(matrix)
        # transpose the matrix
        for row in range(n):
            for col in range(row, n):
                if not row == col:
                    matrix[row][col], matrix[col][row] = matrix[col][row], matrix[row][col]
        
        # reverse the row
        for row in range(len(matrix)):
            matrix[row].reverse()
        

        