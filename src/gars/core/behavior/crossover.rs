use crate::core::environment::member::{Genes, Member};

#[allow(unused_variables)]
pub fn create_child<T>(parent_a: &Member<T>, parent_b: &Member<T>) -> Member<T> {
    let child = Member::new(0, Genes::new(&Vec::new()), 0);

    child
}
