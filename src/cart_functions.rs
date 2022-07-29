use crate::*;

trait ShoppingCartFunctions {
    fn add_item(&mut self, user: AccountId, item: CartItem);
    fn remove_item(&mut self, user: AccountId, item: CartItem);
    fn clear_cart(&mut self, user: AccountId);
    fn get_cart_items(&self, user: AccountId) -> Vec<CartItem>;
    fn get_cart_total(&self, user: AccountId) -> U128;
}

#[near_bindgen]
impl ShoppingCartFunctions for Contract {
    fn add_item(&mut self, user: AccountId, item: CartItem) {
        let mut cart: Vec<CartItem> = self.cart.get(&user).unwrap_or_default();
        cart.push(item);
        self.cart.insert(&user, &cart);
    }
    fn remove_item(&mut self, user: AccountId, item: CartItem) {
        let mut cart: Vec<CartItem> = self.cart.get(&user).unwrap_or_default();
        cart.retain(|cart_item| cart_item != &item);
        self.cart.insert(&user, &cart);
    }

    fn clear_cart(&mut self, user: AccountId) {
        self.cart.remove(&user);
    }

    fn get_cart_items(&self, user: AccountId) -> Vec<CartItem> {
        self.cart.get(&user).unwrap_or_default().clone()
    }

    fn get_cart_total(&self, user: AccountId) -> U128 {
        let _cart: Vec<CartItem> = self.cart.get(&user).unwrap_or_default();
        //cart.iter().fold(U128(0), |acc, item| acc + item.price)
        U128(0)
    }
}
