use gars::core::environment::member::{Genes, Member};
use gars::core::environment::population::Population;

const TEST_DATA_SIZE: usize = 100;
const TEST_DATA: [i32; TEST_DATA_SIZE] = [0; TEST_DATA_SIZE];

#[test]
fn test_get_version() {
    let version = gars::get_version();

    assert!(version != "unknown");
}

#[test]
fn test_genes() {
    let data = TEST_DATA.to_vec();
    let genes = Genes::new(data);

    assert_eq!(&TEST_DATA.to_vec(), genes.get_data());
}

#[test]
fn test_member() {
    let data = TEST_DATA.to_vec();
    let member = Member::new(0, Genes::new(data), 0);

    assert_eq!(&TEST_DATA.to_vec(), member.get_genes().get_data());
}

#[test]
fn test_population() {
    let data = TEST_DATA.to_vec();
    let n_members = 100;
    let mut members = Vec::new();
    for _i in 0..n_members {
        members.push(Member::new(0, Genes::new(data.clone()), 0))
    }
    let population = Population::new(members);
    let pop_members = population.get_members();

    assert_eq!(pop_members.len(), n_members);
}
