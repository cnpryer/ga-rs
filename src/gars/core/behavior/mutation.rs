use crate::core::environment::member::Genes;

const DEFAULT_MUTATION_RATE: f32 = 0.5;

pub struct MutationConfig {
    rate: f32,
}

impl MutationConfig {
    pub fn new(rate: f32) -> MutationConfig {
        MutationConfig { rate }
    }

    pub fn get_rate(&self) -> &f32 {
        &self.rate
    }
}

impl Default for MutationConfig {
    fn default() -> MutationConfig {
        MutationConfig {
            rate: DEFAULT_MUTATION_RATE,
        }
    }
}

pub fn mutate_genes<T>(genes: &Genes<T>, config: Option<MutationConfig>) -> Genes<T> {
    unimplemented!()
}
