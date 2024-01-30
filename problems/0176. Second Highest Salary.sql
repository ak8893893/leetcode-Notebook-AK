# 176. Second Highest Salary
# Write your MySQL query statement below

# solution 1 
-- select MAX(salary) as SecondHighestSalary 
-- from Employee
-- WHERE salary < (select MAX(salary) from Employee  )

# solution 2
-- select MAX(salary) as SecondHighestSalary 
-- from Employee
-- WHERE salary NOT IN (select MAX(salary) from Employee  )

# solution 3
-- select (
--     select DISTINCT salary 
--     from Employee
--     ORDER BY salary DESC LIMIT 1 OFFSET 1
-- ) as SecondHighestSalary

# solution 4
-- select IFNULL (
--     (select DISTINCT salary 
--     from Employee
--     ORDER BY salary DESC LIMIT 1 OFFSET 1),
--     null
-- ) as SecondHighestSalary

# solution 5
WITH ranked as (
    SELECT DISTINCT salary,
    DENSE_RANK() OVER (ORDER BY salary DESC) r
    FROM employee
)

select IFNULL (
    (select salary 
    from ranked
    where r = 2 ),
    null
) as SecondHighestSalary

# solution 6
-- WITH RankedSalaries AS (
--     SELECT 
--         salary,
--         RANK() OVER (ORDER BY salary DESC) AS Ranking
--     FROM Employee
--     GROUP BY salary)
-- SELECT 
--     CASE
--         WHEN MAX(Ranking) != 1 THEN salary
--         ELSE NULL
--         END AS SecondHighestSalary
-- FROM RankedSalaries
-- WHERE Ranking = 2
