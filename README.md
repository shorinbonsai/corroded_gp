# corroded_gp


###Usage
'''rust
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
'''
