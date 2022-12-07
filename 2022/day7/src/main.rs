use std::collections::VecDeque;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let root = Fs::build(input);
    let root_size = root.size();
    let Fs::Dir { children, .. } = root else {
        unreachable!("Input was malformed.");
    };

    let mut sum = 0;
    let mut queue = Vec::new();
    queue.extend(children);
    while let Some(child) = queue.pop() {
        match child {
            Fs::File { .. } => continue,
            Fs::Dir { ref children, .. } => queue.extend(children.clone()),
        }

        let size = child.size();
        if size < 100_000 {
            sum += size;
        }
    }

    if root_size < 100_000 {
        sum += root_size;
    }

    let p1 = sum;
    (p1, 0)
}

#[derive(Clone)]
enum Fs {
    File { name: String, size: usize },
    Dir { name: String, children: Vec<Fs> },
}

impl Fs {
    fn build(input: impl AsRef<str>) -> Self {
        let mut root = Fs::Dir {
            name: "/".into(),
            children: Vec::new(),
        };
        let mut current = &mut root;

        let lines: Vec<&str> = input.as_ref().lines().collect();
        let mut dir = vec!["/"];

        let mut idx = 0;
        while idx < lines.len() {
            let cmd = lines[idx].strip_prefix("$ ").unwrap();
            if cmd.starts_with("cd") {
                let cd = cmd.split_once(' ').unwrap().1;

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

            while idx + 1 < lines.len() && !lines[idx + 1].starts_with("$ ") {
                idx += 1;
                let child = lines[idx];
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

                current.insert(fs);
            }

            idx += 1;
        }

        root
    }

    fn insert(&mut self, file: Fs) {
        let Fs::Dir { children: c, .. } = self else {
            return;
        };

        c.push(file);
    }

    fn name(&self) -> &str {
        match self {
            Fs::File { name: n, .. } => n,
            Fs::Dir { name: n, .. } => n,
        }
    }

    fn get_child<'a>(&'a mut self, name: &str) -> &'a mut Fs {
        match self {
            Fs::Dir { children, .. } => {
                for child in children {
                    if child.name() == name {
                        return child;
                    }
                }

                panic!("Child not found");
            }
            _ => panic!("Cannot get children from Fs::File!"),
        }
    }

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
        assert_eq!((95437, 0), solve(PROVIDED));
    }
}
