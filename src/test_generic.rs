use std::fmt::Debug;
use std::marker::PhantomData;

fn test_generic_1(){
    fn identity<T>(v:T)->T{
        v
    }

    let a = identity(10);
    let b = identity("hello");
    let c = identity(String::from("rust"));

    // identity<i32>
    // identity<&str>
    // identity<String>

    fn pair<T, U>(a: T, b: U) -> (T, U) {
        (a, b)
    }
    let p = pair(10, "hello");
    println!("p:{p:?}");


    struct  Point<T>{
        x:T,
        y:T,
    }
    let p1 = Point {
        x: 1,
        y: 2,
    };

    let p2 = Point {
        x: 1.0,
        y: 2.0,
    };

    // Point<i32>
    // Point<f64>

    struct Pair<T, U> {
        first: T,
        second: U,
    }    

    let p = Pair {
        first: 0x401000u64,
        second: "main".to_string(),
    };    
    // Pair<u64, String>

    struct  Wrapper<T> {
        v:T,
    }
    impl<T> Wrapper<T> {
        fn new(v:T)->Self {
            Self { v }
        }

        fn get(&self) ->&T {
            &self.v
        }
    }

    impl<T:Debug> Wrapper<T> {
        fn print(&self) {
            println!("{:?}", self.v);
        }
    }

    Wrapper::new(4).print();

    struct Cache<K,V>
    where 
    K:Eq + std::hash::Hash {
        k:K,
        v:V
    }

    struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        fn new(value: T) -> Self {
            Self { value }
        }
    }

    impl<T: Clone> Container<T> {
        fn cloned_value(&self) -> T {
            self.value.clone()
        }
    }    

    struct File;
    struct Process;

    struct Handle<T> {
        id: u64,
        _marker: PhantomData<T>,
    }

    let file: Handle<File> = Handle {
        id: 1,
        _marker: PhantomData,
    };

    let process: Handle<Process> = Handle {
        id: 1,
        _marker: PhantomData,
    };    

}

fn test_generic_2() {
    struct VirtualAddress(u64);
    struct RelativeVirtualAddress(u32);
    struct FileOffset(u64);
    struct ImageBase(u64);

    fn rva_to_va(
        base: ImageBase,
        rva: RelativeVirtualAddress,
    ) -> VirtualAddress {
        VirtualAddress(base.0 + rva.0 as u64)
    }    

   

}

fn test_generic_3() {
    struct  Va;
    struct  Rva;
    struct  FileOffset;

    #[derive(Debug, Clone, Copy)]
    struct Address<K> {
        v:u64,
        _kind:PhantomData<K>
    }

    type VirtualAddress = Address<Va>;
    type RelativeAddress = Address<Rva>;
    type Offset = Address<FileOffset>;   


    struct Uninitalized;
    struct Initialized;

    struct Eninge<State> {
        _state:PhantomData<State>,
    }

    impl Eninge<Uninitalized> {
        fn init(self) ->Eninge<Initialized>{
            Eninge{
                _state:PhantomData
            }
        }
    }

    impl Eninge<Initialized> {
        fn run(self) {
            println!("Eninge run...");
        }
    }    
    let e = Eninge::<Uninitalized>{_state : PhantomData}.init();
    e.run();

}

pub(crate) fn test_generic_all(){
    // test_generic_1();
    test_generic_3();
}