use crate::core::environment::member::Member;
use crate::core::environment::population::Population;

pub fn select_parents<T>(population: &Population<T>) -> [&Member<T>; 2] {
    let members = population.get_members();
    let parents = [&members[0], &members[1]];

    parents
}
