pub struct Genes<T> {
    data: Vec<T>,
}

impl<T> Genes<T> {
    pub fn new(data: Vec<T>) -> Genes<T> {
        Genes { data }
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
}

pub struct Member<T> {
    id: u32,
    genes: Genes<T>,
    fitness: i32,
}

impl<T> Member<T> {
    pub fn new(id: u32, genes: Genes<T>, fitness: i32) -> Member<T> {
        Member { id, genes, fitness }
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_genes(&self) -> &Genes<T> {
        &self.genes
    }

    pub fn get_fitness(&self) -> &i32 {
        &self.fitness
    }
}
