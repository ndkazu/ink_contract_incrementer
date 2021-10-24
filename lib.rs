#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
//use ink_storage::collections::{Vec,HashMap,Stash,Bitvec};

#[ink::contract]
mod incrementer {

	#[ink(storage)]
	pub struct Incrementer {
		// Storage Declaration
		value:i32,
		my_value: ink_storage::collections::HashMap<AccountId, i32>,
	}

	impl Incrementer {
		#[ink(constructor)]
		pub fn new(init_value: i32) -> Self {
			// Contract Constructor
			Self{
				value:init_value,
				my_value: ink_storage::collections::HashMap::new(),
			}
		}

		#[ink(constructor)]
		pub fn default() -> Self{
			Self{
				value: 0,
				my_value: Default::default(),
			}
		}

		#[ink(message)]
		pub fn get(&self) -> i32 {
			// Contract Message
			self.value
		}

		#[ink(message)]
		pub fn set(&mut self, by:i32){
			self.value += by;
		}

		#[ink(message)]
		pub fn get_mine(&self) -> i32 {
		let caller = self.env().caller();
		self.my_value_or_zero(&caller)
		}		

		#[ink(message)]
		pub fn inc_mine(&mut self, by:i32){
			let caller = self.env().caller();
			let my_value = self.my_value_or_zero(&caller);
			self.my_value.insert(caller,my_value+by);
		}

		fn my_value_or_zero(&self, of: &AccountId) -> i32 {
			let bb= self.my_value.get(of).unwrap_or(&0);
			*bb
			
		} 
	}

	#[cfg(test)]
	mod tests {
		use super::*;
		use ink_lang as ink;

		#[ink::test]
		fn default_works() {
			// Test Your Contract
		}
	}
}