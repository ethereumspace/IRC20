use chain_cloud_util::emit;
use chain_cloud_util::event::EventTrait;
use chain_cloud_util::event_macro::Event;
use ic_cdk::api;
use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::HashMap;
static mut TotalSupply: Option<Nat> = None;
static mut CurrentSupply: Option<Nat> = None;
static mut Symbol: Option<String> = None;
type BalanceOf = HashMap<Principal, Nat>;
#[derive(Event)]
struct MintEvent {
    method_name: String,
    memo: String,
}

#[derive(Event)]
struct TransferEvent {
    method_name: String,
    memo: String,
}
#[init]
fn init(total_supply: Nat, symbol: String) {
    unsafe {
        TotalSupply = Some(total_supply);
        CurrentSupply = Some(0.into());
        Symbol = Some(symbol);
    }
}
#[update]
async fn mint(account: Principal, amount: Nat) -> () {
    unsafe {
        let mut current = CurrentSupply.as_mut().unwrap().clone();
        let total = TotalSupply.as_mut().unwrap().clone();
        current = current + amount.clone();
        if current > total {
            api::trap("failed to mint");
        }
        CurrentSupply = Some(current)
    }
    let balance_table = storage::get_mut::<BalanceOf>();
    let balance = balance_table.get(&account).cloned().unwrap_or_else(|| Nat::default());
    let new_balance = balance + amount.clone();
    balance_table.insert(account, new_balance);
    let memo = format!("{:?} mint {:?} token", account.to_string(), amount);
    api::print(&memo);
    let mint_event = MintEvent {
        method_name: "mint".to_string(),
        memo: memo,
    };
    emit(mint_event).await;
}

#[update]
async fn transfer(to: Principal, amount: Nat) -> () {
    let caller = api::caller();
    let balance_table = storage::get_mut::<BalanceOf>();
    let balance = balance_table.get(&caller).cloned().unwrap_or_else(|| Nat::default());
    if balance < amount {
        api::trap("Sorry, your credit is running low")
    }
    let new_balance = balance - amount.clone();
    balance_table.insert(caller, new_balance);
    let to_balance = balance_table.get(&to).cloned().unwrap_or_else(|| Nat::default());
    let to_new_balance = to_balance + amount.clone();
    balance_table.insert(to, to_new_balance);
    let memo = format!("{:?} transfer {:?}",caller.to_string(),amount);
    api::print(&memo);
    let transfer_event = TransferEvent {
        method_name: "transfer".to_string(),
        memo: memo,
    };
    emit(transfer_event).await;
}

#[query]
fn balance_of(account: Principal) -> Nat {
    let balance_table = storage::get::<BalanceOf>();
    balance_table.get(&account).cloned().unwrap_or_else(|| Nat::default())
}

#[query]
fn symbol() -> String {
    unsafe{
        Symbol.as_ref().unwrap().clone()
    }
}

#[query]
fn current_supply() -> Nat {
    unsafe{
        CurrentSupply.as_ref().unwrap().clone()
    }
}

#[query]
fn total_supply() -> Nat {
    unsafe{
        TotalSupply.as_ref().unwrap().clone()
    }
}

#[post_upgrade]
fn post_update() {
    unsafe {
        TotalSupply = Some(1000000.into());
        CurrentSupply = Some(0.into());
        Symbol = Some("ICP".to_string());
    }
}