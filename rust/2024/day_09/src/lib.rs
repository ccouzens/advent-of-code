use std::iter::repeat_n;

#[derive(Debug)]
struct Disk {
    blocks: Vec<Option<usize>>,
}

impl Disk {
    fn parse(input: &str) -> Self {
        let blocks = input
            .bytes()
            .enumerate()
            .filter_map(|(i, d)| Some((i, d.checked_sub(b'0')?)))
            .flat_map(|(i, n)| repeat_n((i % 2 == 0).then_some(i / 2), n.into()))
            .collect();

        Self { blocks }
    }

    fn compact_files(&mut self) {
        let mut free_space_search_index = 0;
        let mut block_search_index = self.blocks.len() - 1;
        while free_space_search_index < block_search_index {
            if self.blocks[free_space_search_index].is_some() {
                free_space_search_index += 1;
                continue;
            }
            if self.blocks[block_search_index].is_none() {
                block_search_index -= 1;
                continue;
            }
            self.blocks
                .swap(free_space_search_index, block_search_index);
        }
    }

    fn filesystem_checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(p, file_id)| p * file_id.unwrap_or(0))
            .sum()
    }
}

pub fn part_1(input: &str) -> usize {
    let mut disk = Disk::parse(input);
    disk.compact_files();
    disk.filesystem_checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 1928);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 6241633730082);
    }
}
