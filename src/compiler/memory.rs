#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub class: Box<Class>,
    pub fields: Vec<Object>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Class { obj: Object }

#[derive(Debug, PartialEq, Clone)]
pub struct Heap {
    pub objects: Vec<Object>
}

impl Heap {
    pub fn fetch_pointer(
        field_idx: usize,
        object: &Object
    ) -> Object {
        object.fields[field_idx].clone()
    }

    pub fn store_pointer(
        field_idx: usize,
        object: &mut Object,
        val: Object
    ) {
        object.fields[field_idx] = val
    }
}
