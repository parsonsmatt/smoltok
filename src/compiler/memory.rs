use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Object {
    pub class: Box<Class>,
    pub fields: Vec<Pointer>
}

/// A newtype wrapper around `usize`, used for accessing values in the `Heap`.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Pointer(usize);

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
/// A `Class` is a newtype wrapper around an Object. All classes are objects,
/// after all -- but sometimes it helps to distinguish them.
pub struct Class { obj: Object }

#[derive(Debug, PartialEq, Clone)]
/// The `Heap` record is the heap of objects for the Smoltok runtime. It
/// contains some methods that operation
pub struct Heap {
    pub objects: HashMap<Pointer, Object>,
    pub curr_idx: Pointer,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            objects: HashMap::new(),
            curr_idx: Pointer(0)
        }
    }

    fn access(&self, ptr: &Pointer) -> Option<&Object> {
        self.objects.get(ptr)
    }

    // object pointer access

    pub fn fetch_pointer(&self, field_idx: usize, of_object: Object)
        -> Option<&Object> {
        let ptr = &(of_object.fields[field_idx]);
        self.access(ptr)
    }

//     pub fn store_pointer(mut self, ix: usize, object: Object, value: Object) {
//         let ptr = object.fields[ix];
//
//
//     }
}
