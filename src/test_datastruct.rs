use  std::collections::{ BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use  std::cmp::Reverse;

fn test_datastruct_1() {

    let mut v = vec![10, 20, 30];

    println!("v:{v:?}");

    v.push(33);
    println!("v:{v:?} len:{:?}", v.len());

    v.pop();
    println!("v:{v:?} len:{:?}", v.len());

    println!("index 1:{:?} 6:{:?}", v.get(1), v.get(6));
    println!("first:{:?} last:{:?}",v.first(),v.last());

    v.insert(1, 4);
    println!("v:{v:?}");

    v.remove(1);
    println!("v:{v:?}");

    v.swap_remove(1);
    println!("v:{v:?}");

    v.retain(|x| *x % 2 == 0);
    println!("v:{v:?}");

    let removed:Vec<i32> = v.drain(1..2).collect();
    println!("v:{v:?}");
    println!("removed:{removed:?}");

    v.extend_from_slice(&[10,203,04,040]);
    let (a,b) = v.split_at(3);

    println!("a:{a:?}");
    println!("b:{b:?}");



}

fn test_datastruct_2() {
    let mut q = VecDeque::new();

    q.push_back(10);
    q.push_back(20);
    q.push_back(30);

    println!("q:{q:?}");

    let x = q.pop_back();
    println!("q:{q:?} x:{x:?}");



    let mut stack = Vec::<i32>::new();
    stack.push(2);
    stack.push(3);
    stack.push(4);
    println!("stack:{stack:?}");

    stack.pop();
    println!("stack:{stack:?}");

    stack.pop();
    println!("stack:{stack:?}");

    stack.pop();
    println!("stack:{stack:?}");

    let mut map  = HashMap::new();

    map.insert(0x401000, "main".to_string());
    map.insert(0x402000, "foo".to_string());

    let addr = 0x401000;

    if let Some(name) = map.get_mut(&addr) {
        name.push_str("@1");
    }    
    if let Some(name) = map.get(&addr) {
        println!("addr:{addr:#08X} name:{name}");
    }

    let mut map = HashMap::new();
    let names:[String;10] = std::array::from_fn(|i| i.to_string());
    for name in names {
        *map.entry(name).or_insert(0) += 1;
    }
    println!("map:{map:?}");

    let mut visited = HashSet::new();
    visited.insert(0x401000);
    visited.insert(0x402000);

    if visited.contains(&addr) {
        println!("contain:{addr:08X}");
    }

    if !visited.insert(0x401000) {
        println!("already has 0x401000");
    }


    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(2);

    while let Some(addr) = queue.pop_front() {
        if !visited.insert(addr) {
            continue;
        }

        for next in &[10,20,30,405,0]{
            queue.push_back(*next);
        }
    }

    let mut map = BTreeMap::new();
    map.insert(0x3000, "C");
    map.insert(0x1000, "A");
    map.insert(0x2000, "B");

    for (addr, name) in &map {
        println!("{addr:#x}: {name}");
    }


    for (addr, name) in map.range(0..=0x2000) {
        println!("{addr:#x}: {name}");
    }

    let mut addresses = BTreeSet::new();

    addresses.insert(0x3000);
    addresses.insert(0x1000);
    addresses.insert(0x2000);

    println!("addresses:{addresses:?}");

    let mut heap = BinaryHeap::new();
    heap.push(10);
    heap.push(100);
    heap.push(20);

    println!("{:?}", heap.pop());
    println!("heap:{heap:?}");


    let mut heap = BinaryHeap::new();

    heap.push(Reverse(10));
    heap.push(Reverse(100));
    heap.push(Reverse(20));

    println!("{:?}", heap.pop());
    println!("heap:{heap:?}");


    #[derive(Debug, Eq, PartialEq)]
    struct Task {
        priority:u32,
        name: String,
    }
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut tasks = BinaryHeap::new();
    tasks.push(Task { priority: 1, name: "低".into() });
    tasks.push(Task { priority: 10, name: "高".into() });
    tasks.push(Task { priority: 5, name: "中".into() });

    while let Some(t) = tasks.pop() {
        println!("{}", t.name);  // 高、中、低
    }

}

pub(crate) fn test_datastruct_all() {
    // test_datastruct_1();
    test_datastruct_2();
}