struct Args<'a>(&'a [String]);

struct AllArgs<'a, 'b> {
    prog: &'a String,
    args: Args<'b>,
}

impl<'a, 'b> AllArgs<'a, 'b> {
    fn nth(&self, n: usize) -> Option<&String> {
        if n == 0 {
            Some(self.prog)
        } else {
            self.args.0.get(n - 1)
        }
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let prog = argv[0].clone();
    let args = Args(&argv[1..]);
    let all_args = AllArgs { prog: &prog, args };
    println!("{}", all_args.nth(0).unwrap());
}
