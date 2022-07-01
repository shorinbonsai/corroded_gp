use std::fs::File;
use std::io::prelude::*;

use crate::base::data::Data;
use crate::base::functions::*;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]

/// Node Enum representing a primitive of an Individual
pub enum Node {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Sine,
    Ln,
    Square,
    Input(usize),
    Constant(f32),
}

impl Node {
    /// Returns how many arguments a node takes.
    pub fn arity(&self) -> usize {
        // usize
        match *self {
            Node::Input(_) => 0,
            Node::Constant(_) => 0,
            Node::Sine | Node::Ln | Node::Square => 1,
            _ => 2,
        }
    }

    /// performs operation of node on the args.
    pub fn operation(&self, args: Vec<Vec<f32>>) -> Vec<f32> {
        if self.arity() != args.len() {
            panic!("Number of args does not match node's arity.")
        }
        match *self {
            Node::Addition => add(args[0].to_vec(), args[1].to_vec()),
            Node::Subtraction => subtract(args[0].to_vec(), args[1].to_vec()),
            Node::Sine => sine(args[0].to_vec()),
            Node::Ln => ln(args[0].to_vec()),
            Node::Square => square(args[0].to_vec()),
            Node::Multiplication => multiply(args[0].to_vec(), args[1].to_vec()),
            Node::Division => divide(args[0].to_vec(), args[1].to_vec()),
            _ => panic!("Non-functional node.  Cannot perform operation."),
        }
    }

    ///Return a f32 constant Node in range of -1.0 and 1.0
    pub fn get_ephemeral() -> Node {
        let mut rng = thread_rng();
        let mut set_const: Vec<f32> = vec![];
        set_const.push(-1.0);
        for i in 0..8 {
            let tmp = rng.gen_range(-0.9..=0.9);
            set_const.push(tmp);
        }
        set_const.push(1.0);
        let k: usize = rng.gen_range(0..set_const.len());
        Node::Constant(set_const[k])
    }

    ///Returns a random function from the set of functions defined in the Node enum.
    pub fn get_function() -> Node {
        let mut rng = thread_rng();
        let i: usize = rng.gen_range(0..6);
        match i {
            0 => Node::Addition,
            1 => Node::Subtraction,
            2 => Node::Sine,
            3 => Node::Multiplication,
            4 => Node::Division,
            5 => Node::Ln,
            6 => Node::Square,
            _ => panic!("Node function does not exist"),
        }
    }

    ///pick a random variable for input
    pub fn get_random_input(dimensions: usize) -> Node {
        let mut rng = thread_rng();
        let i: usize = rng.gen_range(0..dimensions);
        Node::Input(i)
    }
}

///An Individual is a flattened tree of nodes
/// consists of terminal nodes for constants and variables, as well as function nodes
#[derive(Debug, Clone)]
pub struct Individual {
    ///Vector consisting of the Nodes of the syntax tree
    chromosome: Vec<Node>,
    ///Training data output
    training_outputs: Option<Vec<f32>>,
    ///Test data output
    test_outputs: Option<Vec<f32>>,
    ///Fitness value (error) for training data
    train_fitness: Option<f32>,
    ///Fitness value (error) for testing data
    test_fitness: Option<f32>,
    ///Number of elements in tree
    size: Option<usize>,
    ///Tree depth
    depth: Option<usize>,
}

impl Individual {
    pub fn cloned(&self) -> Vec<Node> {
        self.chromosome.clone()
    }

    ///fitness for training data
    pub fn train_fit(&self) -> Option<f32> {
        self.train_fitness
    }

    ///fitness for testing data
    pub fn test_fit(&self) -> Option<f32> {
        self.test_fitness
    }
    pub fn size(&self) -> usize {
        if self.chromosome.len() > 0 {
            self.chromosome.len()
        } else {
            self.size.expect("Vector empty")
        }
    }
    pub fn depth(&self) -> usize {
        self.depth.expect("Not computed")
    }

    pub fn training_outputs(&self) -> Vec<f32> {
        self.training_outputs
            .clone()
            .expect("Training data outputs not calculated")
    }

    pub fn test_outputs(&self) -> Vec<f32> {
        self.test_outputs
            .clone()
            .expect("Test data outputs not calculated")
    }

    pub fn outputs(&self) -> (Vec<f32>, Vec<f32>) {
        (self.training_outputs(), self.test_outputs())
    }

    ///Method to initialize an empty Individual
    pub fn new() -> Individual {
        Individual {
            chromosome: vec![],
            training_outputs: None,
            test_outputs: None,
            train_fitness: None,
            test_fitness: None,
            size: None,
            depth: None,
        }
    }

    ///Create individual with grow method.
    ///Generate an expression where each leaf might have a different depth
    pub fn grow(depth_max: usize, data_object: &Data) -> Individual {
        let mut x: Individual = Individual::new();
        x.grow_recurse(0, depth_max, data_object.dimensions());
        x
    }

