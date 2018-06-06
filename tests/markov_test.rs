extern crate markov;

use markov::gen;

#[test]
fn test_gen() {
    let input = "Welcome to the jungle we've got fun and games We got everything you want honey, 
    we know the names We are the people that can find whatever you may need If you got the money, honey we got your disease.";

    let res = gen(input, "welcome", 50);

    println!("{}", res.unwrap().join(" "));
}