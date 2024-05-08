//Rust is an aheadOfTime compiled language like you can compile a program and give the executable to someone else, and they can run it even without having Rust installed

//After compiling successfully using rustc fileName (rustc main.rs), Rust outputs a binary executable like filename(main)
fn main(){          //main function is important, always the first code that runs in every executable rust program
    println!("Hello, world!");  //! stands for a macro, so println is a macro , if it is a normal function it would be println()
}