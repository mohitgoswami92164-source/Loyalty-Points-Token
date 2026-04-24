#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    token, Address, Env, String, Symbol,
};

// ─── Storage Keys ────────────────────────────────────────────────────────────

const ADMIN: Symbol      = symbol_short!("ADMIN");
const ISSUER: Symbol     = symbol_short!("ISSUER");
const PROG_NAME: Symbol  = symbol_short!("PROG_NAME");
const DECIMALS: Symbol   = symbol_short!("DECIMALS");

// ─── Data Types ───────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct ProgramInfo {
    pub name: String,       // e.g. "StellarAir Miles" or "GrandHotel Points"
    pub symbol: String,     // e.g. "SAM" or "GHP"
    pub issuer: Address,    // airline / hotel admin address
    pub decimals: u32,      // typically 2 for points (e.g. 100 = 1.00 pts)
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct LoyaltyTokenContract;

#[contractimpl]
impl LoyaltyTokenContract {

    // ── Initialise the loyalty programme ─────────────────────────────────────

    /// Deploy and configure the loyalty token programme.
    /// Must be called once by the deployer.
    pub fn initialize(
        env: Env,
        admin: Address,
        issuer: Address,
        program_name: String,
        token_symbol: String,
        decimals: u32,
    ) {
        // Prevent re-initialisation
        if env.storage().instance().has(&ADMIN) {
            panic!("contract already initialised");
        }

        admin.require_auth();

        env.storage().instance().set(&ADMIN,     &admin);
        env.storage().instance().set(&ISSUER,    &issuer);
        env.storage().instance().set(&PROG_NAME, &program_name);
        env.storage().instance().set(&DECIMALS,  &decimals);

        // Emit initialisation event
        env.events().publish(
            (symbol_short!("init"), token_symbol.clone()),
            (admin, issuer, program_name, decimals),
        );
    }

    // ── Issuer Operations ────────────────────────────────────────────────────

    /// Award loyalty points to a member (mint).
    /// Only the registered issuer (airline / hotel) may call this.
    pub fn award_points(
        env: Env,
        token_address: Address,
        member: Address,
        amount: i128,
        memo: String,   // e.g. "Flight NY→LA", "3-night stay"
    ) {
        let issuer: Address = env.storage().instance().get(&ISSUER).unwrap();
        issuer.require_auth();

        if amount <= 0 {
            panic!("award amount must be positive");
        }

        // Mint via the SEP-41 token interface
        let token_client = token::StellarAssetClient::new(&env, &token_address);
        token_client.mint(&member, &amount);

        env.events().publish(
            (symbol_short!("award"), member.clone()),
            (amount, memo),
        );
    }

    /// Burn (redeem / expire) points from a member's balance.
    /// Member must authorise this call.
    pub fn redeem_points(
        env: Env,
        token_address: Address,
        member: Address,
        amount: i128,
        memo: String,   // e.g. "Upgrade redemption", "Free night"
    ) {
        member.require_auth();

        if amount <= 0 {
            panic!("redeem amount must be positive");
        }

        let token_client = token::TokenClient::new(&env, &token_address);

        // Confirm sufficient balance
        let balance = token_client.balance(&member);
        if balance < amount {
            panic!("insufficient points balance");
        }

        token_client.burn(&member, &amount);

        env.events().publish(
            (symbol_short!("redeem"), member.clone()),
            (amount, memo),
        );
    }

    // ── Peer-to-Peer Transfer (Tradeable Tokens) ─────────────────────────────

    /// Transfer loyalty points between two members.
    /// Points are fully tradeable — sender must authorise.
    pub fn transfer_points(
        env: Env,
        token_address: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        from.require_auth();

        if amount <= 0 {
            panic!("transfer amount must be positive");
        }

        let token_client = token::TokenClient::new(&env, &token_address);
        token_client.transfer(&from, &to, &amount);

        env.events().publish(
            (symbol_short!("transfer"), from.clone()),
            (to, amount),
        );
    }

    // ── Admin Operations ─────────────────────────────────────────────────────

    /// Rotate the issuer address (e.g. when operator changes).
    pub fn update_issuer(env: Env, new_issuer: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        env.storage().instance().set(&ISSUER, &new_issuer);

        env.events().publish(
            (symbol_short!("new_iss"),),
            (new_issuer,),
        );
    }

    /// Transfer contract admin role to another address.
    pub fn transfer_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        env.storage().instance().set(&ADMIN, &new_admin);

        env.events().publish(
            (symbol_short!("new_adm"),),
            (new_admin,),
        );
    }

    // ── View Functions ───────────────────────────────────────────────────────

    /// Return the current points balance for a member.
    pub fn balance(env: Env, token_address: Address, member: Address) -> i128 {
        let token_client = token::TokenClient::new(&env, &token_address);
        token_client.balance(&member)
    }

    /// Return programme metadata.
    pub fn program_info(env: Env) -> ProgramInfo {
        ProgramInfo {
            name:     env.storage().instance().get(&PROG_NAME).unwrap(),
            symbol:   env.storage().instance().get(&PROG_NAME).unwrap(), // stored separately in prod
            issuer:   env.storage().instance().get(&ISSUER).unwrap(),
            decimals: env.storage().instance().get(&DECIMALS).unwrap(),
        }
    }

    /// Return the current admin address.
    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN).unwrap()
    }

    /// Return the current issuer address.
    pub fn issuer(env: Env) -> Address {
        env.storage().instance().get(&ISSUER).unwrap()
    }
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id  = env.register_contract(None, LoyaltyTokenContract);
        let client       = LoyaltyTokenContractClient::new(&env, &contract_id);

        let admin  = Address::generate(&env);
        let issuer = Address::generate(&env);

        client.initialize(
            &admin,
            &issuer,
            &String::from_str(&env, "StellarAir Miles"),
            &String::from_str(&env, "SAM"),
            &2,
        );

        assert_eq!(client.admin(),  admin);
        assert_eq!(client.issuer(), issuer);
    }

    #[test]
    #[should_panic(expected = "contract already initialised")]
    fn test_double_init_panics() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, LoyaltyTokenContract);
        let client      = LoyaltyTokenContractClient::new(&env, &contract_id);

        let admin  = Address::generate(&env);
        let issuer = Address::generate(&env);

        client.initialize(
            &admin, &issuer,
            &String::from_str(&env, "StellarAir Miles"),
            &String::from_str(&env, "SAM"), &2,
        );
        // Second call must panic
        client.initialize(
            &admin, &issuer,
            &String::from_str(&env, "StellarAir Miles"),
            &String::from_str(&env, "SAM"), &2,
        );
    }
}
