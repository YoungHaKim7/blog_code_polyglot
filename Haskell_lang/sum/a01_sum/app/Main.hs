module Main (main) where

-- Type class for addition
class Add a where
  add :: a -> a -> a

instance Add Int where
  add = (+)

instance Add Integer where
  add = (+)

-- Type class for iteration
class Iterator i where
  forEach :: (a -> b -> b) -> i a -> b -> b

instance Iterator [] where
  forEach f xs z = foldr (\x acc -> f x acc) z xs

-- Sum function using the type classes
sum' :: (Iterator i, Add t, Num t) => i t -> t
sum' xs = forEach (\x acc -> add x acc) xs 0

main :: IO ()
main = do
  let result = sum' [1, 2, 3, 4, 5] :: Int
  putStrLn $ "Sum: " ++ show result
