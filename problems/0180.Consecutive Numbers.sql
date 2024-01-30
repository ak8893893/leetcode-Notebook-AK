# Write your MySQL query statement below
# CTE solution
-- WITH cte AS (
--     SELECT num,
--     lead(num,1) over() as num1,
--     lead(num,2) over() as num2
--     FROM logs
-- )

-- SELECT DISTINCT num AS ConsecutiveNums 
-- FROM cte 
-- WHERE num=num1 AND num=num2

# SELF JOIN THREE TIMES SOLUTION 1
-- SELECT DISTINCT A.Num AS ConsecutiveNums
-- FROM Logs A 
-- JOIN Logs B ON A.Num=B.Num 
-- JOIN Logs C ON B.Num=C.Num 
-- WHERE A.Id = B.Id-1 AND B.Id = C.Id-1

# SELF JOIN THREE TIMES SOLUTION 2
SELECT DISTINCT A.Num AS ConsecutiveNums
FROM Logs A 
JOIN Logs B ON A.id=B.id-1
JOIN Logs C ON B.id=C.id-1
WHERE A.num = B.num AND B.num = C.num
