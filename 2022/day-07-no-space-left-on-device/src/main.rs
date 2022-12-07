use std::collections::HashMap;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
enum Command<'a> {
    ChangeDirectory(&'a str),
    List(Vec<DirectoryEntry<'a>>),
}

impl Command<'_> {
    fn parse_commands(input: &str) -> Vec<Command> {
        assert_eq!(&input[0..2], "$ ");
        input[2..]
            .split("\n$ ")
            .map(|s| Command::parse(s.trim()))
            .collect()
    }

    fn parse<'a>(input: &'a str) -> Command<'a> {
        use Command::*;

        let mut lines = input.lines();
        let mut command_and_args = lines.next().unwrap().split(' ');
        let command = command_and_args.next().unwrap();
        let args: Vec<&str> = command_and_args.collect();

        match command {
            "cd" => {
                assert_eq!(args.len(), 1);
                assert_eq!(lines.next(), None);
                ChangeDirectory(args[0])
            }
            "ls" => {
                assert_eq!(args.len(), 0);
                let entries = lines.map(|l| DirectoryEntry::parse(l)).collect();
                List(entries)
            }
            _ => panic!("bad command: {:?}", input),
        }
    }
}

#[derive(Debug)]
enum DirectoryEntry<'a> {
    Directory(&'a str),
    File(&'a str, usize),
}

impl DirectoryEntry<'_> {
    fn parse<'a>(input: &'a str) -> DirectoryEntry<'a> {
        use DirectoryEntry::*;

        let parts: Vec<&str> = input.split(' ').collect();
        match parts[..] {
            ["dir", name] => Directory(name),
            [file_size, name] => File(name, file_size.parse::<usize>().unwrap()),
            _ => panic!("bad entry: {:?}", input),
        }
    }
}

type Path = Vec<String>;

fn construct_path(path: &Path, name: &str) -> Path {
    let mut new_path = path.clone();
    new_path.push(name.to_string());
    new_path
}

#[derive(Debug)]
struct Interpreter {
    current_path: Path,
    file_system: FileSystem,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            current_path: vec![],
            file_system: FileSystem::new(),
        }
    }

    fn execute_commands(&mut self, commands: &[Command]) {
        for command in commands {
            self.execute_command(command);
        }
    }

    fn execute_command(&mut self, command: &Command) {
        use Command::*;

        println!("executing command: {:?}", command);

        match command {
            ChangeDirectory("/") => {
                self.current_path = vec!["/".to_string()];
            }
            ChangeDirectory("..") => {
                self.current_path.pop();
            }
            ChangeDirectory(dir_name) => {
                self.current_path.push(dir_name.to_string());
            }
            List(contents) => {
                self.file_system
                    .set_directory_contents(&self.current_path, contents);
            }
        }

        println!("new interpreter state: {:?}", self);
    }

    fn directory_sizes(&mut self) -> HashMap<Path, usize> {
        self.file_system.directory_sizes()
    }
}

#[derive(Debug)]
enum FileSystemEntryType {
    Directory,
    File,
}

#[derive(Debug)]
struct FileSystem {
    directory_contents: HashMap<Path, Vec<(FileSystemEntryType, Path)>>,
    entry_sizes: HashMap<Path, usize>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            directory_contents: HashMap::new(),
            entry_sizes: HashMap::new(),
        }
    }

    fn set_directory_contents(&mut self, path: &Path, contents: &[DirectoryEntry]) {
        let entries = contents
            .iter()
            .map(|entry| match entry {
                DirectoryEntry::Directory(name) => {
                    let dir_path = construct_path(path, name);
                    (FileSystemEntryType::Directory, dir_path)
                }
                DirectoryEntry::File(name, size) => {
                    let file_path = construct_path(path, name);
                    self.entry_sizes.insert(file_path.clone(), *size); // cache file size
                    (FileSystemEntryType::File, file_path)
                }
            })
            .collect();
        self.directory_contents.insert(path.clone(), entries);
    }

    fn directory_sizes(&mut self) -> HashMap<Path, usize> {
        self.directory_contents
            .keys()
            .cloned()
            .map(|path| {
                let size =
                    Self::directory_size(&self.directory_contents, &mut self.entry_sizes, &path);
                (path, size)
            })
            .collect()
    }

    fn directory_size(
        directory_contents: &HashMap<Path, Vec<(FileSystemEntryType, Path)>>,
        entry_sizes: &mut HashMap<Path, usize>,
        path: &Path,
    ) -> usize {
        use FileSystemEntryType::*;

        if entry_sizes.contains_key(path) {
            *entry_sizes.get(path).unwrap() // use file size, or cached directory size
        } else {
            let size = directory_contents
                .get(path)
                .unwrap()
                .iter()
                .map(|(e_type, e_path)| match e_type {
                    Directory => Self::directory_size(directory_contents, entry_sizes, e_path),
                    File => *entry_sizes.get(e_path).unwrap(),
                })
                .sum();
            entry_sizes.insert(path.clone(), size); // cache for next time
            size
        }
    }
}

fn part1(input: &str) -> usize {
    let commands = Command::parse_commands(input);
    println!("commands: {:?}", commands);

    let mut interpreter = Interpreter::new();
    interpreter.execute_commands(&commands);
    println!("final interpreter state: {:?}", interpreter);

    let dir_sizes = interpreter.directory_sizes();
    println!("directory sizes: {:?}", dir_sizes);

    dir_sizes
        .into_iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum()
}

fn part2(input: &str) -> usize {
    let commands = Command::parse_commands(input);
    println!("commands: {:?}", commands);

    let mut interpreter = Interpreter::new();
    interpreter.execute_commands(&commands);
    println!("final interpreter state: {:?}", interpreter);

    let dir_sizes = interpreter.directory_sizes();
    println!("directory sizes: {:?}", dir_sizes);

    let root_size = dir_sizes.get(&vec!["/".to_string()]).unwrap();
    println!("root size: {:?}", root_size);

    let free_space = 70000000 - root_size;
    println!("free space: {:?}", free_space);

    let need_space = 30000000 - free_space;
    println!("need space: {:?}", need_space);

    dir_sizes
        .into_iter()
        .filter(|(_, size)| *size > need_space)
        .map(|(_, size)| size)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        $ cd /
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
        7214296 k
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1648397);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 24933642);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1815525);
    }
}
