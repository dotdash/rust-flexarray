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

    fn as_slice<'r>(&'r self) -> &'r [T] {
        use std::mem::transmute;
        use std::raw::Slice;

        unsafe {
            transmute::<Slice<T>,_>(Slice {
                data: transmute(self.flex_element()),
                len: self.len(),
            })
        }
    }

    fn as_mut_slice<'r>(&'r mut self) -> &'r mut [T] {
        use std::mem::transmute;
        use std::raw::Slice;

        unsafe {
            transmute::<Slice<T>,_>(Slice {
                data: transmute(self.flex_element()),
                len: self.len(),
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

    for (i, elem) in foo.as_mut_slice().mut_iter().enumerate() {
        *elem = 2 * i;
    }

    for elem in foo.as_slice().iter() {
        println!("{}", elem);
    }
}
