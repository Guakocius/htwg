file://<WORKSPACE>/Solution.scala
### java.lang.AssertionError: assertion failed: found: Array.apply[Int]: (implicit evidence$5: scala.reflect.ClassTag[Int]): Array[Int], expected: Any

occurred in the presentation compiler.

presentation compiler configuration:


action parameters:
offset: 117
uri: file://<WORKSPACE>/Solution.scala
text:
```scala
class BinarySearch(arr: Vector[Int], key: Int):
  val a: Array[Int] = arr

  def enlarge(): Array[Int] = Array[Int].c@@

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
          li + ((key - a(li)) * (re - li)) / (a(re) - a(li))
        if key < a(mid) then (li, mid-1) else (mid+1, re)
        ).dropWhile((li, re) => li <= re && a((li+re)/2) != key)
          .nextOption().map((li, re) => 
            val mid = if a(li) < a(re) && a(li) <= key && key <= a(re) then
              li + ((key - a(li)) * (re - li)) / (a(re) - a(li))
              if li > re then -1 else mid)
          .getOrElse(-1)

  def insert(key: Int): Boolean = if search(key) < 0 then
    val i: Int = search(key) - 1


@main def main() =
  val n: Int = 10
  val key: Int = 11
  val v: Vector[Int] = Vector()
  val binSearch = BinarySearch(v, key)
  println(s"Hello, World from insert: ${binSearch.insert(n)}")
  println(s"Test search: ${binSearch.search(n)}")


```



#### Error stacktrace:

```
scala.runtime.Scala3RunTime$.assertFailed(Scala3RunTime.scala:8)
	dotty.tools.dotc.typer.Implicits$ImplicitSearch.<init>(Implicits.scala:1206)
	dotty.tools.dotc.interactive.Completion$Completer.implicitConversionTargets(Completion.scala:630)
	dotty.tools.dotc.interactive.Completion$Completer.implicitConversionMemberCompletions(Completion.scala:514)
	dotty.tools.dotc.interactive.Completion$Completer.selectionCompletions(Completion.scala:445)
	dotty.tools.dotc.interactive.Completion$.computeCompletions(Completion.scala:218)
	dotty.tools.dotc.interactive.Completion$.rawCompletions(Completion.scala:78)
	dotty.tools.pc.completions.Completions.enrichedCompilerCompletions(Completions.scala:114)
	dotty.tools.pc.completions.Completions.completions(Completions.scala:136)
	dotty.tools.pc.completions.CompletionProvider.completions(CompletionProvider.scala:139)
	dotty.tools.pc.ScalaPresentationCompiler.complete$$anonfun$1(ScalaPresentationCompiler.scala:150)
```
#### Short summary: 

java.lang.AssertionError: assertion failed: found: Array.apply[Int]: (implicit evidence$5: scala.reflect.ClassTag[Int]): Array[Int], expected: Any