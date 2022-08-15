use near_sdk::require;

use crate::*;

#[near_bindgen]
impl Contract {
    pub fn view_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn view_admins(&self) -> Vec<AccountId> {
        self.admins.as_vector().clone().to_vec()
    }

    pub fn add_admin(&mut self, admin: AccountId) {
        require!(!self.admins.contains(&admin), "Admin is already an admin");
        require!(self.owner == env::signer_account_id(), "Only the owner can add admins");
        self.admins.insert(&admin);
    }

    pub fn remove_admin(&mut self, admin: AccountId) {
        require!(self.admins.contains(&admin), "Admin is not an admin");
        require!(self.owner == env::signer_account_id(), "Only the owner can remove admins");
        self.admins.remove(&admin);
    }

    pub fn is_admin(&self, admin: AccountId) -> bool {
        self.admins.contains(&admin)
    }

    pub fn is_owner(&self, owner: AccountId) -> bool {
        self.owner == owner
    }
    
    pub fn add_admins(&mut self, admins: Vec<AccountId>) {
        require!(self.owner == env::signer_account_id(), "Only the owner can add admins");
        for admin in admins {
            self.add_admin(admin);
        }
    }

    pub fn remove_admins(&mut self, admins: Vec<AccountId>) {
        require!(self.owner == env::signer_account_id(), "Only the owner can remove admins");
        for admin in admins {
            self.remove_admin(admin);
        }
    }

    pub fn change_owner(&mut self, new_owner: AccountId) {
        require!(self.owner == env::signer_account_id(), "Only the owner can change the owner");
        self.owner = new_owner;
    }
}