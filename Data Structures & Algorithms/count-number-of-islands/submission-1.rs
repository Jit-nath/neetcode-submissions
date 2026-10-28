
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut islands = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    islands += 1;
                    Self::dfs(&mut grid, r, c);
                }
            }
        }

        islands
    }

    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
        if grid[r][c] != '1' {
            return;
        }

        grid[r][c] = '0';

        if r > 0 {
            Self::dfs(grid, r - 1, c);
        }
        if r + 1 < grid.len() {
            Self::dfs(grid, r + 1, c);
        }
        if c > 0 {
            Self::dfs(grid, r, c - 1);
        }
        if c + 1 < grid[0].len() {
            Self::dfs(grid, r, c + 1);
        }
    }
}
