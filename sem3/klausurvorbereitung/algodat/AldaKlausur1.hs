import Numeric.Natural

binary :: Natural -> String
binary x
    | x < 2 = show x
    | otherwise = binary (div x 2) ++ show (mod x 2)



main :: IO()
main = do
    let x = 10
    putStrLn ("x in binary: " ++ binary x)