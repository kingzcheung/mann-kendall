use mann_kendall::mann_kendall_test;



fn main() {
    let x = vec![434.346100,433.923218,433.712891,434.910614,435.323761,434.119446,434.950653,433.915039,433.49963];
    let res = mann_kendall_test(x, 0.05);

    println!("{}",res.0);
    println!("{}",res.1);
}
 