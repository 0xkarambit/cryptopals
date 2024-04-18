
mod set1;

fn main() {
    println!("Hhmmmm what should the main file do ??");
    print!("Nyaa...\n");
    // Hmm i can use this file for experimenting with code

    let res = std::str::from_utf8(&[0x41, 0x42, 0x43, 0x44]).unwrap();

    dbg!(b'a' + 29);

    dbg!(res);
}
