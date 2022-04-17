use crate::*;
//Evitamos colisión de la data generando un prefijo para cada colección de storage
//Avoiding data collisition generate a prefix for the storage collections
pub (crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl Contract {
    //Removiendo la venta del marketplace
    //Removing sale from the market, return the removed object
    pub(crate) fn internal_remove_sale(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
    ) -> Sale {
        //Obtener el sale ID único (contract + DELIMITER + token ID)
        //Get the unique sale ID (contract + DELIMITER + token ID)
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        //Obtener el objeto sale removiendo el sale ID unico, si no existe, panic.
        //Get the sale object removing the unique sale ID, if there is no sale, panic.
        let sale = self.sales.remove(&contract_and_token_id).expect("No sale found");
        //Obtener el set de ventas del sales owner, si no existe, panic.
        //Get the set of sales for the sales owner, if there is no set of sales, panic.
        let mut by_owner_id = self.by_owner_id.get(&sale.owner_id).expect("No sales by owner found");
        //Removiendo el sale ID der set de sales del owner.
        //Removing the sale ID from the set of sales for the sales owner.
        by_owner_id.remove(&contract_and_token_id);
        //Si ahora las sales están vacias, remover el set de sales del owner.
        //If now the sales are empty we remove this owner from the map
        if by_owner_id.is_empty() {
            self.by_owner_id.remove(&sale.owner_id);
        } else { 
            //Si aun hay sales, actualizar el set de sales del owner.
            //If there are still sales for this owner, we update the map
            self.by_owner_id.insert(&sale.owner_id, &by_owner_id);
        }

        //Obtener el set de token IDs por sale por el ID del contract NFT, si no existe, panic.
        //Get the set of token IDs by sale for the NFT contract ID, if there is no sale, panic.
        let mut by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id).expect("No sales by nft_contract_id found");
        //Removiendo el token ID del set de token ID.
        //Removing the token ID from the set.
        by_nft_contract_id.remove(&token_id);

        //Si ahora el set de token ID está vacio, remover el set de token ID.
        //If now the set of token ID is empty, we remove this set.
        if by_nft_contract_id.is_empty() {
            self.by_nft_contract_id.remove(&nft_contract_id);
        } else {
            //Si aun hay token ID, actualizar el set de token ID.
            //If there are still token ID for this contract, we update the map
            self.by_nft_contract_id.insert(&nft_contract_id, &by_nft_contract_id);
        }
        sale
    }
}