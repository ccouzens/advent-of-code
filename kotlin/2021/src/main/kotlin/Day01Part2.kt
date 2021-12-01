import java.nio.file.Files
import java.nio.file.Paths
import java.util.*

fun main(args: Array<String>) {
    val stream = Files.newInputStream(Paths.get("src/main/resources/Day01Part1"))
    var increases = 0
    val window: Queue<Int> = LinkedList<Int>()
    stream.buffered().reader().use { reader ->
        reader.forEachLine {
            val current = it.toInt()
            window.add(current)

            if (window.size == 4) {
                val previous = window.remove()
                if (previous < current) { increases ++ }
            }
        }
    }

    println("increases $increases")
}