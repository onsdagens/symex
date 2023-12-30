use symex::{general_assembly::RunConfig, run_elf::run_elf};

fn main() {
    println!("Simple WCET analasis");

    let path_to_elf_file = "target/thumbv6m-none-eabi/release/examples/rtic_simple_resourse";
    let function_name = "IO_IRQ_BANK0";

    let config = RunConfig {
        pc_hooks: vec![],
        register_read_hooks: vec![],
        register_write_hooks: vec![],
        memory_write_hooks: vec![],
        memory_read_hooks: vec![],
        show_path_results: false,
    };

    let results = run_elf(path_to_elf_file, function_name, config).unwrap();

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
