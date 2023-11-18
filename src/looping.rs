
pub fn loop_testing() {
    let command = [('G', 'H'), ('H', '5')];

    for i in 0..3 {
        print!(" {} ", i);
        println!("{:?}", command.get(i));
    }
}