    ///Recursive method for grow method
    fn grow_recurse(&mut self, depth: usize, depth_max: usize, dimensions: usize) {
        let mut rng = thread_rng();
        if depth == depth_max {
            if rng.gen::<bool>() {
                self.chromosome.push(Node::get_ephemeral());
            } else {
                self.chromosome.push(Node::get_random_input(dimensions));
            }
        } else {
            if rng.gen::<bool>() {
                let new_nodey = Node::get_function();
                let arity = new_nodey.arity();
                self.chromosome.push(new_nodey);
                for _ in 0..arity {
                    self.grow_recurse(depth + 1, depth_max, dimensions);
                }
            } else {
                if rng.gen::<bool>() {
                    self.chromosome.push(Node::get_ephemeral());
                } else {
                    self.chromosome.push(Node::get_random_input(dimensions));
                }
            }
        }
    }

    ///Create individual with full method.
    ///Generate an expression where each leaf has the same depth
    pub fn full(depth_max: usize, data_object: &Data) -> Individual {
        let mut x: Individual = Individual::new();
        x.full_recurse(0, depth_max, data_object.dimensions());
        x
    }

    ///Recursive method for full method
    fn full_recurse(&mut self, depth: usize, depth_max: usize, dimensions: usize) {
        let mut rng = thread_rng();
        if depth == depth_max {
            if rng.gen::<bool>() {
                self.chromosome.push(Node::get_ephemeral());
            } else {
                self.chromosome.push(Node::get_random_input(dimensions));
            }
        } else {
            let new_nodey = Node::get_function();
            let arity = new_nodey.arity();
            self.chromosome.push(new_nodey);
            for _ in 0..arity {
                self.full_recurse(depth + 1, depth_max, dimensions);
            }
        }
    }

    ///Clone for node at index position
    fn get_node(&mut self, index: usize) -> Node {
        self.chromosome[index].clone()
    }

    ///Calculate tree depth
    pub fn depth_calc(&mut self) {
        let mut index = 0;
        let depth_init = 0;
        self.depth = Some(0);
        self.depth_calc_recursive(&mut index, &depth_init);
    }

    fn depth_calc_recursive(&mut self, index: &mut usize, depth: &usize) {
        for _ in 0..self.chromosome[*index].arity() {
            *index += 1;
            self.depth_calc_recursive(index, &(depth + 1));
        }
        if *depth > self.depth.unwrap() {
            self.depth = Some(*depth);
        }
    }

    ///Return a vector of results for all data
    fn get_outputs(&mut self, mut index: usize, data: &Vec<Vec<f32>>) -> Vec<f32> {
        let nodey = &self.get_node(index);
        match nodey {
            &Node::Constant(numb) => vec![numb; data[0].len()],
            &Node::Input(x) => data[x].to_vec(),
            _ => {
                let mut arguments: Vec<Vec<f32>> = vec![];
                for _ in 0..nodey.arity() {
                    index += 1;
                    arguments.push(self.get_outputs(index, data));
                }
                nodey.operation(arguments)
            }
        }
    }

    ///Calculate the outputs for the training and test data
    pub fn outputs_calculate(&mut self, datas: &Data) {
        self.training_outputs = Some(self.get_outputs(0, datas.train()));
        self.test_outputs = Some(self.get_outputs(0, datas.test()));
    }

    ///Calculate the root mean squared error for the program outputs vs the data labels
    pub fn eval_fitness(&mut self, datas: &Data) {
        self.train_fitness = Some(root_mean_squared(
            &self.training_outputs(),
            datas.train_targets(),
        ));
        self.test_fitness = Some(root_mean_squared(
            &self.test_outputs(),
            datas.test_targets(),
        ));
    }

    ///Determine number of nodes in a subtree starting at an index
    pub fn nodes_subtree(&self, start: usize) -> usize {
        match self.chromosome[start] {
            Node::Constant(_) => 1,
            Node::Input(_) => 1,
            _ => {
                let mut node_numb = 1;
                for _ in 0..self.chromosome[start].arity() {
                    node_numb += self.nodes_subtree(start + node_numb);
                }
                node_numb
            }
        }
    }

    ///Utility functions for genetic operators
    pub fn left_copy_outside(&self, node_location: usize) -> Vec<Node> {
        let mut left: Vec<Node> = vec![];
        for x in 0..node_location {
            left.push(self.chromosome[x].clone());
        }
        left
    }

    pub fn right_copy_outside(&self, node_location: usize) -> Vec<Node> {
        let mut right: Vec<Node> = vec![];
        for x in node_location..self.chromosome.len() {
            right.push(self.chromosome[x].clone());
        }
        right
    }

    pub fn subtree_copy(&self, from: usize, subtree: usize) -> Vec<Node> {
        let mut copy: Vec<Node> = vec![];
        let to = from + subtree;
        for x in from..to {
            copy.push(self.chromosome[x].clone());
        }
        copy
    }

