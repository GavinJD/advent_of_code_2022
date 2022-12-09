use std::str::FromStr;

use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use itertools::Itertools;

const TOTAL_FILE_SPACE: u64 = 70000000;
const REQUIRED_FILE_SPACE: u64 = 30000000;

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Day 7");
    let input = include_str!("input/day_7.txt");
    println!("Part 1: {}", total_size_of_at_most(100_000, input)?);
    println!("Part 2: {}", size_of_smallest_dir_to_delete(input)?);
    Ok(())
}

fn total_size_of_at_most(limit: u64, input: &str) -> Result<u64> {
    let system = FileSystem::from_str(input)?;

    Ok(system
        .dirs()
        .iter()
        .filter(|dir| dir.size() <= limit)
        .fold(0, |sum, dir2| sum + dir2.size()))
}

fn size_of_smallest_dir_to_delete(input: &str) -> Result<u64> {
    let system = FileSystem::from_str(input)?;

    let used_space: u64 = system.root_size();
    let required_space = REQUIRED_FILE_SPACE as i64 - (TOTAL_FILE_SPACE - used_space) as i64;

    if required_space <= 0 {
        return Err(eyre!("There is already enough space"));
    }

    system
        .dirs()
        .iter()
        .map(|f| f.size())
        .filter(|size| *size >= required_space as u64)
        .min()
        .ok_or(eyre!("No dir found for deletion?"))
}

type FileId = usize;
#[derive(Debug)]
enum FileType {
    Directory {
        id: FileId,
        parent: Option<FileId>,
        name: String,
        content: Vec<FileId>,
        total_size: u64,
    },
    File {
        id: FileId,
        parent: Option<FileId>,
        name: String,
        size: u64,
    },
}

#[derive(Debug)]
struct FileSystem {
    data: Vec<FileType>,
}

impl FileType {
    fn size(&self) -> u64 {
        match self {
            FileType::Directory {
                name: _,
                id: _,
                parent: _,
                content: _,
                total_size,
            } => *total_size,
            FileType::File {
                id: _,
                parent: _,
                name: _,
                size,
            } => *size,
        }
    }

    fn id(&self) -> FileId {
        match self {
            FileType::Directory {
                name: _,
                id,
                parent: _,
                content: _,
                total_size: _,
            } => *id,
            FileType::File {
                id,
                parent: _,
                name: _,
                size: _,
            } => *id,
        }
    }

    fn parent(&self) -> FileId {
        match self {
            FileType::Directory {
                name: _,
                parent,
                id: _,
                content: _,
                total_size: _,
            } => parent.unwrap_or(ROOT_FILE_ID),
            FileType::File {
                parent,
                id: _,
                name: _,
                size: _,
            } => parent.unwrap_or(ROOT_FILE_ID),
        }
    }
}

const ROOT_FILE_ID: usize = 0;
impl FileSystem {
    fn new() -> Self {
        Self {
            data: vec![FileType::Directory {
                name: "/".to_string(),
                content: Vec::new(),
                parent: None,
                id: ROOT_FILE_ID,
                total_size: 0,
            }],
        }
    }

    fn get_child_directory_with_name(&self, parent: FileId, name: &str) -> Option<FileId> {
        self.data
            .get(parent)
            .and_then(|f| match f {
                FileType::Directory {
                    id: _,
                    parent: _,
                    name: _,
                    content,
                    total_size: _,
                } => Some(content),
                _ => None,
            })
            .and_then(|children| {
                children
                    .iter()
                    .filter_map(|id| self.data.get(*id))
                    .find(|f| match f {
                        FileType::Directory {
                            id: _,
                            parent: _,
                            content: _,
                            total_size: _,
                            name: file_name,
                        } => file_name == name,
                        _ => false,
                    })
            })
            .map(|f| f.id())
    }

    fn get_parent_of_file(&self, file_id: FileId) -> Option<FileId> {
        self.data.get(file_id).map(|f| f.parent())
    }

    fn add_file(&mut self, parent: FileId, name: &str, size: u64) -> Result<()> {
        self.add_child_to_directory(parent, self.data.len(), size)?;
        self.data.push(FileType::File {
            id: self.data.len(),
            parent: Some(parent),
            name: name.to_string(),
            size,
        });
        Ok(())
    }

