use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::BTreeMap;
use serde::{Deserialize};
use ic_cdk_macros::*;
use ic_cdk::*;

/**
Every item in the map looks like this:
( ( Principal,  String       ), Principal  )
( ( UserID,     CanisterName ), CanisterID )
**/

#[derive(Deserialize, CandidType, Clone)]
pub struct GetAddressResult {
    canister_name: String,
    canister_id: Option<Principal>,
}

type Key = (Principal, String);
pub struct AddressBook(BTreeMap<Key, Principal>);

impl Default for AddressBook {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl AddressBook {
    pub fn add_address(&mut self, account: Principal, canister_name: String, canister_id: Principal) {
        let pointer: Key = (account, canister_name);
        self.0.insert(
            pointer,
            canister_id
        );
    }

    pub fn remove_address(&mut self, account: Principal, canister_name: String) {
        let pointer: Key = (account, canister_name);
        self.0.remove(&pointer);
    }
    
    pub fn get_address(&mut self, account: Principal, canister_name: String) -> GetAddressResult {
        let pointer: Key = (account, canister_name.clone());
        let canister_id: Option<Principal> = self.0.get(&pointer).cloned();
        GetAddressResult { canister_name: canister_name, canister_id: canister_id }
    }

    pub fn remove_all(&mut self, account: Principal) {
        // binary_search
    }

    pub fn get_all(&mut self, account: Principal) {
        // binary_search
    }
}

#[query]
fn name() -> String {
    String::from("DAB")
}

#[update]
fn add_address(canister_name: String, canister_id: Principal) {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.add_address(caller(), canister_name, canister_id);
}

#[update]
fn remove_address(canister_name: String) {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.remove_address(caller(), canister_name);
}

#[update]
fn get_address(canister_name: String) -> GetAddressResult {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.get_address(caller(), canister_name)
}

#[update]
fn remove_all() {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.remove_all(caller());
}

#[update]
fn get_all() {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.get_all(caller());
}
