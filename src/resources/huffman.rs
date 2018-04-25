use resources::utils::*;

pub fn create_base (string: &Vec<u8>) -> Vec<Node> {
    let mut x: [u64; 255] = [0; 255];
    let mut count: u8 = 0;
    let mut base = Vec::new();
    for el in string {
        x[*el as usize] += 1;
    }
    for freq in x.iter() {
        if freq > &0 {
            let leaf = Node {
                l: None,
                r: None,
                count: Some(*freq),
                value: Some(count as char)
            };
            base.push(leaf);
        }
        count += 1;
    }
    return quick_sort(base);
}

pub fn build (mut base: Vec<Node>) -> Node {
    let n1 = base.pop().unwrap();
    let n2 = base.pop().unwrap();
    let freq = n1.count.unwrap().checked_add(n2.count.unwrap());
    let n3 = Box::new(n1);
    let n4 = Box::new(n2);
    let node = Node {
        l: Some(n3),
        r: Some(n4),
        count: freq,
        value: None
    };
    base.push(node);
    if base.len() == 1 {
        return base.pop().unwrap();
    } else {
        let ordered_base = quick_sort(base);
        /////////// RECURSIVE ///////////
        build(ordered_base)
    }
}

pub fn schema_v2(tree: &Node, route: Vec<u8>) -> Vec<Vec<u8>>{
    let schema = schema_serializer(tree, route);
    let mut new_schema = vec![vec![0]; 255];
    for mut el in schema {
        let marker = el.pop().unwrap();
        new_schema[marker as usize] = el;
    }
    return new_schema;
}

fn schema_serializer(tree: &Node, mut route: Vec<u8>) -> Vec<Vec<u8>> {
    let mut schema = Vec::new();
    if tree.value != None {
        route.push(tree.value.unwrap() as u8);
        schema.push(route);
    } else {
        let mut v1 = route.clone();
        let mut v2 = route;
        v1.push(0);
        v2.push(1);
        if let Some(ref x) = tree.l {
            schema.extend(schema_serializer(&x, v1));
        }
        if let Some(ref y) = tree.r {
            schema.extend(schema_serializer(&y, v2));
        }
    }
    return schema;
}
