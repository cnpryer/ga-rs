# ga-rs

`gars` optimizes generic data using a genetic algorithm.

## The Core Explained

The core is made up of the following logic:

- Populations
- Member
- Genes
- Fitness
- Selection
- Crossover
- Mutation

### Population

The initial population contains a vector of `members`.

```rs
struct Population {
    members: Vec<Member<T>>
}
```

### Members

A `Member` contains a unique id, genetic material, a fitness score, and, in pairs, can produce a child.

```rs
struct Member<T> {
    id: u32,
    genes: Genes<T>,
    fitness: i32
}
```

### Genes

`Genes` are composed of a vector of generic `data`.

```rs
struct Genes<T> {
    data: Vec<T>
}
```

### Fitness

`Fitness` is used to evaluate a `Member` of a `Population` during `Selection`.

```rs
fn evaluate_fitness<T>(member: &Member<T>) -> i32;
```

### Selection

Choosing a pair of parents to breed is determined by using a `Selection` method.

```rs
fn select_parents<T>(population: &Population<T>) -> [&Member<T>; 2];
```

### Crossover

`Crossover` method determines the genetic material for a new `Member` of the next generation.

```rs
fn create_child<T>(parent_a: &Member<T>, parent_b: &Member<T>) -> Member<T>;
```

### Mutation

`Mutation` refers to randomly mutated genetic material.

```rs
fn mutate_genes<T>(genes: &Genes<T>) -> Genes<T>;
```
