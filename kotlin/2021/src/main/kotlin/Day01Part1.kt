import java.nio.file.Files
import java.nio.file.Paths

fun main(args: Array<String>) {
    val stream = Files.newInputStream(Paths.get("src/main/resources/Day01Part1"))
    var increases = 0
    var num: Int? = null
    stream.buffered().reader().use { reader ->
        reader.forEachLine {
            val next = it.toInt()
            if ((num != null) && (num!! < next)) {increases ++}
            num = next
        }
    }

    println("increases $increases")
}