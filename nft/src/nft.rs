use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;

struct Controller(Principal);
impl Default for Controller {
    fn default() -> Self {
        panic!("Cannot set a default controller!")
    }
}

#[init]
fn init() {
    let ic = get_context();
    let controller = ic.caller();
    ic.store(Controller(controller));
}

fn is_controller(account: &Principal) -> bool {
    account == &get_context().get::<Controller>().0
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct NftCanister {
    principal_id: Principal,
    name: String,
    standard: String,
}

#[derive(Default)]
pub struct Registry(HashMap<String, NftCanister>);

impl Registry {
    pub fn add(&mut self, name: String, canister_info: NftCanister) -> String {
        self.0.insert(name, canister_info);
        String::from("Operation was successful.")
    }

    pub fn remove(&mut self, name: &String) -> String {
        if self.0.contains_key(name) {
            self.0.remove(name);
            return String::from("Operation was successful.");
        }

        String::from("No such entry exists in the registry.")
    }

    pub fn edit(
        &mut self,
        name: &String,
        principal_id: Option<Principal>,
        standard: Option<String>,
    ) -> String {
        match self.0.get_mut(name) {
            None => String::from("The canister you want to change does not exist in the registry."),
            Some(canister) => {
                if principal_id.is_some() {
                    canister.principal_id = principal_id.unwrap();
                } else {
                    canister.standard = standard.unwrap();
                }
                return String::from("Operation was successful.");
            }
        }
    }

    pub fn get_canister(&self, name: &String) -> Option<&NftCanister> {
        self.0.get(name)
    }

    pub fn get_all(&self) -> Vec<&NftCanister> {
        self.0.values().collect()
    }
}

#[query]
fn name() -> String {
    String::from("NFT Registry Canister")
}

#[update]
fn add(canister_info: NftCanister) -> String {
    let ic = get_context();
    if !is_controller(&ic.caller()) {
        return String::from("You are not authorized to make changes.");
    }

    let name = canister_info.name.clone();
    if name.len() <= 120 {
        let db = ic.get_mut::<Registry>();
        return db.add(name, canister_info);
    }

    String::from("The name of this canister has exceeded the limitation of 120 characters.")
}

#[update]
fn remove(name: String) -> String {
    let ic = get_context();
    if !is_controller(&ic.caller()) {
        return String::from("You are not authorized to make changes.");
    }

    let db = ic.get_mut::<Registry>();
    db.remove(&name)
}

#[update]
fn edit(name: String, principal_id: Option<Principal>, standard: Option<String>) -> String {
    let ic = get_context();
    if !is_controller(&ic.caller()) {
        return String::from("You are not authorized to make changes.");
    }

    if principal_id.is_none() && standard.is_none() {
        return String::from(
            "You should pass at least one of the principal_id or standard parameters.",
        );
    } else {
        let db = ic.get_mut::<Registry>();
        return db.edit(&name, principal_id, standard);
    }
}

#[update]
fn get_canister(name: String) -> Option<&'static NftCanister> {
    let ic = get_context();
    let db = ic.get_mut::<Registry>();
    db.get_canister(&name)
}

#[update]
fn get_all() -> Vec<&'static NftCanister> {
    let ic = get_context();
    let db = ic.get_mut::<Registry>();
    db.get_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_kit::*;

    #[test]
    fn test_controller() {
        // alice is the controller
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        let mut addition = add(canister_info.clone());
        assert_eq!(addition, String::from("Operation was successful."));
        remove(String::from("xtc"));

        ctx.update_caller(mock_principals::bob());
        addition = add(canister_info);
        assert_eq!(
            addition,
            String::from("You are not authorized to make changes.")
        );
    }

    #[test]
    fn test_add() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert_eq!(
            add(canister_info),
            String::from("Operation was successful.")
        );
    }

    #[test]
    fn test_remove() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert_eq!(
            add(canister_info),
            String::from("Operation was successful.")
        );

        assert_eq!(
            remove(String::from("xtc")),
            String::from("Operation was successful.")
        );
    }

    #[test]
    fn test_get_canister() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert_eq!(
            add(canister_info.clone()),
            String::from("Operation was successful.")
        );

        assert_eq!(get_canister(String::from("xtc")).unwrap(), &canister_info);
        assert!(get_canister(String::from("dab")).is_none());
    }

    #[test]
    fn test_get_all() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert_eq!(
            add(canister_info.clone()),
            String::from("Operation was successful.")
        );

        assert_eq!(get_all(), vec![&canister_info]);
    }
}
