
use distrs::Normal;

fn main() {
    //let x = vec![404.175842,421.266602,439.772827,453.741455,458.643982,469.749268,475.998474,479.279663,486.617310,487.517456,491.321625];
    let x = vec![544.,545.,545.,545.,545.,543.,545.,545.,545.,545.,544.,445.,445.,345.,345.,245.,245.,145.,145.,45.];
    let x = vec![545.099304,545.068542,545.714050,545.292847,545.789612,545.312500,545.127991,545.056091,545.027588,545.015930,545.671631,545.268555,545.102173,545.039062,545.015137,545.668091,545.913147,545.340820,545.122070,545.039001];
    let res = mann_kendall::test(x, 0.05);

    println!("p-value:: {}",res.0);
    println!("z:: {}",res.1);
    println!("h:: {}",res.2);

    // 结果应该是1.959963984540054
    // println!("ppf:: {}",Normal::ppf(0.975, 0f64, 1f64))
}
 