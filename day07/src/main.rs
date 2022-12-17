use id_tree::InsertBehavior::*;
use id_tree::*;
use std::iter::Peekable;
use std::collections::HashMap;

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

    fn size_mut(&mut self) -> &mut u32 {
        &mut self.size
    }
}

fn process_line<'b, 'a>(
    cwd: &NodeId,
    inp: &mut Peekable<impl Iterator<Item = &'b str>>,
    s: &str,
    t: &'a mut Tree<FileNode>,
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
            let n = Node::new(FileNode::new(fname, sz, isdir));
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
            .filter(|k| &t.get(k).unwrap().data().name == d)
            .next()
            .unwrap();
        // println!("cd into {} {:#?}", d, c);
        return Some(c);
    }
}

fn dump_tree(fs: &Tree<FileNode>) {
        let mut s = String::new();
    let _ = &fs.write_formatted(&mut s).unwrap();
    println!("{}", &s);

}

fn main() {
    let mut inp = include_str!("../test.txt").lines().peekable();
    let mut fs: Tree<FileNode> = Tree::new();
    let mut sizes: HashMap<u32, u32> = HashMap::new();
    let _ = inp.next(); // skip "cd /" as implicit at the top of each input
    let mut cwd: NodeId = fs
        .insert(Node::new(FileNode::new("/", 0, true)), AsRoot)
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
    dump_tree(&fs);

    // depth-first enumeration of directories
    let  node_ids = fs
        .traverse_post_order_ids(&root)
        .unwrap();


    // we're looking at each dir node
    // the sizes are the sum of the child sizes

    let mut t: u32 = 0;
    let mut wasdir: bool = true;
    for n in node_ids {
        // n is &Node
        println!("{:#?}", &n);
        let nn: &mut Node<FileNode> = fs.get_mut(&n).unwrap();
        let fnode: &mut FileNode = nn.data_mut();

        if fnode.dir {
            let mut kids = fs.children(&n).unwrap();
            fnode.size = t;
            wasdir = true;
        }
    }
    dump_tree(&fs);
}
