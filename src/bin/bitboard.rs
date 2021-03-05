use chsslib::Bitboard;

fn main() {
    let x = Bitboard::new(0);
    println!("{}", x);

    let a = 'A' as u8;
    print!("enum Square {{\n");
    for rank in (0..8).rev() {
        print!("\t");
        for file in 0..8 {
            print!("{}{}, ", (a + file) as char, rank + 1);
        }
        print!("\n");
    }
    print!("}}");

    print!("\n{}", (('a' as u8) + 1) as char)
}
