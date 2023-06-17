use proconio::input;

fn main() {
    input! {N: usize, W: [String;N]}
    let V = ["and", "not", "that", "the", "you"];
    let mut flag = 0;
    for w in W {
        for v in &V {
            if w == v.to_string() {
                flag = 1
            }
        }
    }
    if flag == 1 {
        println!("Yes")
    } else {
        println!("No")
    }
}
