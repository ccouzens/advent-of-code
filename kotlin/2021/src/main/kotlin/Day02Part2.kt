import java.nio.file.Files
import java.nio.file.Paths

fun main(args: Array<String>) {
    val stream = Files.newInputStream(Paths.get("src/main/resources/Day02"))
    var horizontalPosition = 0
    var depth = 0
    var aim = 0
    stream.buffered().reader().use { reader ->
        reader.forEachLine {
            val (direction, magnitudeS) = it.split(" ", limit= 2)
            val magnitude = magnitudeS.toInt()
            when (direction) {
                "forward" -> {
                    horizontalPosition += magnitude
                    depth += magnitude * aim
                }
                "down" -> aim += magnitude
                "up" -> aim -= magnitude
            }
        }
    }

    println("depth $depth")
    println("horizontal $horizontalPosition")
    println("product ${depth * horizontalPosition}")
}