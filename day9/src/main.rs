const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut free = false;
    let mut buffer: Vec<Option<usize>> = vec![];

    let mut id: usize = 0;
    for ch in INPUT.trim().as_bytes() {
        if free {
            buffer.resize(buffer.len() + (ch - b'0') as usize, None);
        } else {
            buffer.resize(buffer.len() + (ch - b'0') as usize, Some(id));
            id += 1;
        }
        free = !free;
    }

    buffer.reverse();

    let mut left = 0;

    while left < buffer.len() {
        while left < buffer.len() && buffer[left].is_none() {
            left += 1;
        }

        let start = left;
        let current_id = buffer[left];

        while left < buffer.len() && buffer[left] == current_id {
            left += 1;
        }

        let some_slice = start..left;

        let slice_len = buffer.len() - left;
        let window_size = some_slice.len();

        if slice_len + 1 > window_size {
            // println!("{}", slice_len + 1 - window_size);

            let v: Vec<_> = (0..(slice_len + 1 - window_size))
                .rev()
                .filter(|i| {
                    buffer[left + i..left + i + window_size]
                        .iter()
                        .all(|id| id.is_none())
                })
                .map(|i| (left + i, window_size))
                .collect();

            if let Some(range) = v.first() {
                // println!("Range found: {:?}, doing swap", range.0..range.0 + range.1);
                for i in 0..range.1 {
                    buffer.swap(some_slice.start + i, range.0 + i);
                }
            } else {
                //println!("Space N/A");
            }
        } else {
            //println!("No windows.");
        }
    }

    buffer.reverse();

    let mut checksum: usize = 0;
    for (i, num) in buffer.iter().enumerate() {
        match num {
            Some(a) => checksum += i * a,
            None => continue,
        }
    }
    println!("{checksum}");
}

fn part1() {
    let mut free = false;
    let mut buffer: Vec<Option<usize>> = vec![];

    let mut id: usize = 0;
    for ch in INPUT.trim().as_bytes() {
        if free {
            buffer.resize(buffer.len() + (ch - b'0') as usize, None);
        } else {
            buffer.resize(buffer.len() + (ch - b'0') as usize, Some(id));
            id += 1;
        }
        free = !free;
    }

    let mut left = 0;
    let mut right = buffer.len() - 1;
    while left < right {
        while buffer[left].is_some() && left < right {
            left += 1;
        }
        while buffer[right].is_none() && left < right {
            right -= 1;
        }
        if left < right {
            buffer.swap(left, right);
        }
    }

    let mut checksum: usize = 0;
    for (i, num) in buffer.into_iter().enumerate() {
        match num {
            Some(a) => checksum += i * a,
            None => break,
        }
    }

    println!("{checksum}");
}

trait GetByIsize<T> {
    fn get_i(&self, pos: isize) -> Option<&T>;
}

impl<T> GetByIsize<T> for Vec<T> {
    fn get_i(&self, pos: isize) -> Option<&T> {
        if pos < 0 {
            None
        } else {
            self.get(pos as usize)
        }
    }
}
