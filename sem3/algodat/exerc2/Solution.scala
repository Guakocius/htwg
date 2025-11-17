class BinarySearch(arr: Vector[Int], key: Int):
  val a: Vector[Int] = arr

  def search(key: Int): Int = 
    Iterator.iterate(0, a.length - 1)((li, re) =>
        val mid: Int = (li+re)/2
        if key < a(mid) then (li, mid-1) else (mid+1, re)
        ).dropWhile((li, re) => li <= re && a((li+re)/2) != key)
          .nextOption()
  def insert(key: Int): Boolean = true


@main def main() =
  val n: Int = 10
  val key: Int = 11
  val v: Vector[Int] = Vector()
  val binSearch = BinarySearch(v, key)
  println(s"Hello, World from insert: ${binSearch.insert(n)}")
  println(s"Test search: ${binSearch.search(n)}")

