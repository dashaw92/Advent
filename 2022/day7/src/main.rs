use std::error::Error;
use std::fs::read_to_string;

const P1_SIZE_MAX: usize = 100_000;
const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    //I need the overall size for part 2, so before destructuring it,
    //store the size
    let root = Fs::build(input);

    //List of directory sizes in the Fs
    let mut dir_sizes = vec![root.size()];

    //Fs::build always returns Fs::Dir ("/" is the root of the Fs)
    let Fs::Dir { children, .. } = root else {
        unreachable!("Input was malformed.");
    };

    //Traverse the FS 😄
    let mut queue = Vec::new();
    queue.extend(children);
    while let Some(child) = queue.pop() {
        let size = child.size();
        match child {
            //Both parts don't need Files directly, they're only used to add
            //size to their parent Dirs
            Fs::File { .. } => continue,
            //Add children to the stack for traversal
            Fs::Dir { children, .. } => queue.extend(children),
        };

        //Only reachable when `child` is a Fs::Dir.
        //Structured this way to avoid cloning the `children` vector
        dir_sizes.push(size);
    }

    //Find only directories that are less than P1_SIZE_MAX in size, and sum them up
    let p1 = dir_sizes.iter().filter(|&size| *size < P1_SIZE_MAX).sum();

    let used = TOTAL_SPACE - dir_sizes[0];
    let needed = NEEDED_SPACE - used;

    //Find directories that are greater in size than `needed`, then get
    //the smallest directory from that
    let p2 = *dir_sizes
        .iter()
        .filter(|&size| *size >= needed)
        .min()
        .unwrap();

    (p1, p2)
}

enum Fs {
    File { name: String, size: usize },
    Dir { name: String, children: Vec<Fs> },
}

impl Fs {
    //wew 😅
    fn build(input: impl AsRef<str>) -> Self {
        //Base of the filesystem
        let mut root = Fs::Dir {
            name: "/".into(),
            children: Vec::new(),
        };

        //Moving pointer to current position in the filesystem
        let mut current = &mut root;
        //Stringification of `current` - Fs does not track
        //parents, so this is how I will keep track of lineage
        let mut dir = vec!["/"];

        let lines: Vec<&str> = input.as_ref().lines().collect();

        //position in input.
        let mut idx = 0;
        while idx < lines.len() {
            //Strategy: cmd will *always* be a command string: ["$ cd <arg>" | "$ ls"]
            //The input always starts with a command, which is excellent for bootstrapping the loop
            //If the cmd is "cd", figure out the new value of `current` and `dir`, then continue to the
            //next line, which will always be a command.
            let cmd = lines[idx].strip_prefix("$ ").unwrap();
            if cmd.starts_with("cd") {
                let cd = cmd.split_once(' ').unwrap().1;

                //Since Fs's don't keep track of their parents,
                //the implementation of ".." is quite silly:
                //Start at root, then iterate from n = [1..]
                //and set `current` to be child.children[n]
                match cd {
                    "/" => {
                        dir.clear();
                        dir.push("/");
                        current = &mut root;
                    }
                    ".." => {
                        dir.pop();
                        current = &mut root;
                        for child in &dir[1..] {
                            current = current.get_child(child);
                        }
                    }
                    _ => {
                        dir.push(cd);
                        current = current.get_child(cd);
                    }
                }

                idx += 1;
                continue;
            }

            //Greedily consume the output of the "ls" command until the next line
            //is EOF or another command, meaning we've parsed the "ls" output entirely
            while idx + 1 < lines.len() && !lines[idx + 1].starts_with("$ ") {
                idx += 1;
                let child = lines[idx];

                //will either be "dir <name>" or "<size> <name>"
                let (ty, name) = child.split_once(' ').unwrap();

                let fs = match ty {
                    "dir" => Fs::Dir {
                        name: name.into(),
                        children: Vec::new(),
                    },
                    size => Fs::File {
                        name: name.into(),
                        size: size.parse().unwrap(),
                    },
                };

                //Since `current` is always set to the correct position in the
                //filesystem, just directly insert the parsed Fs into `current`'s
                //children list
                current.insert(fs);
            }

            idx += 1;
        }

        root
    }

    //Unwrap an Fs::Dir and push the `file` into its children
    fn insert(&mut self, file: Fs) {
        let Fs::Dir { children: c, .. } = self else {
            unreachable!("Malformed input! Fs::File has no children!");
        };

        c.push(file);
    }

    //Helper for unwrapping the Fs instance
    fn name(&self) -> &str {
        match self {
            Fs::File { name: n, .. } => n,
            Fs::Dir { name: n, .. } => n,
        }
    }

    //Iterate over self's children to find the `child` named `name`.
    fn get_child<'a>(&'a mut self, name: &str) -> &'a mut Fs {
        match self {
            Fs::Dir { children, .. } => children
                .iter_mut()
                .find(|child| child.name() == name)
                .expect("Child not found"),
            _ => panic!("Cannot get children from Fs::File!"),
        }
    }

    //The size of a Fs is:
    // - the size directly if self is a File
    // - the sum of all children Fs if self is a Dir
    fn size(&self) -> usize {
        match self {
            Fs::File { size, .. } => *size,
            Fs::Dir { children, .. } => children.iter().map(Fs::size).sum(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "$ cd /
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

    #[test]
    fn provided_p1() {
        assert_eq!((95437, 24933642), solve(PROVIDED));
    }
}
