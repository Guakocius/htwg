class BinarySearch(arr: Vector[Int], key: Int):
  val v: Vector[Int] = arr

  def search(key: Int): Int = v

  def insert(key: Int): Boolean =
    println(v.length)
    
    true


@main def main() =
  val n: Int = 10
  val key: Int = 11
  val v: Vector[Int] = Vector()
  val binSearch = BinarySearch(v, key)
  println(s"Hello, World from insert: ${binSearch.insert(10)}")
  println(s"Test search: ${binSearch.search(10)}")

