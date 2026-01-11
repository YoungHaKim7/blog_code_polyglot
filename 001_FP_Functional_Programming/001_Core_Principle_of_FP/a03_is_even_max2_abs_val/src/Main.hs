-- isEven
isEven :: Int -> Bool
isEven x = x `mod` 2 == 0

-- max
max2 :: Int -> Int -> Int
max2 x y = if x > y then x else y

-- abs
absVal :: Int -> Int
absVal x = if x < 0 then -x else x

main :: IO ()
main = do
    let result = isEven 2
    print result

    let res_max = max2 10 8
    print res_max

    let res_absval = absVal (-20)
    print res_absval
