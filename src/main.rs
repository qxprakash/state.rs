mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	// Constructor for the Runtime struct
	fn new() -> Self {
		// Initialize a new instance of Runtime with default values
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
	runtime.balances.set_balance(&alice, 100);

	// start emulating a block
	/* TODO: Increment the block number in system. */
	/* TODO: Assert the block number is what we expect. */

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	/* TODO: Increment the nonce of `alice`. */
	runtime.system.inc_nonce(&alice);
	/* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
		- The transfer _could_ return an error. We should use `map_err` to print
		  the error if there is one.
		- We should capture the result of the transfer in an unused variable like `_res`.
	*/
	let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| println!("{}", e));

	// second transaction
	/* TODO: Increment the nonce of `alice` again. */
	runtime.system.inc_nonce(&alice);
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	println!("final state of runtime instance --> {:#?}", runtime);
}