    pub fn insert(&mut self, vec: Vec<Node>) {
        for x in vec {
            self.chromosome.push(x);
        }
    }

    pub fn print_nodes(&self, file: &mut File) {
        let mut first: String = match self.chromosome[0] {
            Node::Addition => "+".to_string(),
            Node::Subtraction => "-".to_string(),
            Node::Sine => "sin(".to_string(),
            Node::Multiplication => "*".to_string(),
            Node::Division => "/".to_string(),
            Node::Ln => "ln(".to_string(),
            Node::Square => "square(".to_string(),
            Node::Constant(x) => format!("({})", x),
            Node::Input(j) => format!("x{}", j),
            _ => "".to_string(),
        };
        first.push_str("(");
        // println!("{}", first);

        for i in 1..self.chromosome.len() {
            let nodey: String = match self.chromosome[i] {
                Node::Addition => "+".to_string(),
                Node::Subtraction => "-".to_string(),
                Node::Sine => "sin(".to_string(),
                Node::Multiplication => "*".to_string(),
                Node::Division => "/".to_string(),
                Node::Ln => "ln(".to_string(),
                Node::Square => "square(".to_string(),
                Node::Constant(x) => format!("({})", x),
                Node::Input(j) => format!("x{}", j),
                _ => "".to_string(),
            };
            if self.chromosome[i].arity() == 2 {
                let tmp: String = format!("{}(", nodey);
                first.push_str(&tmp);
                // print!("{}(", nodey);
            } else if self.chromosome[i].arity() == 1 {
                first.push_str(&nodey);
                // print!("{}", nodey);
            } else {
                let tmp: String = format!("{})", nodey);
                first.push_str(&tmp);
                // print!("{})", nodey);
            }
        }
        println!("{}", &first);
        writeln!(file, "{}", first).expect("write failed");
    }
}

pub mod genetics {
    extern crate rand;
    use crate::base::data::Data;
    use crate::base::individual::Individual;
    use crate::base::individual::Node;
    use rand::{thread_rng, Rng};

    ///Subtree crossover.  Random points chosen for parents 1 and 2
    /// the subtree of parent 1 is replaced by result subtree from parent 2.
    pub fn subtree_cross(parent1: &Individual, parent2: &Individual, datas: &Data) -> Individual {
        let mut child = Individual::new();
        let mut rng = thread_rng();
        let cross_point1: usize = rng.gen_range(0..parent1.size());
        let cross_point2: usize = rng.gen_range(0..parent2.size());
        let sub_parent1: usize = parent1.nodes_subtree(cross_point1);
        let sub_parent2: usize = parent2.nodes_subtree(cross_point2);
        let parent1_left: Vec<Node> = parent1.left_copy_outside(cross_point1);
        let parent2_subtree: Vec<Node> = parent2.subtree_copy(cross_point2, sub_parent2);
        let new_point = cross_point1 + sub_parent1;
        let parent1_right: Vec<Node> = parent1.right_copy_outside(new_point);
        child.insert(parent1_left);
        child.insert(parent2_subtree);
        child.insert(parent1_right);

        //evaluate child tree
        child.outputs_calculate(datas);
        child.eval_fitness(datas);
        if child.train_fitness.unwrap().is_nan() {
            return parent1.clone();
        }
        child.size = Some(child.chromosome.len());
        child.depth_calc();
        //check tree isn't too deep (adjust here if needed)
        if child.depth > Some(20) {
            return parent1.clone();
        }
        child
    }

    ///Single point mutation.  A random subtree is generated using the
    /// grow() method to replace a subtree from parent1
    /// new subtree has max depth of 5
    pub fn single_point_mut(parent1: &Individual, datas: &Data) -> Individual {
        let mut child: Individual = Individual::new();
        let mut rng = thread_rng();
        let mut_point: usize = rng.gen_range(0..parent1.size());
        let parent1_subpoint: usize = parent1.nodes_subtree(mut_point);

        let parent1_left: Vec<Node> = parent1.left_copy_outside(mut_point);
        let mutate: Individual = Individual::grow(5, datas);
        let new_point = mut_point + parent1_subpoint;
        let parent1_right: Vec<Node> = parent1.right_copy_outside(new_point);

        child.insert(parent1_left);
        //only need the chromosome vector from this Individual
        child.insert(mutate.cloned());
        child.insert(parent1_right);

        //evaluate child
        child.outputs_calculate(datas);
        child.eval_fitness(datas);
        if child.train_fitness.unwrap().is_nan() {
            return parent1.clone();
        }
        child.size = Some(child.chromosome.len());
        child.depth_calc();

        //check tree isn't too deep (adjust here if needed)
        if child.depth > Some(20) {
            return parent1.clone();
        }
        child
    }
}
