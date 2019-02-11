extern crate crypto;

use std::collections::HashMap;
use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

enum Node {
    Branch([String; 17]),
    Flag((Vec<u8>, String)),
    Null(),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Null() => write!(f, "[Null Node]"),
            Node::Branch(branch) => {
                write!(f, "Branch[");
                for i in 0..16 {
                    write!(f, "{}=\"{}\", ", i, branch[i]);
                }
                write!(f, "value={}]", branch[16])
            },
            Node::Flag((encoded_prefix, value)) => {
                write!(f, "{}<{:?}, value=\"{}\">",
                       if is_ext_node(encoded_prefix.to_vec()) {"Ext"} else {"Leaf"},
                       compact_decode(encoded_prefix.to_vec()),
                       value)
            }
        }
    }
}

impl Node {
    fn clone(&self) -> Node {
        match self {
            Node::Flag((prefix, value)) => Node::Flag((prefix.to_vec(), value.to_string())),
            Node::Branch(branch) => {
                let mut value: [String; 17] = empty_branch_value();
                for i in 0..17 {
                    value[i] = branch[i].to_string();
                }
                Node::Branch(value)
            },
            _ => Node::Null(),
        }
    }
}

#[derive(Debug)]
struct MerklePatriciaTrie {
    db: HashMap<String, Node>,
    root: String,
}

impl MerklePatriciaTrie {
    fn new() -> MerklePatriciaTrie {
        let mut mpt = MerklePatriciaTrie {db: HashMap::new(), root: String::new()};
        mpt.db.insert(String::new(), Node::Null());
        mpt
    }

    fn clone(&self) -> MerklePatriciaTrie {
        let mut hashmap = HashMap::new();
        for (k, v) in self.db.iter() {
            hashmap.insert((*k).to_string(), (*v).clone());
        }
        MerklePatriciaTrie {db: hashmap, root: self.root.to_string()}
    }

    fn get(&mut self, key: &str) -> String {
        // TODO
    }

    fn insert(&mut self, key: &str, new_value: &str) {
        // TODO
    }

    fn delete(&mut self, key: &str) -> String {
        // TODO
    }

    fn mpt_to_string(&self) -> String {
        let mut content: String = String::new();
        content = content + &*format!("ROOT={}\n", self.root);
        for (hash, node) in &self.db {
            content = content + &*format!("{}: {:?}\n", hash, node);
        }
        content
    }

    fn print(&self) {
        println!("{}", self.mpt_to_string());
    }

    fn order_nodes(&self) -> String {
        let raw_content = self.mpt_to_string();
        let content: Vec<&str> = raw_content.split("\n").collect();
        let mut queue: Vec<&str> = Vec::new();
        let mut temp1 = content[0];
        let mut temp2: Vec<&str> = temp1.split("HashStart").collect();
        temp1 = temp2[1];
        temp2 = temp1.split("HashEnd").collect();
        temp1 = temp2[0];
        queue.push(temp1);
        let mut i = -1;
        let mut rs = String::new();
        while let Some(cur_hash) = queue.pop() {
            i += 1;
            println!("cur={}", cur_hash);
            let mut line: &str = "";
            for each in &content {
                if each.starts_with(&*(format!("HashStart{}HashEnd", cur_hash))) {
                    temp2 = each.split("HashEnd: ").collect();
                    line = temp2[1];
                    rs = rs + each + "\n";
                    rs = rs.replace(&*(format!("HashStart{}HashEnd", cur_hash)), &*(format!("Hash{}", i)));
                }
            }
            temp2 = line.split("HashStart").collect();
            let mut flag = true;
            for each in temp2 {
                if flag {
                    flag = false;
                    continue
                }
                let temp3: Vec<&str> = each.split("HashEnd").collect();
                queue.push(temp3[0]);
            }
        }
        rs
    }
}

fn compact_encode(hex_array: Vec<u8>) -> Vec<u8> {
    // TODO
}

// If Leaf, ignore 16 at the end
fn compact_decode(encoded_arr: Vec<u8>) -> Vec<u8> {
    // TODO
}

fn is_ext_node(encoded_arr: Vec<u8>) -> bool {
    (encoded_arr[0] as i32) / 16 < 2
}

fn test_compact_encode() {
    assert_eq!(compact_decode(compact_encode(vec![1, 2, 3, 4, 5])),
               vec![1, 2, 3, 4, 5]);
    assert_eq!(compact_decode(compact_encode(vec![0, 1, 2, 3, 4, 5])),
               vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(compact_decode(compact_encode(vec![0, 15, 1, 12, 11, 8, 16])),
               vec![0, 15, 1, 12, 11, 8]);
    assert_eq!(compact_decode(compact_encode(vec![15, 1, 12, 11, 8, 16])),
               vec![15, 1, 12, 11, 8]);
}

fn hash_node(node: &Node) -> String {
    let mut hasher = Sha3::sha3_256();
    match node {
        Node::Branch(branch) => {
            let mut input = String::from("branch_");
            for each in branch {
                input += &*each;
            }
            hasher.input_str(&*input);
        },
        Node::Flag((encoded_prefix, value)) => {hasher.input_str(&*value);},
        Node::Null() => {hasher.input_str("");},
    }
    String::from("HashStart_") + &*(hasher.result_str()) + "_HashEnd"
}

fn empty_branch_value() -> [String; 17] {
    [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(), String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(), String::new(), String::new()]
}