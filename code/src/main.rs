mod help;

fn main() {
    help::print_usage(&mut std::io::stdout()).expect("Unable to print usage");
}
