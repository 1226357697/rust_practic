use std::rc::Rc;
use std::cell::{RefCell};
use  std::sync::{Arc, Mutex,RwLock,Weak};

// Box<T>
// = 单 owner

// Rc<T>
// = 多 owner，单线程

// Arc<T>
// = 多 owner，多线程

// 普通 &mut
// = 编译期独占修改

// RefCell<T>
// = 单线程 runtime borrow checking

// Mutex<T>
// = 多线程互斥修改

// RwLock<T>
// = 多线程多读单写

// | 需求 | 类型 |
// |---|---|
// | 单 owner | `T` |
// | 单 owner + heap | `Box<T>` |
// | 单线程多个 owner | `Rc<T>` |
// | 多线程多个 owner | `Arc<T>` |
// | 单线程共享 + 可修改 | `Rc<RefCell<T>>` |
// | 多线程共享 + 可修改 | `Arc<Mutex<T>>` |
// | 多线程共享 + 读多写少 | `Arc<RwLock<T>>` |


fn test_box_1() {
    let mut x = Box::new(10);
    *x = 10;

    let raw:* mut i32 = Box::into_raw(x);
    let x = unsafe  {*raw};
    println!("x:{x:?} raw:{raw:p}");

    let b = Box::new(42);
    println!("Box 变量地址（栈）: {:p}", &b);
    println!("Box 指向的堆地址: {:p}", &*b);
    println!("Box::as_ref 地址: {:p}", b.as_ref());

    // Box::into_raw
    let b2 = Box::new(100);
    let raw = Box::into_raw(b2);
    println!("into_raw 地址: {:p}", raw);
    let _ = unsafe { Box::from_raw(raw) };  // 还原

    // String
    let s = String::from("hello");
    println!("String 变量地址: {:p}", &s);
    println!("String 堆数据地址: {:p}", s.as_ptr());    
}

fn test_box_2() {
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("{}", Rc::strong_count(&a));

    // Error message
    // let mut x = 10;
    // let a = &mut x;
    // let b = &mut x;
    // println!("a:{a:?} b:{b:?}");

    // let v = RefCell::new(10);
    // let mut b = v.borrow_mut();
    // {
    //     let mut v = v.borrow_mut(); // crash
    //     *v += 1;

        
    // }
    // println!("{}", v.borrow());
    // println!("{}", *b);


    struct Node {
        value: i32,
        children: Vec<Rc<RefCell<Node>>>,
    }    

    let data = Arc::new(String::from("hello"));

    let a = Arc::clone(&data);
    let b = Arc::clone(&data);

    let value = Mutex::new(10);

    {
        let mut guard = value.lock().unwrap();
        *guard += 1;
    }

    println!("value:{value:?}");

    let counter = Arc::new(Mutex::new(0));  
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move||{
            let mut n = counter.lock().unwrap();
            *n += 1;
        });
        handles.push(handle);
    }  

    for h in handles{
        h.join().unwrap();
    }

    println!("counter:{counter:?}");

    let data = RwLock::new(vec![1, 2, 3]);
    let r1 = data.read().unwrap();
    let r2 = data.read().unwrap();

    let mut w = data.write().unwrap();
    w.push(4);  


// struct BasicBlock {
//     address: u64,

//     successors: Vec<Rc<RefCell<BasicBlock>>>,

//     predecessor: Option<Weak<RefCell<BasicBlock>>>, // 循环引用
// }    

}

pub(crate) fn test_box_all() {
    // test_box_1();
    test_box_2();
}