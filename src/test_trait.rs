use std::{fmt::Debug, ops::Deref, path::Path};


fn test_trait_1() {

    trait Printable {
        fn print(&self);

        fn debug_print(&self) {
            println!("debug");
        }

        fn info_print(&self) {
            println!("info");
        }
    }


    #[derive(Debug)]
    struct User {
        name:String
    }

    impl Printable for User {
        fn print(&self) {
            println!("{}", self.name);
            self.debug_print();
            self.info_print();
        }

        fn info_print(&self) {
            println!("user info");
        }
    }

    let u1 = User {
        name: "yan".to_string()
    };
    u1.print();

    fn to_print<T>(o:&T) 
    where T:Printable + Debug
    {
        println!("obj: {:?}", o);
        o.print();
    }

    to_print(&u1);

    fn print_value(value: &impl Printable) {
        value.print();
    }

    print_value(&u1);
    println!("u1: {:?}", u1);

    fn print_value2(value: impl Printable) {
        value.print();
    }

    print_value2(u1);
    // println!("u1: {:?}", u1); // 被移动了

    fn numbers() -> impl Iterator<Item = i32> {
        0..10
    }

    println!("number: {:?}", numbers().collect::<Vec<_>>());


    fn create() -> Box<dyn Printable> {
        Box::new(User {
            name: "Tom".into(),
        })
    }    
    create().print();
}


fn test_trait_2() {
    
    trait Parser {
        type Output;

        fn parse(&self) ->Self::Output;
    }

    struct IntParser;

    type MyInt = i32;
    impl Parser for IntParser {
        type Output = MyInt;
        fn parse(&self) ->Self::Output {
            100
        }
    }
}

fn test_trait_3() {

    trait MyPrint {
        fn my_print(&self) ;
    }

    impl<T:std::fmt::Debug> MyPrint for T  {
        fn my_print(&self) {
            println!("{self:?}");
        }
    }

    5.my_print();
    "hello".my_print();                  // "hello"
    vec![1, 2, 3].my_print();            // [1, 2, 3]    
}

fn test_trait_4() {
    let s = String::from("hello");
    let s: String = "hello".into();


    #[derive(Debug)]
    struct Address(u64);

    impl From<u64> for Address  { 
        fn from(value: u64) -> Self {
            Address(value)
        }
    }

    let a = Address::from(3);
    let b: Address = 32u64.into();
    let c =  Into::<Address>::into(32u64);
    println!("a:{:?} b:{:?} c:{:?}", a, b, c);

    #[derive(Debug)]
    struct MyError(std::io::Error);

    impl From<std::io::Error> for MyError {
        fn from(value: std::io::Error) -> Self {
            MyError(value)
        }
    }

    fn foo()->Result<(), MyError> {
        std::fs::read("12312wqweqweq.txt")?;
        Ok(())
    }
    let result = foo();
    println!("result:{:?}", result);


    fn foo1<T>(path:T)
    where T:AsRef<Path>
    {
        let s  = path.as_ref();
        println!("{s:?}");
    }

    foo1("1.txt");
    foo1("1.txt".to_string());
    foo1(Path::new("1.txt"));

    fn print<S: AsRef<str>>(s: S) {
        let s: &str = s.as_ref();
        println!("{s}");
    }
    print("hello");
    print(String::from("hello"));


    #[derive(Debug)]
    struct MyString(String);

    impl MyString {
        fn new(v:impl AsRef<str>)->Self {
            MyString(v.as_ref().to_owned())
        }
    }

    impl Deref for MyString {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            return &self.0;
        }
    }
    let s1 = MyString::new("hello world");
    let ref1:&str = &s1;
    println!("ref1:{ref1:?} size:{:?}", s1.len());

    let r1: &str = &s1;          // Deref coercion
    let r2: &str = s1.deref();   // 显式
    let r3: &str = &*s1;         // 显式解引用再取引用
    let r4: &str = s1.as_ref();  // 如果实现了 AsRef<str>

    println!("r1:{r1} r2:{r2} r3:{r3} r4:{r4}");

}

fn test_trait_5() {
    struct Counter {
        current: u32,
        end: u32,
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }

        
    }


    let counter = Counter {
        current:0,
        end:5
    };

    for x in counter {
        println!("x:{x}");
    }

    let counter = Counter {
        current:0,
        end:5
    };
    let s = counter
    .into_iter()
    .filter(|x| *x <3)
    .map(|x| x * 2)
    .collect::<Vec<_>>();
    println!("s:{s:?}");


}

pub(crate) fn test_trait_all() {
    // test_trait_1();
    // test_trait_2();
    // test_trait_3();
    // test_trait_4();
    test_trait_5();
}