module Lib
    ( Node(key,val), buildHeap, heapSort ) where

data Node = Node { key :: Int, val :: Int }

heapSort :: Ord k => [Node k v] -> [Node k v]
heapifyDown :: Ord k => STArray a Int (Node k v) -> Int -> Int -> ST s ()

buildHeap :: Ord k => [Node k v] -> [Node k v]


heapifyDown a n i = do
    let
        li = 2*i
        re = li+1

    max' <- do
        m <- readArray a i
        liOk <- if li > n then Just <$> readArray a li else pure Nothing
        reOk <- if re > n then Just <$> readArray a re else pure Nothing

        let maxL = case liOk of
                Just lv -> if key lv > key m then li else i
                _       -> i
        case reOk of
            Just rv -> if key rv > key (a ! maxL) then pure re else pure maxL
            _       -> pure maxL
    when (max' /= i) $ do
        swap arr i max'
        heapifyDown arr n max'


buildHeap xs = [length,-1..0]
