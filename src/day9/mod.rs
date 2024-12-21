use std::collections::HashMap;
use std::iter::Rev;
use std::str::Chars;

struct DiskReverseIterator<'a> {
    iterator: Rev<Chars<'a>>,
    buffer: Vec<usize>,
    last_file_id: usize,
}

impl<'a> Iterator for DiskReverseIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {

            let next = self.iterator.next()?;
            // parse next for how large this file block is
            let file_size = next.to_digit(10).unwrap();
            println!("Adding {} blocks of {} to buffer", file_size, self.last_file_id);
            self.buffer.append(&mut vec![self.last_file_id; file_size as usize]);
            // now pop the next free space and throw it away
            self.iterator.next();
            // decrement the last file id
            self.last_file_id -= 1;
        }
        self.buffer.pop()
    }
}

impl<'a> DiskReverseIterator<'a> {
    fn new(input: &'a str) -> Self {
        let mut iterator = input.chars().rev();
        let last_file_id = {
            if input.len() % 2 == 0 {
                // the last character is free space, so pop it and throw it away now
                iterator.next();
                input.len() / 2 - 1
            } else {
                input.len() / 2
            }
        };
        Self {
            iterator,
            buffer: Vec::new(),
            last_file_id,
        }
    }
}

struct File {
    id: usize,
    size: usize,
    position: usize,
}

#[derive(Copy, Clone, Debug)]
struct FreeSpace {
    size: usize,
    position: usize,
}
pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");

    let input = if test { test_input } else { real_input };

    let mut disk_iterator = DiskReverseIterator::new(input);

    let mut mode = 'f'; // 'f' for file, 's' for space
    let mut current_file_id: usize = 0;
    let mut position: usize = 0;
    // this is hilarious, but I think it's just a fold?
    let result = input.chars().fold(0usize, |mut acc, c| {
        if mode == 'f' {
            // parse the number
            let iterations = c.to_digit(10).unwrap();
            // println!("{} blocks of file: {}", iterations, current_file_id);
            let last_file_id = disk_iterator.last_file_id;
            if last_file_id < current_file_id {
                // we are done, just need to handle the end
                mode = 's';
                return acc;
            }
            for _ in 0..iterations {
                // println!("Adding {} * {} to acc", current_file_id, position);
                acc += current_file_id * position;
                position += 1;
            }
            current_file_id += 1;
            mode = 's';
            return acc;
        }
        if mode == 's' {
            // parse the number
            let iterations = c.to_digit(10).unwrap();
            // println!("{} blocks of free space", iterations);
            let last_file_id = disk_iterator.last_file_id;
            if last_file_id < current_file_id {
                // we are done, just need to handle the end
                while !disk_iterator.buffer.is_empty() {
                    let next_file_id = disk_iterator.buffer.pop().unwrap();
                    // println!("Adding {} * {} to acc", next_file_id, position);
                    acc += next_file_id * position;
                    position += 1;
                }
                mode = 'f';
                return acc;
            }
            for _ in 0..iterations {

                let next_file_id = disk_iterator.next().unwrap();
                // println!("Adding {} * {} to acc", next_file_id, position);
                acc += next_file_id * position;
                position += 1;
            }
            mode = 'f';
            return acc;
        }
        return acc;
    });
    println!("Result: {}", result);

    // actually we'll take a whole different approach with part 2
    // scan through the whole disk and index the files and keep their size id and position
    // also index the free space by its size and position
    // then reverse iterate through the files and look up where the free space is that will fit it. Update the file position
    // finally we have to sort the files by their position and run the calculations

    let mut files = Vec::new();
    let mut free_spaces = Vec::new();
    let mut current_file_id = 0;
    let mut current_position = 0;
    let mut mode = 'f';
    for c in input.chars() {
        if let Some(next_size)  = c.to_digit(10)  {
            if mode == 'f' {
                let file = File {
                    id: current_file_id,
                    size: next_size as usize,
                    position: current_position,
                };
                files.push(file);
                current_position += next_size as usize;
                current_file_id += 1;
                mode = 's';
            } else {
                let free_space = FreeSpace {
                    size: next_size as usize,
                    position: current_position,
                };
                free_spaces.push(free_space);
                current_position += next_size as usize;
                mode = 'f';
            }
        }
    }

    for file in files.iter_mut().rev() {
        // find the free space that fits this file
        // println!("Trying to find space for file: {} size: {}", file.id, file.size);

        let mut space = free_spaces.iter_mut().find(|space| space.size >= file.size && space.position <= file.position);
        if let Some(s) = space {
            let orig_pos = file.position;
            // println!("Found space found: {} size: {} ", s.position, s.size);
            file.position = s.position;
            // now we have to awkwardly shorten this free space
            if s.size > file.size {
                // println!("Shortening free space {} {}", s.position, s.size);
                s.position += file.size;
                s.size -= file.size;
            } else {
                // println!("Removed free space {} {}", s.position, s.size);
                // remove the space
                s.size = 0;

            }
            // ugg, we also have to combine the free spaces on either side, if any. That's a headache :(
            // create a new free space
            let mut new_space = FreeSpace {
                size: file.size,
                position: orig_pos,
            };
            // insert that space into the free spaces at the first position where new_space position is greater
            let index = free_spaces.iter().position(|s| s.position > new_space.position);
            match index {
                Some(index) => free_spaces.insert(index, new_space),
                None => free_spaces.push(new_space),
            }

            // now scan through and combine any free spaces that are adjacent, unfortunately we are going to have to allocate memory here
            let mut new_free_spaces = Vec::new();
            let mut free_space_iter = free_spaces.iter().peekable();
            while let Some(space) = free_space_iter.next() {
                // println!("Checking space: {} {}", space.position, space.size);
                let mut adjacent_spaces = vec![space];
                while let Some(next_space) = free_space_iter.peek() {
                    // println!("Checking next space: size: {} {} == {}", next_space.size, next_space.position, space.position + adjacent_spaces.iter().map(|s| s.size).sum::<usize>());
                    if next_space.position == space.position + adjacent_spaces.iter().map(|s| s.size).sum::<usize>() {
                        // println!("Found adjacent space: {} {}", next_space.position, next_space.size);
                        adjacent_spaces.push(next_space);
                        free_space_iter.next();
                    } else {
                        break;
                    }
                }
                // combine the adjacent spaces
                let new_space = FreeSpace {
                    size: adjacent_spaces.iter().map(|s| s.size).sum(),
                    position: adjacent_spaces[0].position,
                };
                if new_space.size > 0 {
                    new_free_spaces.push(new_space);
                }

            }
            // println!("New free spaces: {:?}", new_free_spaces);
            free_spaces = new_free_spaces;
        }



    }

    files.sort_by_key(|file| file.position);
    let result = files.iter().fold(0usize, |mut acc, file| {
        for i in 0..file.size {
            // println!("Adding to acc {} * {}", file.id, file.position + i);
            acc += file.id * (file.position + i);
        }
        acc
    });
    println!("Result Part 2: {}", result);
}