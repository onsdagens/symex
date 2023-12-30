use symex::{
    general_assembly::{project::MemoryHookAddress, state::GAState, Result, RunConfig},
    run_elf::run_elf,
    smt::DExpr,
};

// This example show how hooks can be used to get at which cycle a resource is locked and unlocked in a simple
// RTIC application. To keep in mind is that cycles are added after the instruction is executed and the hook
// is run during instruction execution. Therefore care needs to be taken to measure the critical section
// correctly.

// To run the example first build the "rtic_simple_resourse" in armv6-m-examples by doing:
// cd armv6-m-examples
// cargo build --release --example rtic_simple_resourse
// cd ..
//
// Then run the analysis by: cargo run -p wcet-analasis-examples --release --example wcet_resource_times

fn main() {
    println!("Simple WCET analasis");

    // path to the elf file to analyse.
    let path_to_elf_file = "target/thumbv6m-none-eabi/release/examples/rtic_simple_resourse";
    // name of the task in the elf file (same as associated interrupt vector for HW tasks).
    let function_name = "IO_IRQ_BANK0";

    // Hook to run when the interrupt mask is reset (looked).
    let lock_hook: fn(state: &mut GAState, addr: u64, value: DExpr, bits: u32) -> Result<()> =
        |state, addr, value, bits| {
            // save the current cycle count to the laps vector.
            state
                .cycle_laps
                .push((state.cycle_count, "lock".to_owned()));
            Ok(())
        };

    // Hook to run when the interrupt mask is set (unlocked).
    let unlock_hook: fn(state: &mut GAState, addr: u64, value: DExpr, bits: u32) -> Result<()> =
        |state, addr, value, bits| {
            // save the current cycle count to the laps vector.
            state
                .cycle_laps
                .push((state.cycle_count, "unlock".to_owned()));
            Ok(())
        };

    // create a run configuration with the hooks associated with the correct addresses.
    let config = RunConfig {
        pc_hooks: vec![],
        register_read_hooks: vec![],
        register_write_hooks: vec![],
        memory_write_hooks: vec![
            (MemoryHookAddress::Single(0xe000e100), unlock_hook),
            (MemoryHookAddress::Single(0xe000e180), lock_hook),
        ],
        memory_read_hooks: vec![],
        show_path_results: false,
    };

    // run the symbolic execution
    let results = run_elf(path_to_elf_file, function_name, config).unwrap();

    // Find the longest path and print out the saved cycle counts for lock and unlock.
    let mut max = 0;
    let paths = results.len();
    for result in results {
        println!("cycle laps: {:?}", result.cycle_laps);
        max = max.max(result.max_cycles);
    }

    println!(
        "Found {} paths and the longest path takes {} cycles.",
        paths, max
    );
}
