use std::ops::Deref;



fn test_string_1() {
    let s :String = "  123123你好  ".to_string();
    let sref = s.as_str();
    let sref:&str=  &s;
    let sref:&String=  &s;
    let sref=  s.deref();
    
    println!("is empty :{:?}", s.is_empty());
    println!("size :{:?}", s.len());
    println!("count :{:?}", s.chars().count());
    println!("contains 你好 :{:?}", s.contains("你好"));
    println!("contains 'r' :{:?}", s.contains('2'));
    println!("startwith '123' :{:?}", s.starts_with("123"));
    println!("find '你' :{:?}", s.find("你"));
    println!("replace' :{:?}", s.replace("你", "哈哈"));
    println!("trim' :{:?}", s.trim());

    let s = "a,b,c";
    println!("p:{:?}",  s.split(",").collect::<Vec<_>>());
    println!("p:{:?}",  s.split_once(","));

    let s = "hello   rust\tworld";
    let parts: Vec<_> = s.split_whitespace().collect();
    println!("parts: {:?}", parts);
    println!("to_uppercase: {:?}", s.to_uppercase());
    println!("to_lowercase: {:?}", s.to_lowercase());

    let mut s = s.to_string();
    s.push('!');
    s.push_str(".......");
    println!("s: {:?}", s);

    let s = "你好😂！！..";
    for c in s.chars() {
        println!("{c}");
    }
    println!("at 3:{:?}", s.to_string().chars().nth(2));
    println!("bytes: {:?}", s.bytes().collect::<Vec<_>>());
    

    let s = "hello";
    let sub = &s[1..3];
    println!("sub: {:?}", sub);
    
    let sub = s.get(1..3);
    println!("sub: {:?}", sub);

    let s = "prefix_hello_stuffix";
    let result = s.strip_prefix("prefix_");
    println!("result: {:?}", result);
        let result = s.strip_suffix("_stuffix");
    println!("result: {:?}", result);

    let s = "a b c";
    let result = s
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>();
    println!("result :{result}");

    let s = "abc123xyz456";

    let digits: String = s
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    
    println!("result :{digits}");

    let digits: String = s
        .chars()
        .filter_map(|c| c.is_ascii_digit().then_some( char::from_u32(c as u32 + 1)))
        .flatten()
        .collect();
    
    println!("result :{digits}");    
}


fn test_string_2() {
    let a = String::from("hello");
    let b = " rust";

    let c = a + b;
    println!("c: {:?}", c);

    let name = "test";
    let addr = 0x1000;

    let s = format!("function {name} at {addr:#x}");
    println!("s: {:?}", s);

    let parts = ["a", "b", "c"];
    println!("join: {:?}", parts.join(","));

    if let Ok(n) = "123".parse::<i32>() {
        println!("{n}");
    }
    let addr = u64::from_str_radix("1000", 16);
    
    println!("{:?}", addr);

    let s = "0x401000";

    let addr = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .and_then(|v| u64::from_str_radix(v, 16).ok());   

    println!("{:?}", addr);

    let s = "123\n12412\r\n1514142\n";
    let result = s.lines().collect::<Vec<_>>();
    println!("{:?}", result);    
    
}

pub(crate) fn test_string_all() {
    // test_string_1();
    test_string_2();
}