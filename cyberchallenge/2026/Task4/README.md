Fourth and last problem on 2026 programming selection.  
  
In a R × C grid (R ≤ 2), numbered with (0, 0) on the top left and (R − 1, C − 1) on the bottom right, a
non-negative integer is written in each cell, with the number on the (0, 0) cell always being 0. A pawn is sitting in the (0, 0) cell and, every second, it can do one of the following actions:  
+ Move horizontally or vertically, but not diagonally (i.e., incrementing or decrementing exactly one of its coordinates, when possible).  
+ Wait in that position without moving.  
  
The pawn wants to visit each cell exactly one time, but it can visit a cell only if the number of seconds passed from the beginning is strictly higher than the number written in it. What is the minimum time required to visit all the cells?  
  
### INPUT  
Line 1: an integer T, number of testcases
Line 2: integer R and C, representing the number of rows and columns
Line 3~: R lines each containing space-separated C integers

### OUTPUT  
T lines, each of them representing the answer to one of the testcases  
  
Subtask 1: 1 ≤ T ≤ 100, R = 1, C ≤ 10^3  
Subtask 2: 1 ≤ T ≤ 100, 1 ≤ R ≤ 2, C ≤ 10^3  
Subtask 3: 1 ≤ T ≤ 100, 1 ≤ R ≤ 2, C ≤ 10^5  

---
### EXPLANATION  
*wip*