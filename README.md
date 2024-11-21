# corroded_gp

### Architecture
The overall structure of this GP system is intended to be modular in nature. Various methods such
as for selection or fitness can be changed by the user and called from the central control part of the
software. There are the following initial components to the system:
- main.rs - Primary entry point to the software, where dataset is established and parameters
selected for the evolution
- gp.rs - The next level of control flow to the software, containing initialization and control
elements
- pop.rs - File containing the logic surrounding the population of Individuals
- individual.rs - File containing the expression tree representation
- functions.rs - File to hold all function terminal logic as well as fitness
- data.rs - File to manage dataset manipulation

### Control Flow
The control flow of the system takes place in the evolve.rs file, and this file contains the following
primary functions:
- new() - initialization method for the GP struct, contains evolution parameters
- set() methods - setter methods for all the parameters of evolution
- init pop() - method to initialize the population, calls the chosen generation method
- print state() - displays state information such as fitness and depth
The Gp struct contains all the basic adjustable parameters for the running of the system. It’s
primary methods are the print state() method which outputs the current state of the run, and the
evolve method. The evolve method initializes the output file, then goes through a series of steps
in the evolutionary process. The population is replaced at every generation, so it first takes an
initialized population and for each generation selects the parents for the new population. If the
random number generated meets the threshold for crossover then two parents are used in a standard
subtree crossover. Otherwise the first parent is passed to a single point mutation method. Which
will be discussed later on. After the genetic operators have been called the new population is sorted
by its fitness and its current state is printed to the terminal and to file. After all evolution the best
performing individual then has its expression tree printed to the console and to the results.txt file.


## User Guide

### Basic Operation
The basic default settings for a run of the GP is 80% crossover and 20% mutation with a pop-
ulation of 200 and a tournament size of 3. These can be modified in a straight forward manner as
per figure 3. In this snippet a symbolic regression data file is loaded in to a data struct, at which
point a Gp struct instance is initialized with that dataset. To show customization the population is
then set to 1000 and the population and crossover chances are set.

The default initial max depth of an expression tree is 5, but this is easily modified by the used.
The maximum tree depth is currently set to 20, but the speed of the program is such that this
could be increased if needed. However like in many systems, bloat is a serious issue for the trees
generated with this GP system so increasing the depth too much would only exasperate that.
It is important to note with Datasets used in this software, that as of now the system expects
the last column of data to be the expected output or label for the data. So for a symbolic regression
example the first column will be a range of *x* values and the second column will be the expected outputs for those *x* values.
Output of the program is to the terminal and to a text file named ”results.txt”. This can be
changed by the user by replacing that file name in the gp.rs file, in the evolve() method.

### Usage

```rust
fn main() {
    //When establishing dataset need a string for the file path and a float giving what percent to use for testing
    let dataset = Data::new("sin-data.txt", 0.5);
    let mut gp = Gp::new(dataset)
        .set_pop(1000)
        .set_tourn_size(5)
        .set_cross_chance(0.8);

    gp.init_pop();
    gp.evolve(100);
}
```
### Changing Fitness
To modify the fitness method there needs to be a couple of simple modifications.
The first modification would be made as per figure 4 and where root mean squared() is called
the user can change that for their fitness function of choice. Currently the sum of absolute error
is also available and would be called with ”sae()”. If one wanted to add their own fitness value,
the only additional step that would be needed would be to add the new fitness function to the
functions.rs file and make the above change to the evaluate function.
The second change to fitness would be to change the fitness from a minimization fitness as it
currently is, into a maximization function. To do this we make the following adjustment to the
tournament() method in the pop.rs file. Referring to figure 5, the second last line would be changed
from ”child = tourn[0].clone();” into ”child = tourn[tourn.len() - 1].clone();”. Since the list of fit-
nesses is sorted in ascending fashion this would return the largest value.

### Changing Function Language
The next big customization a user might want to perform would be to modify the function language
used by the GP system. To add a new function terminal for the GP to use the first step would be
to implement the new operation in the functions.rs file. After that there would need to be 4 main
changes made to the rest of the system in the individual.rs file.

First of all changes would be made to the Node Enum. Secondly the new function would be added there as well. Third there would need to be additions to the
node operations, and finally the new function terminal would need to be added to
the random function method for the Node enum as well.
.
