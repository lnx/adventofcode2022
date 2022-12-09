use std::cell::Cell;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct File {
    kind: Kind,
    path: String,
    size: Cell<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Dir,
    File,
}

impl File {
    fn new(kind: Kind, path: &str, size: u32) -> Self {
        Self { kind, path: path.to_string(), size: Cell::new(size) }
    }
}

fn get_parent(abspath: &str) -> Option<String> {
    if abspath == "/" {
        return None;
    }

    abspath.rsplitn(2, '/').skip(1).next().map(|p| {
        if p.is_empty() {
            "/".to_string()
        } else {
            p.to_string()
        }
    })
}

fn get_abspath(dir: &str, name: &str) -> String {
    dir.trim_end_matches('/').to_string() + "/" + name
}

fn build(input: &str) -> (HashMap<String, File>, HashMap<String, HashSet<String>>) {
    let mut files = HashMap::new();
    let mut children = HashMap::new();
    let mut dir = "".to_string();
    for line in input.lines() {
        if line.starts_with("$ cd ") { // command cd
            let name = line.trim_start_matches("$ cd ");
            if dir == "" {
                dir = name.to_string();
            } else if name == ".." {
                dir = get_parent(&dir).unwrap()
            } else {
                dir = get_abspath(&dir, name);
            }
        } else if line.starts_with("$ ls") { // command ls
            // do nothing
        } else if line.starts_with("dir ") { // dir
            let name = line.trim_start_matches("dir ");
            let abspath = get_abspath(&dir, name);
            let file = File::new(Kind::Dir, &abspath, 0);
            files.insert(abspath.clone(), file);
            children.entry(dir.clone()).or_insert(HashSet::new()).insert(abspath.clone());
        } else { // file
            // println!("line:{:?}", line);
            let mut split = line.split(" ");
            let size = split.next().unwrap().parse::<u32>().unwrap();
            let name = split.next().unwrap();
            let abspath = get_abspath(&dir, name);
            let file = File::new(Kind::File, &abspath, size);
            files.insert(abspath.clone(), file);
            children.entry(dir.clone()).or_insert(HashSet::new()).insert(abspath.clone());
        }
    }
    files.insert("/".to_string(), File::new(Kind::Dir, "/", 0));
    (files, children)
}

fn calculate_size(path: &str, files: &HashMap<String, File>, children: &HashMap<String, HashSet<String>>) -> u32 {
    let file = files.get(path).unwrap();
    match file.kind {
        Kind::Dir => {
            if file.size.get() == 0 {
                let mut sum = 0;
                for child in children.get(&file.path).unwrap() {
                    sum += calculate_size(child, files, children);
                }
                file.size.set(sum);
            }
            file.size.get()
        }
        Kind::File => file.size.get(),
    }
}

fn puzzle1(input: &str) -> u32 {
    let (files, children) = build(&input);
    let _ = calculate_size("/", &files, &children);
    files.iter().map(|(_, f)| f)
        .filter(|f| f.kind == Kind::Dir && f.size.get() < 100000)
        .map(|f| f.size.get())
        .sum()
}

fn puzzle2(input: &str) -> u32 {
    let (files, children) = build(&input);
    let _ = calculate_size("/", &files, &children);
    let available = 70000000 - files.get("/").unwrap().size.get();
    let freeup = max(30000000 - available, 0);
    files.iter().map(|(_, f)| f)
        .filter(|f| f.kind == Kind::Dir && f.size.get() >= freeup)
        .map(|f| f.size.get()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "$ cd /
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
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 95437);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 24933642);
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}
