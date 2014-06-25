extern crate core;

struct FlexArrayField<T>([T, ..0]);

trait FlexArray<T> {
    fn len(&self) -> uint;
    fn flex_element<'r>(&'r self) -> &'r FlexArrayField<T>;

    fn allocate(base: Self) -> Box<Self> {
        use std::mem::{min_align_of, size_of, transmute};
        use std::rt::heap;

        unsafe {
            let size = size_of::<Self>() + base.len() * size_of::<T>();
            let mem = heap::allocate(size, min_align_of::<Self>());
            let mut result: Box<Self> = transmute(mem);
            *result = base;
            result
        }
    }

    fn as_slice<'r, T: FlexArray<U>, U>(&'r self) -> &'r [U] {
        use std::mem::transmute;
        use std::raw::Slice;

        unsafe {
            transmute::<Slice<U>, &'r [U]>(Slice {
                data: transmute(self.flex_element()),
                len: self.len() as uint,
            })
        }
    }

    fn as_mut_slice<'r, T: FlexArray<U>, U>(&'r mut self) -> &'r mut [U] {
        use std::mem::transmute;
        use std::raw::Slice;

        unsafe {
            transmute::<Slice<U>, &'r mut [U]>(Slice {
                data: transmute(self.flex_element()),
                len: self.len() as uint,
            })
        }
    }
}

#[repr(C)]
struct Foo {
    some_param: u8,
    length: u8,
    info: FlexArrayField<uint>,
}

impl FlexArray<uint> for Foo {
    fn len(&self) -> uint {
        self.length as uint
    }

    fn flex_element<'r>(&'r self) -> &'r FlexArrayField<uint> {
        &self.info
    }
}

fn main() {
    let mut foo: Box<Foo> = FlexArray::allocate(Foo {
        some_param: 8,
        length: 8,
        info: FlexArrayField([])
    });

    for (i, elem) in foo.as_mut_slice::<Foo,_>().mut_iter().enumerate() {
        *elem = 2 * i;
    }

    for elem in foo.as_slice::<Foo,_>().iter() {
        println!("{}", elem);
    }
}
