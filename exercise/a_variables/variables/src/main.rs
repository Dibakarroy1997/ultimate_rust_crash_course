// Part 1
// fn main() {
//     let missiles = 8;
//     let ready = 2;
//     println!("Firing {} of my {} missiles...", ready, missiles);
// }

// Part 2
// const STARTING_MISSILES: i32 = 8;
// const READY_AMOUNT: i32 = 2;

// fn main() {
//     let mut missiles: i32 = STARTING_MISSILES;
//     let ready: i32 = READY_AMOUNT;
//     println!("Firing {} of my {} missiles...", ready, missiles);
//     missiles = missiles - ready;
//     println!("{} missiles left", missiles);
// }

// Extra challenge
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
