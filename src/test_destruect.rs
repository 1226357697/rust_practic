use std::f32::consts::EULER_GAMMA;



fn test_destruect_1(){
    let p = (10 ,20);
    let (x, y) = p;

    println!("{x}, {y}");

    fn op1() ->(i32, i32){
        (100 ,200)
    }
    
    let (x, y) = op1();
    
    println!("{x}, {y}");

    let value = (1, (2, 3));
    let (_, (a,b)) = value;
    println!("{a}, {b}");

    let point = (0, 10);

    match point {
        (0, y) => println!("on Y axis: {y}"),
        (x, 0) => println!("on X axis: {x}"),
        (x, y) => println!("point: {x}, {y}"),
    }
}


fn test_destruect_2(){
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32
    }

    let p = Point {
        x:10,
        y:20
    };
    println!("{p:?}");

    let Point { x, y:py } = p;
    println!("{x}, {py}");


    #[derive(Debug)]
    struct Point3D {
        x: i32,
        y: i32,
        z:i32
    }

    let Point3D{
        y,
        z:pz,
        ..
    } = Point3D{
        x:1,
        y:2,
        z:3
    };
    
    println!("{y}, {pz}");

    fn print_point(Point { x, y }: Point) {
        
        println!("print_point: {x}, {y}");
    }
    print_point(p);
}

fn test_destruect_3(){
    enum  Message {
        Quit,
        Move(i32, i32),
        Write(String),
        Register {
            name:String,
            age:i32
        }
    }

    let msg = Message::Move(10, 20);
    let mut msg = Message::Register{
        name:"123".to_owned(),
        age:2
    };
    match &mut msg {
        Message::Quit=>{
            println!("quit");
        }
        Message::Move(x, y) => {
            println!("Move x:{x} y:{y}");
        }
        Message::Write(s) =>{
            println!("msg: {s}");
        }
        Message::Register { name:n, age }=>{
            
            println!("register: name:{n} age:{age}");
        }
    }

    if let Message::Register { name, age} = &msg {

            println!("register: name:{name} age:{age}");
    }

    struct Function {
        name:String,
        range:(u64, u64)
    }

    let Function {
        name,
        range:(start, end)
    } =Function { name: "()".to_owned(), range: (1,2) };
    
    println!("name:{name} range:{start} - {end}");

    #[derive(Debug)]
    enum Event {
        Function{
            name:String,
            range:(u64, u64)
        },
        Error(String),
    }

    let event = Event::Function { 
        name:String::from("value"),
        range:(1,32)
    };
    match event {
        Event::Error(s) =>eprintln!("event error :{s}"),
        Event::Function { name:n, range:(s,e) } =>{
            println!("name:{n} range:{s} - {e}")
        }
    }
    let event = Event::Function { 
        name:String::from("value"),
        range:(1,32)
    };
    
    match event {
        ref e  @ Event::Function { ref name, .. } if name =="value"=>{
            println!("e:{:?}", e);
        }
        _=>{}
    }

    let x = 5;
    match x {
        n @ 1..=10 =>{
            println!("small number: {n}");
        }
        _=>{}
    }
    

}

pub(crate) fn test_destruect_all() {
    // test_destruect_1();
    // test_destruect_2();
    test_destruect_3();
}