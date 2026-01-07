data Node a
 = Empty
 | Node
    { key :: Int
    , value :: a
    , left :: Node a
    , right :: Node a
    } deriving (Show, Eq)

buildHeap :: [Node a] -> [Node a]
buildHeap a = heapifyDown (a i)

heapSort :: [Node a] -> [Node a]
heapSort _ = []

heapifyDown :: [Node a] -> Integer -> [Node a]
heapifyDown a i = do
    let
        max = i
        li = 2*i
        re = li+1
    if 
    []