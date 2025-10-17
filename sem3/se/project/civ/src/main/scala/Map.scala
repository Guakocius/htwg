package de.htwg.se

import scalafx.scene.paint.Color

val RESET = "\u001b[0m"
val GREEN = "\u001b[32m"
val YELLOW = "\u001b[33m"
val WHITE = "\u001b[37m"
val BLUE = "\u001b[34m"

def isBoat(unit: PlayerUnit): Boolean = {
  true
}

case class Tile(x: Int, y: Int, terrain: Terrain, var unit: Option[PlayerUnit] = None,
  var city: Option[PlayerCity] = None) {
    override def toString(): String = (if (terrain.color == Color.GREEN) GREEN else if (terrain.color == Color.BROWN) YELLOW else if (terrain.color == Color.BLUE) BLUE else WHITE) + terrain.symbol + RESET
  }
sealed trait Terrain { 
  val moveCost: Int 
  val color: Color
  def symbol: String
}
case object Grassland extends Terrain { 
  val moveCost = 1 
  val color = Color.GREEN
  val symbol = " . "
}
case object Mountain extends Terrain { 
  val moveCost = 3
  val color = Color.BROWN
  val symbol = " ^ "
}
case object Ocean extends Terrain {
  val moveCost = 2
  val color = Color.BLUE
  val symbol = " _ "
}

