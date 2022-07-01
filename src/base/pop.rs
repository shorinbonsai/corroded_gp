use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::base::data::Data;
use crate::base::individual::Individual;

#[derive(Debug)]
pub struct Population {
    pop: Vec<Individual>,
}

impl Population {
    ///Size of population
    pub fn size(&self) -> usize {
        self.pop.len()
    }

    pub fn get_pop(&self) -> Vec<Individual> {
        self.pop.clone()
    }

    ///Init empty population
    pub fn new() -> Population {
        Population { pop: vec![] }
    }

    ///Checks if population is empty and returns true if vector is empty
    pub fn is_empty(&self) -> bool {
        self.pop.len() == 0
    }

    ///Moves a singular Individual to pop
    pub fn insert(&mut self, indiv: Individual) {
        self.pop.push(indiv);
    }

    ///Return mutable reference to population
    pub fn mut_pop(&mut self) -> &mut Vec<Individual> {
        &mut self.pop
    }

    ///Moves multiple Individuals to population in one go
    pub fn insert_multiple(&mut self, indivs: Vec<Individual>) {
        self.pop.extend(indivs)
    }

    ///Population sort by fitness
    pub fn fit_sort(&mut self) {
        //this needs Rust nightly compiler build to work so am using stopgap method for now
        // self.pop
        //     .sort_by(|a, b| a.train_fit().unwrap().total_cmp(&b.train_fit().unwrap()));
        self.pop.sort_by_key(|ind| {
            (ind.train_fit().expect("no training error computed") * 1000000.0) as u64
        });
    }

    ///To return best individual
    pub fn first_getter(&self) -> &Individual {
        &self.pop[0]
    }

    ///Return the best "numb" of values from population as sorted by fitness
    pub fn get_best(&self, numb: usize) -> Vec<Individual> {
        self.pop.iter().take(numb).map(|x| x.clone()).collect()
    }

    ///Init population using ramped half and half strategy
    pub fn ramped(psize: usize, max_depth: usize, datas: &Data) -> Population {
        let mut pop = Population::new();
        let ind_layer: f32 = (psize / max_depth) as f32;
        let ind_remain: f32 = (psize % max_depth) as f32;
        let mut numb_pop_full = (ind_layer / 2.0).ceil() as i32;
        let mut numb_pop_grow = (ind_layer / 2.0).floor() as i32;

        for i in 1..=max_depth {
            if i == max_depth {
                numb_pop_grow = (ind_layer + ind_remain / 2.0).floor() as i32;
                numb_pop_full = (ind_layer + ind_remain / 2.0).ceil() as i32;
            }
            for _ in 0..numb_pop_full {
                let mut x = Individual::full(i, datas);
                x.outputs_calculate(datas);
                x.eval_fitness(datas);
                x.depth_calc();
                pop.pop.push(x);
            }
            for _ in 0..numb_pop_grow {
                let mut x = Individual::grow(i, datas);
                x.outputs_calculate(datas);
                x.eval_fitness(datas);
                x.depth_calc();
                pop.pop.push(x);
            }
        }
        pop
    }

    ///Tournament Selection
    /// Args:
    ///  size: tournament size
    pub fn tournament(&self, size: usize) -> Individual {
        let mut rng = thread_rng();
        let mut tourn: Vec<Individual> = vec![];
        while tourn.len() < size {
            let indiv = self.pop.choose(&mut rng).expect("empty");
            tourn.push(indiv.clone());
        }
        //This needs Rust nightly so stopgap method used for now
        // tourn.sort_by(|a, b| a.train_fit().unwrap().total_cmp(&b.train_fit().unwrap()));
        tourn.sort_by_key(|ind| {
            (ind.train_fit().expect("no training error computed") * 1000000.0) as u64
        });
        let child: Individual;
        for i in &tourn {
            if !i.train_fit().unwrap().is_nan() {
                child = i.clone();
                return child;
            }
        }
        // let child = tourn[0].clone();
        child = tourn[0].clone();
        child
    }

    // pub fn tournament(&self, size: usize) -> &Individual {}
}
