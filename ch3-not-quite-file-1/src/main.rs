#![allow(unused_variables)]
#![allow(dead_code)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// using '!' as return type indicates to the Function never returns
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // macro that crashes the program if it is encountered
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(f1, vec![])
    close(&mut f1);
}
