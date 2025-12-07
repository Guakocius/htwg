import scala.util.boundary, boundary.break

class BinarySearch(arr: Array[Int]):
  var a: Array[Int] = arr

  def enlarge(a: Array[Int]): Array[Int] = Array.copyOf(a, a.length+1)
  def shrink(a: Array[Int]): Array[Int] = Array.copyOf(a, a.length-1)


  def search(key: Int): Int = 
    Iterator.iterate((0, a.length - 1))((li, re) =>
        val mid: Int = (li+re)/2
        if key < a(mid) then (li, mid-1) else (mid+1, re)
        ).dropWhile((li, re) => li <= re && a((li+re)/2) != key)
          .nextOption().map((li, re) => 
              val mid = (li+re)/2; if li > re then -1 else mid)
          .getOrElse(-1)

  def searchInterpol(key: Int): Int = 
    Iterator.iterate((0, a.length - 1))((li, re) =>
        val mid: Int = if a(li) < a(re) && a(li) <= key && key <= a(re) then 
        li + ((key - a(li)) * (re - li)) / (a(re) - a(li)) else (li+re)/2
        if key < a(mid) then (li, mid-1) else (mid+1, re)
        ).dropWhile((li, re) => li <= re && a((li+re)/2) != key)
          .nextOption().map((li, re) => 
            val mid: Int = if a(li) < a(re) && a(li) <= key && key <= a(re) then 
            li + ((key - a(li)) * (re - li)) / (a(re) - a(li)) else (li+re)/2
              if li > re then -1 else mid)
          .getOrElse(-1)

  def searchLLB(key: Int): Int = 
    Iterator.iterate((0, a.length - 1))((li, re) =>
        while li <= re do
          val m = (li+re)/2
          if a(mid) <= key then
            li = mid+1
            m


          if key < a(mid) then (li, mid-1) else (mid+1, re)
          ).dropWhile((li, re) => li <= re && a((li+re)/2) != key)
            .nextOption().map((li, re) => 
              val mid: Int = if a(li) < a(re) && a(li) <= key && key <= a(re) then 
              li + ((key - a(li)) * (re - li)) / (a(re) - a(li)) else (li+re)/2
                if li > re then -1 else mid)
            .getOrElse(-1)




  def insert(key: Int): Boolean = 
    boundary:
      val h: Int = search(key)
      if h >= 0 then break(false)
      else
        a = enlarge(a)
        val i: Int = if a.length <= 1 then -1 else h - 1
        println(s"i: ${i}")
        for j <- a.length - 1 until i+1 by -1 do a.update(j, a(j-1))
        a.update(i+1, key)
        break(true)
    

  def remove(key: Int): Boolean = 
    if search(key) < 0 then
      val i: Int = search(key)
      if i > -1 then for j <- i until a.length - 1 do a.update(j, a(j+1))
    a = shrink(a)
    if !a.exists(k => k == key) then true else false


@main def main() =
  val n: Int = 10
  val key: Int = 11
  val a: Array[Int] = Array()
  val binSearch = BinarySearch(a)
  println(s"Hello, World from insert: ${binSearch.insert(n)}")
  binSearch.insert(key)
  binSearch.a.foreach(i => print(s"${i} "))
  println
  binSearch.insert(n-1)
  for i <- 0 to 10 do binSearch.insert(i)
  println(s"Test search: ${binSearch.search(n)}")
  binSearch.a.foreach(i => print(s"${i} "))
  println
 
  binSearch.remove(n)
  binSearch.a.foreach(i => print(s"${i} "))
  println

