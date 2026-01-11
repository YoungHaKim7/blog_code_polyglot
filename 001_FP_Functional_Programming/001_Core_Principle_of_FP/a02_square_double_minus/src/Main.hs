add :: Int -> Int -> Int
add x y = x + y

square :: Int -> Int
square x = x * x

double :: Int -> Int
double x = x * 2



main :: IO ()
main = do
    let result = add 3 5
    print result

    let res_square = square 3
    print res_square

    let res_double = double 9
    print res_double
