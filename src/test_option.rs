

fn option_1() {
    let x: Option<i32> = Some(10);
    let y: Option<i32> = None;

    println!("x: {x:?}, y :{y:?}");
    println!("x: {:?}, y {:?}", x.is_some(), y.is_none());
    println!("x: {:?}, y {:?}", x.is_none(), y.is_none());
    println!("x: {:?}", x.unwrap());

    println!("x: {:?}", x.is_some_and(|x| x>100));
    println!("y: {:?}", y.is_none_or(|x| x>10));

    let x: Option<i32> = None;
    println!("x: {:?}", x.unwrap_or(100));
    println!("x: {:?}", x.unwrap_or_default());
    println!("x: {:?}", x.unwrap_or_else(|| 999));


    let x: Option<i32> = None;
    println!("x: {:?}", x.expect("the x value is None")); // crash here
    println!("x: {:?}", x.unwrap()); // crash here

}


fn option_2() {
    let x = Some(10).map(|v| v*2);
    let none = None::<i32>.map(|v| v*2);
    println!("x: {x:?} None: {none:?}");

    let id: Option<i32> = Some(10).and_then(|v| Some(v*2)).and_then(|v| Some(100*v));
    let age: Option<i32> = None::<i32>.and_then(|v| Some(v*2)).and_then(|v| Some(100*v));
    println!("id: {id:?} age: {age:?}");

    let a = None::<i32>;
    let b = None::<i32>; 
    let c =Some(10) .and_then(|v| Some(v*23));
    println!("value: {:?}", a.or(b).or_else(|| None).or(c));

    let x  = Some(10);
    let mut y = x.filter(|x| *x >5);
    let z = x.filter(|x| *x >10);
    println!("x: {x:?}, y:{y:?}, z:{z:?}");

    let z = y.take();
    println!("x: {x:?}, y:{y:?}, z:{z:?}");

    
}


pub(crate) fn test_option_all() {
    // option_1();
    option_2();
}