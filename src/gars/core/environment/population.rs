use crate::core::environment::member::Member;

pub struct Population<T> {
    members: Vec<Member<T>>,
}

impl<T> Population<T> {
    pub fn new(members: Vec<Member<T>>) -> Population<T> {
        Population { members }
    }

    pub fn get_members(&self) -> &Vec<Member<T>> {
        &self.members
    }
}
