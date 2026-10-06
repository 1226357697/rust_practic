

fn test_iter_1() {
    let mut v = vec![1,2,3,3,4,5,6];

    for x in v.iter(){
        println!("{:?}", *x);
    }

    
    for x in v.iter_mut(){
        *x +=2;
        println!("{:?}", *x);
    }

    for x in &v{
        println!("{:?}", *x);
    }

    
    for x in & mut v{
        *x +=2;
        println!("{:?}", *x);
    }

    // 拷贝一份，不占所有权。
    for x in v.clone(){
        println!("{:?}", x);
    }

    for x in v.into_iter(){
        println!("{:?}", x);
    }

    // println!("item 1: {:?}", v.get(1)); // tigger compile an error



}

fn test_iter_2() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let evens:Vec<_> = nums
    .iter()
    .filter(|x| **x % 2 == 0)
    .collect();
    println!("evens:{:?}", evens);

    let evens:Vec<i32> = evens
    .iter()
    .map(|x| **x * 2)
    .collect();
    println!("evens:{:?}", evens);

    let evens:Vec<i32> = nums
    .iter()
    .filter(|x| **x % 2 == 0)
    .map(|x| *x * 2)
    .collect();
    println!("evens:{:?}", evens);

    if let Some(result) = evens.iter().find(|x| **x == 8) {
        println!("found 8 is {:?}", result);
    } else {
        eprintln!("not found the 8 item");
    }
    if let Some(result) = evens.iter().position(|x| *x == 8) {
        println!("found 8 index is {:?}", result);
    } else {
        eprintln!("not found the 8 item index");
    }

    println!("any 8: {:?}", evens.iter().any(|x| *x == 8));
    println!("all even : {:?}", evens.iter().all(|x| *x % 2 == 0));
    println!("count event number is {:?}", evens.iter().filter(|x| *x % 2 == 0).count());
    println!("take 2:{:?}", evens.iter().take(2).collect::<Vec<_>>());
    println!("skip 2:{:?}", evens.iter().skip(2).collect::<Vec<_>>());
    println!("min :{:?}, max :{:?} sum:{:?}", evens.iter().min(), evens.iter().max(), evens.iter().sum::<i32>());

    let result = nums
    .iter()
    .fold(100, |acc, x| acc + (x * 2) );
    println!("fold: sum(2x) is :{:?}", result );
}

fn test_iter_3() {
    let nums = vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9, 10]
    ];

    println!("nums: {:?}", nums);
    let  nums:Vec<_> = nums.iter().flatten().collect();
    println!("nums: {:?}", nums);

    let nums = vec![1, 3, 4, 6, 7];
    let result = nums
    .iter()
    .flat_map(|v|{
        (1..=*v)
    })
    .collect::<Vec<_>>();
    println!("result:{:?}, sum:{:?}", result, result.iter().sum::<i32>());

    let opt_nums = vec![
        Some(10),
        None,
        Some(20)
    ];
    println!("opt_nums :{:?}", opt_nums.iter().flatten().collect::<Vec<_>>());

    let strings = vec!["10", "abc", "20", "xyz", "30"];
    let nums = strings
    .iter()
    .filter_map(|v| v.parse::<i32>().ok())
    .collect::<Vec<_>>();
    println!("nums:{:?}", nums);


    let nums = vec![1,2,3,4,5];
    let result = nums
    .iter()
    .scan(100, |s, v| {
        *s += v * 2;
        Some(*s + 1)
    })
    .collect::<Vec<_>>();
    println!("the result of sacn is {:?}", result);

    let a  = (1..20).collect::<Vec<_>>();
    let b  = (101..120).collect::<Vec<_>>();
    let c = a.iter().zip(b.clone()).collect::<Vec<_>>();
    println!("c is {:?}", c);

    let d = a.iter().chain(b.iter()).collect::<Vec<_>>();
    println!("chain: {:?}", d);


}

pub(crate) fn test_iter_all() {
    // test_iter_1();
    // test_iter_2();
    test_iter_3();
}