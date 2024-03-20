use symex::{general_assembly::RunConfig, run_elf::run_elf};
fn main() {
    println!("Simple WCET analasis");
    use tracing::{span, Level};
    use tracing_subscriber::FmtSubscriber;
//let span = span!(Level::TRACE, "my_span");
// `enter` returns a RAII guard which, when dropped, exits the span. this
// indicates that we are in the span for the current lexical scope.
//let _enter = span.enter();
    let path_to_elf_file = "/home/pawel/symex/symex/target/riscv32i-unknown-none-elf/release/examples/simple";
    let function_name = "rust_simple_test";
    //let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
    //    .with_max_level(Level::TRACE)
        // completes the builder.
    //    .finish();

    //tracing::subscriber::set_global_default(subscriber)
    //    .expect("setting default subscriber failed");
    let config = RunConfig {
        pc_hooks: vec![],
        register_read_hooks: vec![],
        register_write_hooks: vec![],
        memory_write_hooks: vec![],
        memory_read_hooks: vec![],
        show_path_results: false,
    };

    let results = run_elf(path_to_elf_file, function_name, config).unwrap();
   // println!("{:?}", results);
    let mut max = 0;
    let paths = results.len();
    for result in results {
        max = max.max(result.max_cycles);
    }

    println!(
        "Found {} paths and the longest path takes {} cycles.",
        paths, max
    );
}
