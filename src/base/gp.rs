extern crate rand;
use std::fs::File;
use std::io::prelude::*;

use rand::{thread_rng, Rng};

use crate::base::data::Data;
use crate::base::individual::genetics;
use crate::base::individual::Individual;
use crate::base::individual::Node;
use crate::base::pop::Population;

///Gp struct containing the algorithm state
pub struct Gp {
    dataset: Data,
    cross_chance: f32,
    mut_chance: f32,
    population: Population,
    population_size: usize,
    tournament_size: usize,
}

impl Gp {
    pub fn new(datas: Data) -> Gp {
        Gp {
            dataset: datas,
            cross_chance: 0.8,
            mut_chance: 0.2,
            population: Population::new(),
            population_size: 200,
            tournament_size: 3,
        }
    }

    ///Set the population size
    pub fn set_pop(mut self, pop_size: usize) -> Gp {
        self.population_size = pop_size;
        self
    }

    /// Initialization of population with ramped half and half generation with initial max depth of 5
    pub fn init_pop(&mut self) {
        self.population = Population::ramped(self.population_size, 5, &self.dataset);
    }

    pub fn set_tourn_size(mut self, tourn: usize) -> Gp {
        self.tournament_size = tourn;
        self
    }

    pub fn set_mut_chance(mut self, mut_chance: f32) -> Gp {
        self.mut_chance = mut_chance;
        self
    }

    pub fn set_cross_chance(mut self, cross_chance: f32) -> Gp {
        self.cross_chance = cross_chance;
        self
    }

    ///Method to print the current state of the fitnesses for an individual
    fn print_state(&self, file: &mut File) {
        let ind = self.population.first_getter();
        let tmp_pop = self.population.get_pop();
        let mut fitnesses = vec![];
        for i in tmp_pop {
            let fit = (i.train_fit().unwrap());
            if !fit.is_nan() && !fit.is_infinite() {
                fitnesses.push(fit);
            }
        }
        let sum: f32 = Iterator::sum(fitnesses.iter());
        let mean = sum / fitnesses.len() as f32;
        let line = format!("----------------------");
        println!("{}", line);
        writeln!(file, "{}", line).expect("write failed");
        let line = format!("train:\t{:?}", ind.train_fit().unwrap());
        println!("{}", line);
        writeln!(file, "{}", line).expect("write failed");
        let line = format!("training mean:\t{:?}", mean);
        println!("{}", line);
        writeln!(file, "{}", line).expect("write failed");
        let line = format!("test:\t{:?}", ind.test_fit().unwrap());
        println!("{}", line);
        writeln!(file, "{}", line).expect("write failed");
        let line = format!("size:\t{:?}", ind.size());
        println!("{}", line);
        writeln!(file, "{}", line).expect("write failed");
        let line = format!("depth:\t{:?}\n\n", ind.depth());
        println!("{}", line);
        writeln!(file, "{}", line).expect("write failed");
    }

    ///Core evolution loop
    pub fn evolve(&mut self, numb_gens: usize) {
        // I/O
        let mut file = File::create("results.txt").expect("create failed");

        let mut rng = thread_rng();
        if self.population.size() == 0 {
            self.init_pop();
        }

        for x in 0..numb_gens {
            let mut children = Population::new();
            println!("Generation: {}", x + 1);
            while children.size() < self.population.size() {
                let child: Individual;
                let mut parent1: Individual = Individual::new();
                while parent1.train_fit().is_none() || parent1.train_fit().unwrap().is_nan() {
                    parent1 = self.population.tournament(self.tournament_size);
                }
                let chance: f32 = rng.gen();
                if chance < self.cross_chance {
                    let mut parent2: Individual = Individual::new();
                    while parent2.train_fit().is_none() || parent2.train_fit().unwrap().is_nan() {
                        parent2 = self.population.tournament(self.tournament_size);
                    }
                    let parent2 = self.population.tournament(self.tournament_size);
                    child = genetics::subtree_cross(&parent1, &parent2, &self.dataset);
                } else {
                    child = genetics::single_point_mut(&parent1, &self.dataset);
                }
                children.insert(child);
            }
            self.population = children;
            self.population.fit_sort();
            self.print_state(&mut file);
            if x == numb_gens - 1 {
                let ind = self.population.first_getter().clone();
                ind.print_nodes(&mut file);
            }
        }
    }
}
