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

    fn compact_whole_files(&mut self) {
        let mut largest_remaining_gap = self.blocks.len();
        let mut file_end = self.blocks.len() - 1;

        for (i, j) in (0..self.blocks.len()).zip(1..self.blocks.len()).rev() {
            if largest_remaining_gap == 0 {
                break;
            }
            match (
                self.blocks[i],
                self.blocks[j],
                self.blocks[i] == self.blocks[j],
            ) {
                (Some(_), None, _) => {
                    file_end = i;
                }
                (None, None, _) | (_, _, true) => {}
                (None, Some(_), _) | (Some(_), Some(_), false) => {
                    let file_start = j;
                    let file_len = file_end - file_start + 1;
                    if file_len > largest_remaining_gap {
                        file_end = i;
                        continue;
                    }

                    let mut gap_start = 0;
                    let mut largest_gap_this_time = 0;
                    for (k, l) in (0..=file_start).zip(1..=file_start) {
                        match (self.blocks[k], self.blocks[l]) {
                            (Some(_), None) => {
                                gap_start = l;
                            }
                            (None, Some(_)) => {
                                let gap_end = k;
                                let gap_len = gap_end - gap_start + 1;
                                largest_gap_this_time = usize::max(largest_gap_this_time, gap_len);

                                if gap_len >= file_len {
                                    for (m, n) in (gap_start..=gap_end).zip(file_start..=file_end) {
                                        self.blocks.swap(m, n);
                                    }
                                    largest_gap_this_time = largest_remaining_gap;
                                    break;
                                }
                            }
                            (None, None) | (Some(_), Some(_)) => {}
                        }
                    }
                    largest_remaining_gap = largest_gap_this_time;
                    file_end = i;
                }
            }
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

pub fn part_2(input: &str) -> usize {
    let mut disk = Disk::parse(input);
    disk.compact_whole_files();
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

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 2858);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 6265268809555);
    }
}
