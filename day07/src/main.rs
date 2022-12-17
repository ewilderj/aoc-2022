use id_tree::InsertBehavior::*;
use id_tree::*;
use std::cell::{RefCell, RefMut};
use std::iter::Peekable;


#[derive(Debug, PartialEq, Clone)]
struct FileNode {
    name: String,
    size: u32,
    dir: bool,
}

impl FileNode {
    fn new(n: &str, s: u32, d: bool) -> FileNode {
        FileNode {
            name: n.to_string(),
            size: s,
            dir: d,
        }
    }
}

// use refcells to easily mutate data inside an immutable structure
type MyTree = Tree<RefCell<FileNode>>;

fn process_line<'b, 'a>(
    cwd: &NodeId,
    inp: &mut Peekable<impl Iterator<Item = &'b str>>,
    s: &str,
    t: &'a mut MyTree,
) -> Option<&'a NodeId> {
    // println!("{}", s);
    if s == "$ ls" {
        // add nodes as children of cwd
        while inp.peek() != None && !inp.peek().unwrap().starts_with('$') {
            let d: Vec<&str> = inp.next().unwrap().split_whitespace().collect();
            let fname = d[1];
            let (isdir, sz) = match d[0].parse::<u32>() {
                Ok(i) => (false, i),
                _ => (true, 0),
            };
            let n = Node::new(RefCell::new(FileNode::new(fname, sz, isdir)));
            let _ = &t.insert(n, UnderNode(cwd)).unwrap();
        }
        // println!("{:#?}", t);
        return None;
    }
    // other case, it's a directory
    let d = s.split_whitespace().nth(2).unwrap();
    // println!("CD {}", d);
    if d == ".." {
        let cnode = &t.get(cwd).unwrap();
        return Some(cnode.parent().unwrap());
    } else {
        // look for child node with data().name == d
        let c: &NodeId = &t
            .children_ids(cwd)
            .unwrap()
            .filter(|k| &t.get(k).unwrap().data().borrow().name == d)
            .next()
            .unwrap();
        // println!("cd into {} {:#?}", d, c);
        return Some(c);
    }
}

fn main() {
    let mut inp = include_str!("../input.txt").lines().peekable();
    let mut fs: MyTree = Tree::new();
    let _ = inp.next(); // skip "cd /" as implicit at the top of each input
    let mut cwd: NodeId = fs
        .insert(Node::new(RefCell::new(FileNode::new("/", 0, true))), AsRoot)
        .unwrap();
    let root: NodeId = cwd.clone();
    //   println!("{:#?}", &fs.get(&cwd).unwrap());
    while let Some(s) = inp.next() {
        if s.starts_with('$') {
            if let Some(nd) = process_line(&cwd, &mut inp, s, &mut fs) {
                cwd = nd.clone(); // avoids borrowing fs
                                  // println!("NEW CWD IS {:#?}", &fs.get(&cwd).unwrap());
            }
        }
    }

    // depth-first enumeration of directories
    let node_ids = fs
        .traverse_post_order_ids(&root)
        .unwrap()
        .filter(|n| fs.get(n).unwrap().data().borrow().dir);

    // compute directory size
    let mut v: Vec<u32> = node_ids.clone().map (|n| {
        let s: u32 = fs
            .children(&n)
            .unwrap()
            .map(|c| c.data().borrow().size)
            .sum::<u32>();
        fs.get(&n).unwrap().data().borrow_mut().size = s;
        s
    }).collect();
    v.sort();

    let p1: u32 = v.iter().filter(|&s| *s <= 100000).sum();
    println!("part1: {}", p1);

    let root_size = fs.get(&root).unwrap().data().borrow().size;
    let needed_space = 30000000 - (70000000 - root_size);
    let p2: u32 = *v.iter().filter(|&s| *s >= needed_space).next().unwrap();
    println!("part2: {}", p2);
}
