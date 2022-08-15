use near_sdk::require;

use crate::*;
//const GAS_FOR_PARAS_BUY: Gas = Gas(1011280000000000000000000);

trait BuyFromOtherMarketplaces {
    fn buy_from_other_marketplaces(&mut self, user: AccountId, item: TokenId, price: U128);
}

#[near_bindgen]
impl BuyFromOtherMarketplaces for Contract {

    #[payable]
    fn buy_from_other_marketplaces(&mut self, user: AccountId, item: TokenId, price: U128) {
        require!(env::attached_deposit() >= price.0 + ONE_NEAR, "No depositaste el precio + 1 NEAR");
        let cart: Vec<CartItem> = self.cart.get(&user).unwrap_or_default();
        
        let mut buy_item: CartItem = CartItem {
            token_id: item.clone(),
            contract_id: AccountId::new_unchecked("".to_string()),
        };
        for cart_item in cart {
            if cart_item.token_id == item {
                buy_item.contract_id = cart_item.contract_id.clone();
                break;
            };
        };
        if buy_item.contract_id == AccountId::new_unchecked("".to_string()) {
            return;
        } else {
            //self.remove_item(user.clone(), buy_item.clone());
            ext_contract::ext(buy_item.contract_id.clone()).with_attached_deposit(
                price.0 + ONE_NEAR,
            ).with_static_gas(GAS_FOR_NFT_TRANSFER).nft_buy(buy_item.token_id.clone());
            //Add callback for send the nft to the user
        }
        
    }

}