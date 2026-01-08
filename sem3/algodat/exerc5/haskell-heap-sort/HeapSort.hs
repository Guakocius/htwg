import Control.Monad
import Data.Array.MArray
import Data.Array.IO

left :: (Integral i, Ix i) => i -> i -> i
left n i = 2 * i + 1 -- Starts at idx 1 => Internally transformed to be zero-based

right :: (Integral i, Ix i) => i -> i -> i
right n i = 2 * i + 2

getN :: (Ix i, Integral i, MArray a e m) => a i e -> m i
getN a = do
    (i0, i1) <- getBounds a
    return (i1 - i0 + 1)

-- Helper Functions for heapifyDown and buildHeap break conditions
isLeaf :: (Integral i, Ix i) => i -> i -> Bool
isLeaf n i = left n i >= n

isEdge :: (Integral i, Ix i) => i -> i -> Bool
isEdge n i = right n i >= n

atIndex :: (Integral i, Ix i, MArray a e m) => a i e -> i -> m e
atIndex a i = do
    (i0, _) <- getBounds a
    readArray a (i + i0)

swap :: (Integral i, Ix i, MArray a e m) => a i e -> i -> i -> m ()
swap a i1 i2 = do
    (i0, _) <- getBounds a
    v1 <- readArray a (i1 + i0)
    v2 <- readArray a (i2 + i0)
    writeArray a (i1 + i0) v2
    writeArray a (i2 + i0) v1

data Dirn = L | R | N

heapifyDown :: (Integral i, Ix i, Ord e, MArray a e m) => a i e -> i -> i -> m ()

heapifyDown a n i = do
    c <- atIndex a i
    let
        --max = i
        li = left n i
        re = right n i
    x <- if isLeaf n i then return N
        else if isEdge n i then do
            l <- atIndex a li
            if c >= l then return N 
            else return L
        else do
            l <- atIndex a li
            r <- atIndex a re
            if c >= l && c >= r then return N
            else if l >= r then return L
            else return R
    case x of 
        L -> do
            swap a i li
            heapifyDown a n li
        R -> do
            swap a i re
            heapifyDown a n re
        N -> return ()


buildHeap :: (Integral i, Ix i, Ord e, MArray a e m) => a i e -> m ()
buildHeap a = do
    n <- getN a
    mapM_ (heapifyDown a n) [div n 2 - 1, div n 2 - 2 .. 0]

extract :: (Integral i, Ix i, Ord e, MArray a e m) => a i e -> m ()
extract a = do
    n <- getN a
    let extractRoot k = do
            swap a k 0
            heapifyDown a k 0
    mapM_ extractRoot [n - 1, n - 2 .. 1]

heapSort :: (Integral i, Ix i, Ord e, MArray a e m) => a i e -> m ()
heapSort a = do
    buildHeap a
    extract a

main :: IO ()
main = do
    let arr = [5, 3, 8, 4, 2, 7, 1, 6] :: [Int]
    mArr <- newListArray (0, length arr - 1) arr :: IO (IOArray Int Int)
    heapSort mArr
    sortedArr <- getElems mArr
    print sortedArr
