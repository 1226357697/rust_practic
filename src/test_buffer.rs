use std::{mem::MaybeUninit, ptr::NonNull, vec};
use std::mem::offset_of;



fn test_buffer_1() {

    let data = vec![1, 2, 3, 4];
    let slice:&[u8] = &data;
    println!("slice:{slice:?}");

    let data = [1u8, 2, 3, 4];
    let slice :&[u8] = &data;
    println!("slice:{slice:?}");
    let slice :&[u8] = &data[..];
    println!("slice:{slice:?}");

    let data = vec![1u8, 2,3 ];
    let slice = data.as_slice();
    println!("slice:{slice:?}");

    let slice:&[u8] = &data;
    println!("slice:{slice:?}");    

    let slice = &data[..];
    println!("slice:{slice:?}");    

    let mut data = vec![1, 2, 3];
    let slice:&mut [u8] = &mut data;
    slice[0] = 100;
    println!("data: {data:?}");

    let a = &[1, 2, 3];
    let b = &[1, 2, 3];
    println!("a == b is {:?} data == a is {:?}", a==b, a == data.as_slice());


}


fn test_buffer_2() {
    let mut buf = Vec::with_capacity(0x100);
    
    buf.extend_from_slice(b"MZ");

    let value = 0x20u32;
    buf.extend_from_slice(&value.to_le_bytes());

    let data:[u8;4] = [1,2,3,4];
    buf.extend(data.iter());
    println!("buf: {buf:?}, capacity:{:?}", buf.capacity());

    buf.reserve(0x1000);
    println!("buf: {buf:?}, capacity:{:?}", buf.capacity());

    // slice to array
    let bytes:[u8;4] = buf.get(2..2+4).unwrap().try_into().unwrap();
    let v = u32::from_le_bytes(bytes);
    println!("v: {v:<08x}");

    // padding
    // alignment
    // endianness
    // reper
    struct Header {
        a:u32,
        b:u16,
    }
    let mut buf = Vec::<u8>::with_capacity(0x100);
    let header = Header{
        a:10,
        b:20
    };

    //recommend
    buf.extend_from_slice(&header.a.to_le_bytes());
    buf.extend_from_slice(&header.b.to_le_bytes());
    println!("buf: {buf:?}");


    // padding
    // alignment
    // endianness    
    #[repr(C)]
    struct PackHeader {
        magic: u32,
        size: u16,
    }
    
}

