#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

// Symbol for global count
const BOX_COUNT: Symbol = symbol_short!("BOXCNT");

#[contracttype]
#[derive(Clone)]
pub struct GiftBox {
    pub box_id: u64,
    pub owner: String,
    pub contents: String,
    pub customized: bool,
}

#[contracttype]
pub enum GiftRegistry {
    Box(u64),
}

#[contract]
pub struct GiftBoxContract;

#[contractimpl]
impl GiftBoxContract {
    // Create a new customizable gift box
    pub fn create_box(env: Env, owner: String, contents: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&BOX_COUNT).unwrap_or(0);
        count += 1;

        let gift_box = GiftBox {
            box_id: count,
            owner,
            contents,
            customized: false,
        };

        env.storage().instance().set(&GiftRegistry::Box(count), &gift_box);
        env.storage().instance().set(&BOX_COUNT, &count);
        count
    }

    // Customize an existing gift box
    pub fn customize_box(env: Env, box_id: u64, new_contents: String) {
        let mut gift_box: GiftBox = env.storage().instance()
            .get(&GiftRegistry::Box(box_id))
            .expect("Gift box not found");

        gift_box.contents = new_contents;
        gift_box.customized = true;
        env.storage().instance().set(&GiftRegistry::Box(box_id), &gift_box);
    }

    // View gift box details by ID
    pub fn view_box(env: Env, box_id: u64) -> GiftBox {
        env.storage().instance()
            .get(&GiftRegistry::Box(box_id))
            .expect("Gift box not found")
    }
}
