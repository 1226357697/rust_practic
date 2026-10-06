// Either many immutable references or one mutable reference.
#![feature(cow_is_borrowed)]
use  std::borrow::Cow;
use std::{fmt::Debug, ops::AddAssign};


fn test_ownship_1(){
    let s = "Hello world".to_string();
    let s1 = s;
    // println!("s: {:?}", s); // compile error
    println!("s1: {:?}", s1); 

    let s = "Hello world".to_string();
    let s1 = s.clone();
    println!("s: {:?}", s); 
    println!("s1: {:?}", s1);     

    let mut s = "Hello world".to_string();
    let s1 = &s;
    let s2 = s1;
    println!("s: {:?}", s); 
    println!("s1: {:?}", s1);     

    let s3 =    &mut s;
    // println!("s2: {:?}", s2);        // compile error 
    println!("s3: {:?}", s3);      

    let mut s = "Hello world".to_string();
    let mut s1 = &mut s;
    // let s2 =  s1; // move 
    let s2 =  &mut *s1; //re-borrow
    // println!("s: {:?}", s);  //compile error
    // println!("s2: {:?}", s2);     
    // println!("s1: {:?}", s1);     


    // println!("s2: {:?}", s2);     
    // println!("s: {:?}", s);  //compile error
    // println!("s1: {:?}", s1);      

    println!("s2: {:?}", s2);     
    println!("s1: {:?}", s1);      
    println!("s: {:?}", s);  
       

}

fn test_ownship_2() {
    fn modify(x:&mut i32) {
        *x += 1
    }

    let mut x = 3i32;
    let r = &mut x;

    // println!("x:{:?}", x); //There is already a variable.

    // Implicit reborrowing
    modify(r);
    modify(r);
    modify(r);

    // Implicit reborrowing
    fn modify_2<T:AddAssign + From<i32>>(x:&mut T) {
        *x += T::from(1)
    }

    let mut x = 5;
    modify_2(r);
    modify_2(r);
    modify_2(r);
    println!("x:{x}");



    fn modify_3<T:Debug>(x: T) {
        println!("x:{:?}",x);
    }

    let mut x = 5;
    let r = &mut x;
    modify_3(&r);
    modify_3(&r);
    modify_3(&r);
    println!("x:{x}");   


    fn modify_4<T:Debug+From<i32>>(x:&mut T) {
        *x = T::from(3);
        println!("x:{:?}",x);
    }

    let mut x = 5;
    let r = &mut x;
    modify_4(r);
    modify_4(r);
    modify_4(r);

    modify_4(&mut *r);
    modify_4(&mut *r);
    modify_4(&mut *r);

    println!("x:{x}");         

    fn foo(x:&mut i32) ->&mut i32 {
        *x +=2;
        x
    }

    let mut x = 4;
    let r = &mut x;
    foo(&mut *r);
    foo(&mut *r);
    foo(&mut *r);

    println!("x: {:?}", x);


    let mut x = 4;
    let r = &mut x;
    let r1 = foo(&mut *r);
    // An error occurred while borrowing.
    // foo(&mut *r);
    // foo(&mut *r);
    println!("r1: {:?}", r1);

    println!("x: {:?}", x);    
}

fn test_ownship_3(){

    #[derive(Debug)]
    struct Profile<'a> {
        file:&'a String,
        time: String
    }

    let f1 = "1.txt".to_string();
    let p1 = Profile{
        file: &f1,
        time: "1:32".to_string()
    };

    println!("p1: {:?}", p1);

    let r1 = p1.file;
    println!("r1: {:?}", r1);

    
    let t = p1.time;
    println!("t: {:?}", t);


    println!("p1.file: {:?}", p1.file);
    // println!("p1.time: {:?}", p1.time); // has been moved
    // println!("p1: {:?}", p1); // has been moved

    // drop(f1); // cannot move out of `f1` because it is borrowed
    println!("p1.file: {:?}", p1.file);

}


fn test_ownship_4() {

    fn make_normalize_string(s: &str)->Cow<'_, str> {
        if s.contains('\\') {
            Cow::Owned(s.replace('\\', "/"))
        } else {
            Cow::Borrowed(s)
        }
    }
    let path = make_normalize_string("C:\\1.txt");
    println!("path: {:?} ", path );
    match &path {
        Cow::Borrowed(r)=> println!("borrowed {:?} ", r),
        Cow::Owned(r)=> println!("Owned {:?}", r),
    }

    let path = make_normalize_string("/home/yan/1.txt");
    println!("path: {:?}", path);    
        match &path {
        Cow::Borrowed(r)=> println!("borrowed {:?} ", r),
        Cow::Owned(r)=> println!("Owned {:?}", r),
    }

}

pub(crate) fn test_ownship_all() {
    // test_ownship_1();
    test_ownship_2();
    // test_ownship_3();
    // test_ownship_4();
}