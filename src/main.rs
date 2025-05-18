mod modules;
use modules::vm::Vm;
use modules::utils::{ center_print, debug, parse_args, simple_rand };
use modules::parser::parse_program;

fn main() {
    let args = parse_args();
    let file_path = args.get(1).map(|s| s.as_str());
    let debug_f = args.get(2).map(|s| s.as_str());
    let (program, debug_mode) = parse_program(file_path);
    let mut vm = Vm::new();
    vm.load_program(program);

    center_print("OUTPUT", 80);

    let start_time = std::time::Instant::now();
    vm.run();
    let elapsed_time = start_time.elapsed();
    center_print("EXECUTION FINISHED", 80);
    if debug_f == Some("-d") || debug_mode {
        debug(elapsed_time, &vm);
    }

    println!("{}", simple_rand());
}
