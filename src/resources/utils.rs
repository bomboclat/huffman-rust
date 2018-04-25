use std::collections::VecDeque;

///////////////// STRUCTS //////////////////
#[derive(PartialEq)]
pub struct Node {
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
    pub count: Option<u64>,
    pub value: Option<char>
}

//////////////// FUNCTIONS /////////////////

pub fn quick_sort (vector: Vec<Node>) -> Vec<Node> {
    if vector.len() <= 1 {
        return vector;
    } else {
        let mut count = 0;
        let mut a1 = Vec::new();
        let mut a2 = Vec::new();
        let mut pivot_el = Node {
            l: None,
            r: None,
            count: None,
            value: None
        };
        let mut pivot: Option<u64> = Some(0);
        for el in vector {
            if count == 0 {
                pivot_el = el;
                pivot = pivot_el.count;
            } else {
                let frequency = el.count;
                if frequency >= pivot {
                    a1.push(el);
                } else if frequency <= pivot {
                    a2.push(el);
                }
            }
            count += 1;
        }
        
        /////////// RECURSIVE ///////////
        let mut b1 = quick_sort(a1);
        let b2 = quick_sort(a2);
        b1.push(pivot_el);
        b1.extend(b2);
        return b1;
    }
}

pub fn bit_to_byte(file: &mut Vec<u8>) -> Vec<u8> {
    let x: u8 = 2;
    let mut lenght = file.len();
    let byte_compl = 8 - (lenght % 8);
    for _i in 0..byte_compl {file.push(0)}
    lenght = file.len()+1;
    let mut bytes = Vec::with_capacity(lenght/8+1);
    let mut index = 8;
    while index <= lenght {
        let mut count: i32 = 7;
        let mut result = 0;
        let v1 = &file[(index - 8)..index];
        for i in v1 {
            if count > 0 {
                result = result + (i * x << count-1);
                count -= 1;
            } else {
                result = result + i;
            }
        }
        index += 8;
        bytes.push(result);
    }
    bytes.push(byte_compl as u8);
    return bytes
}

pub fn byte_to_bit(file: &mut VecDeque<u8>) -> VecDeque<u8> {
    let byte_compl = file.pop_back().unwrap();
    let lenght = file.len() * 8 - 1;
    let mut new_file = VecDeque::with_capacity(lenght);
    for i in file {
        let mut arr: [u8; 8] = [0; 8];
        let mut el = *i;
        let mut count = 7;
        while el > 0 {
            let num = el >> 1;
            let res = el - (num << 1);
            arr[count as usize] = res;
            el = num;
            count -= 1;
        }
        for x in arr.iter() {
            new_file.push_back(*x);
        }
    }
    for _i in 0..byte_compl {new_file.pop_back();};
    return new_file;
}

pub fn serialize_tree(tree: Node) -> String {
    let mut serialized = String::new();
    if tree.value != None {
        serialized.push('>');
        serialized.push(tree.value.unwrap());
    } else {
        serialized.push('#');
        serialized.push_str(&serialize_tree(*tree.l.unwrap()));
        serialized.push_str(&serialize_tree(*tree.r.unwrap()));
    }
    return serialized;
}

pub fn deserialize_tree(mut file: &mut VecDeque<u8>) -> Node {
    let marker = file.pop_front().unwrap() as char;
    let mut node = Node {
            l: None,
            r: None,
            count: None,
            value: None
        };
    if marker == '>' {
        let letter = file.pop_front().unwrap() as char;
        node.value = Some(letter);
    } else if marker == '#' {
        node.l = Some(Box::new(deserialize_tree(&mut file)));
        node.r = Some(Box::new(deserialize_tree(&mut file)));
    }
    return node;
}