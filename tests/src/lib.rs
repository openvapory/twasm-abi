#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), feature(alloc))]
#![feature(proc_macro)]
#![cfg(test)]

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::vec::Vec;

extern crate pwasm_abi;
extern crate pwasm_abi_derive;

use pwasm_abi_derive::legacy_dispatch;

type U256 = [u8; 32];
type H256 = [u8; 32];

#[legacy_dispatch]
trait TestContract {
	fn baz(&mut self, _p1: u32, _p2: bool);
	fn boo(&mut self, _arg: u32) -> u32;
	fn sam(&mut self, _p1: Vec<u8>, _p2: bool, _p3: Vec<U256>);
}

const PAYLOAD_SAMPLE_1: &[u8] = &[
	0xcd, 0xcd, 0x77, 0xc0,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
];

#[test]
fn baz_dispatch() {
	#[derive(Default)]
	struct TestContractInstance {
		called: bool,
		called_wrong: bool,
	}

	impl TestContract for TestContractInstance {
		fn baz(&mut self, p1: u32, p2: bool) {
			assert_eq!(p1, 69);
			assert_eq!(p2, true);
			self.called = true;
		}
		fn boo(&mut self, _arg: u32) -> u32 {
			self.called_wrong = true;
			0
		}
		fn sam(&mut self, _p1: Vec<u8>, _p2: bool, _p3: Vec<[u8; 32]>) {
			self.called_wrong = true;
		}
	}

	let instance = TestContractInstance::default();
	let mut endpoint = Endpoint::new(instance);
	let result = endpoint.dispatch(PAYLOAD_SAMPLE_1);

	assert_eq!(result, Vec::new());

	assert!(instance.called);
	assert!(!instance.called_wrong);
}

#[test]
fn sam_dispatch() {
	#[derive(Default)]
	struct TestContractInstance {
		called: bool,
		called_wrong: bool,
	}

	impl TestContract for TestContractInstance {
		fn sam(&mut self, p1: Vec<u8>, p2: bool, p3: Vec<U256>) {
			assert_eq!(p1, vec![100, 97, 118, 101]);
			self.called = true;
		}
		fn baz(&mut self, _p1: u32, _p2: bool) {
			self.called_wrong = true;
		}
		fn boo(&mut self, _arg: u32) -> u32 {
			self.called_wrong = true;
			0
		}		
	}

	let instance = TestContractInstance::default();
	let mut endpoint = Endpoint::new(instance);
	let result = endpoint.dispatch(PAYLOAD_SAMPLE_1);

	assert_eq!(result, Vec::new());

	assert!(instance.called);
	assert!(!instance.called_wrong);
}