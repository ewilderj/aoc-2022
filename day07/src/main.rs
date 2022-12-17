use id_tree::InsertBehavior::*;
use id_tree::*;
use std::cell::{RefCell, Ref, RefMut};
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

// and hide some of the repetition with convenience functions
trait EasyFileNode {
    fn filenode(&self, n: &NodeId) -> Ref<FileNode>;
    fn filenode_mut(&self, n: &NodeId) -> RefMut<FileNode>;
}

impl EasyFileNode for MyTree {
    fn filenode(&self, n: &NodeId) -> Ref<FileNode> {
        return self.get(n).unwrap().data().borrow();
    }
    fn filenode_mut(&self, n: &NodeId) -> RefMut<FileNode> {
        return self.get(n).unwrap().data().borrow_mut();
    }
}

fn process_line<'b, 'a>(
    cwd: &NodeId,
    inp: &mut Peekable<impl Iterator<Item = &'b str>>,
    s: &'b str,
    t: &'a mut MyTree,
) -> Option<&'a NodeId> {
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
        return None;
    }

    // other case, it's a directory
    let d = s.split_whitespace().nth(2).unwrap();
    if d == ".." {
        let cnode = &t.get(cwd).unwrap();
        return Some(cnode.parent().unwrap());
    } else {
        // look for child node with data().name == d
        let c: &NodeId = &t
            .children_ids(cwd)
            .unwrap()
            .filter(|k| &t.filenode(k).name == d)
            .next()
            .unwrap();
        // returning a value changes the cwd
        return Some(c);
    }
}

fn main() {
    let mut inp = include_str!("../input.txt").lines().peekable();
    let mut fs: MyTree = Tree::new();
    let _ = inp.next(); // skip "cd /" as implicit at the top of each input

    // create our root node
    let mut cwd: NodeId = fs
        .insert(Node::new(RefCell::new(FileNode::new("/", 0, true))), AsRoot)
        .unwrap();
    let root: NodeId = cwd.clone();

    while let Some(s) = inp.next() {
        if s.starts_with('$') {
            // process line, and if it's a "cd", change the cwd
            if let Some(nd) = process_line(&cwd, &mut inp, s, &mut fs) {
                cwd = nd.clone();
            }
        }
    }

    // make a depth-first iterator over directory node_ids
    let node_ids = fs
        .traverse_post_order_ids(&root)
        .unwrap()
        .filter(|n| fs.filenode(n).dir);

    // compute directory sizes and collect them
    let mut v: Vec<u32> = node_ids
        .map(|n| {
            let s: u32 = fs
                .children(&n)
                .unwrap()
                .map(|c| c.data().borrow().size)
                .sum::<u32>();
            fs.filenode_mut(&n).size = s;
            s
        })
        .collect();

    // sort vector as it makes part2 easy
    v.sort();

    let p1: u32 = v.iter().filter(|&s| *s <= 100000).sum();
    println!("part1: {}", p1);

    let needed_space = fs.filenode(&root).size - 40000000;
    let p2: u32 = *v.iter().filter(|&s| *s >= needed_space).next().unwrap();
    println!("part2: {}", p2);
}
