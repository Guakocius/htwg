package de.htwg.se

case class Player(name: String, playerUnits: Array[PlayerUnit], playerCities: Array[PlayerCity])
case class PlayerUnit(name: String, owner: Player, var x: Int, var y: Int)
case class PlayerCity(owner: Player, x: Int, y: Int)

