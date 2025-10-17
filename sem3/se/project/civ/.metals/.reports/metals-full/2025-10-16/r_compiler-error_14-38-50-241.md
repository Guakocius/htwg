error id: 1E214C4732F5BA9FEE591810908CA72A
file://<WORKSPACE>/src/main/scala/Main.scala
### java.lang.IndexOutOfBoundsException: -1

occurred in the presentation compiler.



action parameters:
offset: 724
uri: file://<WORKSPACE>/src/main/scala/Main.scala
text:
```scala
package de.htwg.se

import scalafx.application.JFXApp3
import scalafx.scene.Scene
import scalafx.scene.layout.HBox
import scalafx.scene.text.Text
import scalafx.scene.paint.LinearGradient
import scalafx.scene.paint.Color._
import scalafx.scene.paint._
import scalafx.scene.paint.Stops
import scalafx.scene.effect.DropShadow
import scalafx.geometry.Insets
import scalafx.scene.layout.GridPane
import scalafx.scene.shape.StrokeLineCap.Square
import scalafx.scene.shape.Rectangle
import scalafx.scene.paint.PaintIncludes.string2sfxColor

object Civ extends JFXApp3 {
  val worldMap: Vector[Vector[Tile]] =
    Vector.tabulate(40, 40)((y, x) => Tile(x, y, if ((x + y) % 5 == 0) Mountain else if ((x + y) % 10 == 0 && (x + y) % 5@@ != 0) Ocean else Grassland))
  val worldGrid = new GridPane
  for {
        row <- worldMap
        tile <- row
      } {
        val rect = new Rectangle {
          width_=(40)
          height_=(40)
          fill = tile.terrain.color
        }
        worldGrid.add(rect, tile.x, tile.y)
      }

  override def start(): Unit = {
    for (row <- worldMap) {
      println(row.map(_.toString).mkString)
    }
    stage = new JFXApp3.PrimaryStage {
      title = "Civilization in Scala"
      scene = new Scene(800, 600) {
        content = worldGrid
      }
    }
  }
}

```


presentation compiler configuration:
Scala version: 3.7.3-bin-nonbootstrapped
Classpath:
<HOME>/.cache/coursier/v1/https/repo1.maven.org/maven2/org/scala-lang/scala3-library_3/3.7.3/scala3-library_3-3.7.3.jar [exists ], <HOME>/.cache/coursier/v1/https/repo1.maven.org/maven2/org/scala-lang/scala-library/2.13.16/scala-library-2.13.16.jar [exists ]
Options:





#### Error stacktrace:

```
scala.collection.LinearSeqOps.apply(LinearSeq.scala:129)
	scala.collection.LinearSeqOps.apply$(LinearSeq.scala:128)
	scala.collection.immutable.List.apply(List.scala:79)
	dotty.tools.dotc.util.Signatures$.applyCallInfo(Signatures.scala:244)
	dotty.tools.dotc.util.Signatures$.computeSignatureHelp(Signatures.scala:101)
	dotty.tools.dotc.util.Signatures$.signatureHelp(Signatures.scala:88)
	dotty.tools.pc.SignatureHelpProvider$.signatureHelp(SignatureHelpProvider.scala:46)
	dotty.tools.pc.ScalaPresentationCompiler.signatureHelp$$anonfun$1(ScalaPresentationCompiler.scala:498)
```
#### Short summary: 

java.lang.IndexOutOfBoundsException: -1