fn test_buffer_3(){ 
    // &T
    // &mut T
    // *const T
    // *mut T
    // NonNull<T>
    // Box<T>
    // Rc<T>
    // Arc<T>

    let mut x = 10;
    let p:*const i32 = &x;
    println!("x ptr:{:p} p:{p:?}",&x);
    unsafe {
        println!("*p:{:?}", *p);
    }

    let p :*mut i32=&mut x;
    unsafe {
        *p = 20;
    }
    println!("x {x}");

    
    let p1 :*mut i32=&mut x;
    let p2 :*mut i32=&mut x;
    let p3 :*mut i32=&mut x;
    unsafe {
        println!("p1:{:?} p2:{:?} p3:{:?}", *p1, *p2, *p3);
    }

    let data = [10, 20, 30];
    let p  = data.as_ptr();
    unsafe {
        println!("p:{:?}, index 1 is {:?} ptr:{:?} offset:{:?}", p, *p.add(1), p.add(1), p.offset(1));
    }
    
    let slice = unsafe {
        std::slice::from_raw_parts(p, 4) //越界
    };
    println!("slice:{slice:?}");


    let p1 = slice.as_ptr();
    let len = slice.len();
    println!("p1:{p1:?}, len:{len}");


    unsafe {
        let p = data.as_ptr().cast::<u8>();
        let size =  data.len() * size_of::<i32>();
        let bytes = (0..size)
            .into_iter()
            .map(|i|*p.add(i))
            .collect::<Vec<_>>();
        println!("bytes:{bytes:?}");

        let v1 = std::ptr::read(p1);
        let v2 = std::ptr::read(p1.add(2));
        println!("v1 {v1:?} v2 {v2:?}");

        let p = p1.byte_add(2);
        // let v1 = std::ptr::read(p); // undefiend error
        let v1 = std::ptr::read_unaligned(p);
        
        println!("p:{p:?} v1 {v1:X} v2 {v2:?}");

    }

    let mut value = 10;
    let ptr = NonNull::from(&mut value);    
    println!("ptr:{ptr:?}");


    let b = Box::new([1,2,3,4]);
    let p  =b.as_ptr();
    println!("p:  {p:?}");
    let ptr = Box::into_raw(b);
    println!("ptr:{ptr:?}");

    let b = unsafe {
        Box::from_raw(ptr)
    };

    let p  =b.as_ptr();
    println!("p:  {p:?}");    


    let mut v = vec![1, 2,3];
    let b:Box<[u8]> = v.into_boxed_slice();
    println!("b:{b:?}");
    // println!("v:{v:?}"); // has been moved
    v = b.into_vec();
    // println!("b:{b:?}"); // has been moved
    println!("v:{v:?}");

    
    let mut  data = Vec::new();
    data.extend_from_slice(b"MZ");
    data.extend_from_slice(b"\x90\x90\x90");
    data.extend_from_slice(b"END");
    if data.starts_with(b"MZ") {
        println!("data satrt with MZ");
    }

    if data.ends_with(b"END") {
        println!("data end with MZ");
    }    
    if data.contains(&0x90) {
        println!("data contain 0x90");
    }

    if let Some(pos) = data.iter().position(|b|*b == 0x90){
        println!("data  0x90 at {pos:0x}");
    }

    let positions:Vec<_> = data
    .iter()
    .enumerate()
    .filter_map(|(i, &v)|(v == 0x90).then_some(i))
    .collect();
    println!("positions:{positions:?}");

    let pattern = b"MZ";
    let pos = data
    .windows(pattern.len())
    .position(|w| w == pattern);
    println!("pos:{pos:?}");

    let data = [1,2,3,4,5];
    for w in data.windows( 2) {
        println!("w:{w:?}");
    }

    for block in data.chunks(2) {
        println!("block:{block:?}");
    }

    let mut chunks = data.chunks_exact(2); 
    for block in &mut chunks{
        println!("block: {block:?}");
    }
    let remain = chunks.remainder();
    println!("remain: {remain:?}");
    

    let mut data = [1, 2, 3, 4, 5];

    data.copy_within(1..3, 0);
    println!("data:{data:?}");

    let data1 = [1,2,3,4];
    let mut data2:[i32;4] = [0;4];
    unsafe {
        let p1 = data1.as_ptr();
        let p2 = data2.as_mut_ptr();
        std::ptr::copy_nonoverlapping(p1, p2, data1.len());
    }
    println!("data2:{data2:?}");

    #[derive(Debug)]
    struct Header {
        magic:u32,
        size:u16,
    }

    let mut value = MaybeUninit::<Header>::uninit();
    let ptr = value.as_mut_ptr();
    unsafe {println!("ptr:{ptr:?} *ptr:{:?}", *ptr);}
    unsafe {
        ptr.write(Header { magic: 10, size: 20 });    
    }
    let value = unsafe {
        value.assume_init()
    };

    println!("value:{value:?}");
    println!("Header size is {:?}  alignment:{:?}", std::mem::size_of::<Header>(),  std::mem::align_of::<Header>());

    #[repr(C, packed)]
    struct Header2 {
        a: u32,
        b: u16,
    }
    println!("Header2 size is {:?} alignment:{:?}", std::mem::size_of::<Header2>(), std::mem::align_of::<Header2>());
    
    println!("a offset:{:?} b offset:{:?}",offset_of!(Header2, a), offset_of!(Header2, b) );

}

fn test_buffer_4() {
    struct Reader<'a> 
    {
        data:&'a[u8],
        pos:usize
    }

    impl<'a> Reader<'a> {
        fn new(data:&'a [u8]) ->Self {
            Self { data, pos: 0 }
        }

        fn read_u8(&mut self) ->Option<u8> {
            let v = *self.data.get(self.pos)?;
            self.pos += 1;
            Some(v)
        }

        fn read_u16_le(&mut self) ->Option<u16> {
            let bytes = self.data.get(self.pos..self.pos+2)?;
            self.pos += 2;
            Some(u16::from_le_bytes(
                bytes.try_into().ok()?
            ))
        }

        fn read_u32_le(&mut self) ->Option<u32> {
            let size = std::mem::size_of::<u32>();
            let bytes = self.data.get(self.pos..self.pos+size)?;
            self.pos += size;
            Some(u32::from_le_bytes(
                bytes.try_into().ok()?
            ))
        }    


        fn read_bytes(
            &mut self,
            len: usize,
        ) -> Option<&'a [u8]> {
            let bytes = self.data.get(
                self.pos..self.pos + len
            )?;

            self.pos += len;

            Some(bytes)
        }            
    }

    struct Writer {
        data:Vec<u8>,
    }

    impl Writer {
        fn write_u32_le(&mut self, v:u32) {
            self.data.extend_from_slice(&v.to_le_bytes());
        }

        fn write_bytes(&mut self, data: &[u8]) {
            self.data.extend_from_slice(data);
        }        
    }

}

pub(super) fn test_buffer_all() {
    // test_buffer_1();
    // test_buffer_2();
    // test_buffer_3();
    test_buffer_4();
}