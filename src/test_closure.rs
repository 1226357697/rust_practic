
// closure
// → 捕获 ownership

// Fn/FnMut/FnOnce
// → trait

// map/filter
// → 接收 FnMut

// thread::spawn
// → move closure

// Box<dyn Fn()>
// → trait object + smart pointer

fn test_closure_1() {
    {
        let s = String::from("hello");
        let f = move || {
            println!("{s}");
        };

        f();
        f();
        f();
        // has been moved into the closure
        //println!("{s}");
    }
    

    {
        let s = String::from("hello");

        let f = move || {
            drop(s);
        };

        f();
        // f(); // has beed consume
    }


    fn run<F>(f:F)
    where 
    F:Fn() 
    {
        f();
        f();
        f();
        f();
    }

    fn run_mut<F>(mut f: F)
    where 
    F:FnMut()
    {
        f();
        f();
        f();
        f();
    }

    let mut s = String::from("hello");
    {
        let f = ||println!("s:{s}");
        run(f);
        run(f);
        run(f);
        run(f);
        run(f);
    }


    let mut f_m = ||{
        s.push('a');
        println!("s:{s}");
    };

    run_mut(&mut f_m);
    run_mut(&mut f_m);
    run_mut(&mut f_m);
    run_mut(&mut f_m);
    run_mut(&mut f_m);
    run_mut(&mut f_m);

    fn run_once(f: impl FnOnce()){
        f();
    }
    let f_once = ||{
        s.push('a');
        println!("s:{s}");
        drop(s);
    };
    run_once(f_once);
    // run_once(f_once); // error

    struct Callback {
        cb: Box<dyn Fn()>,
    }    

    let c = Callback {
        cb: Box::new(|| {
            println!("hello");
        }),
    };    

    (c.cb)();

}

pub(crate) fn test_closure_all() {
    test_closure_1();


}