    fn add_empty_directory(&mut self, parent: FileId, name: &str) -> Result<()> {
        self.add_child_to_directory(parent, self.data.len(), 0)?;
        self.data.push(FileType::Directory {
            id: self.data.len(),
            parent: Some(parent),
            name: name.to_string(),
            content: Vec::new(),
            total_size: 0,
        });
        Ok(())
    }

    fn dirs(&self) -> Vec<&FileType> {
        self.data
            .iter()
            .filter(|f| match f {
                FileType::File { .. } => false,
                FileType::Directory { .. } => true,
            })
            .collect::<Vec<_>>()
    }

    fn add_child_to_directory(
        &mut self,
        parent: FileId,
        child: FileId,
        size_increase: u64,
    ) -> Result<()> {
        let mut updated_item_parent = None;

        match self
            .data
            .get_mut(parent)
            .ok_or(eyre!("Could not add child because parent not found"))?
        {
            FileType::Directory {
                id: _,
                parent: p_id,
                name: _,
                content,
                total_size,
            } => {
                *total_size += size_increase;
                content.push(child);
                if p_id.is_some() && size_increase > 0 {
                    updated_item_parent = Some(p_id.unwrap());
                }
            }
            _ => return Err(eyre!("Cannot add child to a file")),
        };
        if let Some(found_item_parent) = updated_item_parent {
            self.update_parent_size(found_item_parent, size_increase)?;
        };

        Ok(())
    }

    fn update_parent_size(&mut self, id: usize, size_increase: u64) -> Result<()> {
        let mut parent_of_updated = None;

        if let Some(f) = self.data.get_mut(id) {
            match f {
                FileType::Directory {
                    id: _,
                    parent,
                    name: _,
                    content: _,
                    total_size,
                } => {
                    *total_size += size_increase;
                    if parent.is_some() {
                        parent_of_updated = Some(parent.unwrap());
                    }
                }
                FileType::File {
                    id: _,
                    parent: _,
                    name: _,
                    size: _,
                } => return Err(eyre!("Found file with parent as file!")),
            }
        }

        if let Some(found_item_parent) = parent_of_updated {
            self.update_parent_size(found_item_parent, size_increase)?;
        }

        Ok(())
    }

    fn root_size(&self) -> u64 {
        self.data.get(0).unwrap().size()
    }
}

impl FromStr for FileSystem {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut f = FileSystem::new();

        let mut current_dir = ROOT_FILE_ID;
        for line in s
            .lines()
            .skip_while(|l| !l.eq_ignore_ascii_case("$ cd /"))
            .skip(1)
        {
            // dbg!(&f);
            if line.starts_with('$') {
                // command
                if line.len() == 4 {
                    // '$ ls' command
                    continue;
                } else {
                    let (command, arg) = line
                        .split_whitespace()
                        .skip(1)
                        .next_tuple()
                        .ok_or(eyre!("Malformed command"))?;

                    match command {
                        "cd" => {
                            if arg == ".." {
                                current_dir = f
                                    .get_parent_of_file(current_dir)
                                    .ok_or(eyre!("Parent not found"))?;
                            } else {
                                current_dir = f
                                    .get_child_directory_with_name(current_dir, arg)
                                    .ok_or(eyre!("Attempting to cd to a non valid directory"))?;
                            }
                        }
                        _ => return Err(eyre!("Unrecognized command")),
                    }
                }
            } else {
                // description
                let (first, second) = line
                    .split_once(' ')
                    .ok_or(eyre!("Ill formed file item descriptor"))?;

                if first == "dir" {
                    f.add_empty_directory(current_dir, second)?;
                } else {
                    f.add_file(current_dir, second, first.parse::<u64>()?)?;
                }
            }
        }

        Ok(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{total_size_of_at_most, size_of_smallest_dir_to_delete};

    #[test]
    fn example_part1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(total_size_of_at_most(100_000, input).unwrap(), 95437);
    }


    #[test]
    fn example_part2() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(size_of_smallest_dir_to_delete(input).unwrap(), 24933642);
    }
}
