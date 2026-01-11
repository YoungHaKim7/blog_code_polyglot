numbers :: [Int]
numbers = [10, 20, 30, 40]

-- Access by index (not recommended for performance)
value :: Int
value = numbers !! 2   -- 30

squares :: [Int]
squares = map (^2) numbers

evens :: [Int]
evens = filter even numbers

total :: Int
total = sum numbers

main :: IO ()
main = do
    putStrLn "=== Basic List ==="
    print numbers          -- [10,20,30,40]

    putStrLn "\n=== Index Access (use carefully) ==="
    print value            -- 30

    putStrLn "\n=== Map: Transform each element ==="
    print squares          -- [100,400,900,1600]
    print (map (*3) numbers)  -- [30,60,90,120]

    putStrLn "\n=== Filter: Select elements ==="
    print evens            -- [10,20,30,40] (all are even)
    print (filter (>25) numbers) -- [30,40]

    putStrLn "\n=== Fold: Reduce to single value ==="
    print total            -- 100
    print (product numbers) -- 2400000
    print (foldl (+) 0 numbers) -- 100
    print (foldr (-) 0 numbers) -- -20

    putStrLn "\n=== List Operations ==="
    print (head numbers)   -- 10
    print (tail numbers)   -- [20,30,40]
    print (init numbers)   -- [10,20,30]
    print (last numbers)   -- 40
    print (length numbers) -- 4
    print (null numbers)   -- False

    putStrLn "\n=== List Construction ==="
    print (5 : numbers)    -- [5,10,20,30,40]
    print (numbers ++ [50,60]) -- [10,20,30,40,50,60]
    print (replicate 4 7)  -- [7,7,7,7]
    print (take 3 numbers) -- [10,20,30]
    print (drop 2 numbers) -- [30,40]

    putStrLn "\n=== List Comprehensions ==="
    print [x*2 | x <- numbers]  -- [20,40,60,80]
    print [x | x <- numbers, x > 25]  -- [30,40]
    print [(x,y) | x <- [1..3], y <- [10..12]] -- Cartesian product

    putStrLn "\n=== Zipping ==="
    print (zip [1..4] "abcd")  -- [(1,'a'),(2,'b'),(3,'c'),(4,'d')]
    print (zipWith (+) numbers [1..4])  -- [11,22,33,44]

    putStrLn "\n=== More Useful Functions ==="
    print (reverse numbers) -- [40,30,20,10]
    print (minimum numbers) -- 10
    print (maximum numbers) -- 40
