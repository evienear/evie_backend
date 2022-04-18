use crate::*;

#[near_bindgen]
impl Contract {
    //views

    //Retorna el número de ventas que tiene el contrato
    //Returns the number of sales the contract has
    pub fn get_supply_sales(&self) -> U64 {
        //Retorna el lenght como un U64
        //Returns the lenght as a U64
        U64(self.sales.len())
    }

    //Retorna el número de ventas que tiene una cuenta dada (El resultado es un String)
    //Returns the number of sales an account has (The result is a String)
    pub fn get_supply_by_owner_id(
        &self,
        account_id: AccountId,
    ) -> U64 {
        //Obtenemos las ventas del owner ID
        //Get the sales of the owner ID
        let by_owner_id = self.by_owner_id.get(&account_id);
        //Si hay algun set, retornamos el lenght, sino retornamos 0
        //If there is any set, return the lenght, else return 0
        if let Some(by_owner_id) = by_owner_id {
            U64(by_owner_id.len())
        } else {
            U64(0)
        }
    }


    //Retorna una lista paginada de objectos Sale por una cuenta dada (El resultado es un Vector)
    //Returns a paginated list of Sale objects by an account (The result is a Vector)
    pub fn get_sales_by_owner_id(
        &self, 
        account_id: AccountId, 
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Sale> {
        //obtiene el set de token IDs a la venta de la cuenta dada
        //get the set of token IDs for sale for the given account ID
        let by_owner_id = self.by_owner_id.get(&account_id);
        //Si hay un set de ventas, seteamos sales con ese set, y si no, lo seteamos con un Vector vacío
        //If there is a set of sales, set sales with that set, and if not, set sales with an empty Vector
        let sales = if let Some(by_owner_id) = by_owner_id {
            by_owner_id
        } else {
            return vec![];
        };
        //Convertiremos el UnorderedSet a un Vector de Strings
        //Convert the UnorderedSet to a Vector of Strings
        let keys = sales.as_vector();

        //Limite inicial, si hay from_index lo usamos, sino, lo seteamos en 0
        //Initial limit, if there is from_index we use it, otherwise we set it to 0
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iteramos sobre las llaves del vector
        //iterate over the keys of the vector
        // keys.iter().skip(start as usize).take(u128::from(limit.unwrap_or(U128(0))) as usize).map(|key| {
        //     //obtenemos la venta a partir de la llave
        //     //get the sale from the key
        //     self.sales.get(key).unwrap()
        // }).collect()
        keys.iter()
        //Saltamos al indice inicial
        //Skip to the initial index
        .skip(start as usize)
        //Tomamos el limite, si no hay limite, lo seteamos en 0
        //Take the limit, if there is no limit, set it to 0
        .take(limit.unwrap_or(0) as usize)
        //Mapeamos los token IDs en objectos Sale
        //Map the token IDs to Sale objects
        .map(|token_id| self.sales.get(&token_id).unwrap())
        //Regresamos a un Vector
        //Return to a Vector
        .collect()
    }

    //Retornamos numero de ventas por contrato nft (retorna String)
    //Returns the number of sales by contract nft (Return String)
    pub fn get_supply_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
    ) -> U64 {
        //Obtenemos el set de token IDs a la venta del contrato nft dado
        //Get the set of token IDs for sale of the given nft contract ID
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);

        //Si hay un set de ventas, retornamos su lenght, sino, retornamos 0
        //If there is a set of sales, return its lenght, otherwise return 0
        if let Some(by_nft_contract_id) = by_nft_contract_id {
            U64(by_nft_contract_id.len())
        } else {
            U64(0)
        }
    }

    //Retornamos numero de ventas por contrato nft (retorna Vector de sales)
    //Returns the number of sales by contract nft (Return Vector of sales)
    pub fn get_sales_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Sale> {
        //Obtenemos el set de token IDs a la venta del contrato nft dado
        //Get the set of token IDs for sale of the given nft contract ID
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);

        //Si hay un set de ventas, seteamos sales con ese set, y si no, lo seteamos con un Vector vacío
        //If there is a set of sales, set sales with that set, and if not, set sales with an empty Vector
        let sales = if let Some(by_nft_contract_id) = by_nft_contract_id {
            by_nft_contract_id
        } else {
            return vec![];
        };

        //Limite inicial, si hay from_index lo usamos, sino, lo seteamos en 0
        //Initial limit, if there is from_index we use it, otherwise we set it to 0
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //Convertiremos el UnorderedSet a un Vector de Strings
        //Convert the UnorderedSet to a Vector of Strings
        let keys = sales.as_vector();

        //iteramos sobre las llaves del vector
        //iterate over the keys of the vector
        keys.iter()
        //Saltamos al indice inicial
        //Skip to the initial index
        .skip(start as usize)
        //Tomamos el limite, si no hay limite, lo seteamos en 0
        //Take the limit, if there is no limit, set it to 0
        .take(limit.unwrap_or(0) as usize)
        //Mapeamos los token IDs en objectos Sale pasando el ID unico (contract + DELIMETER + token ID)
        //Map the token IDs to Sale objects passing the unique ID (contract + DELIMETER + token ID)
        .map(|token_id| self.sales.get(&format!("{}{}{}", nft_contract_id, DELIMETER, token_id)).unwrap())
        //Regresamos a un Vector
        //Return to a Vector
        .collect()
    }

    //Obtener información de la venta por ID unico (contract + DELIMETER + token ID)
    //Get information of the sale by unique ID (contract + DELIMETER + token ID)
    pub fn get_sale(&self, nft_contract_token: ContractAndTokenId) -> Option<Sale> {
        //Intentamos obtener el objeto sale por el ID unico, es opcional porque puede no existir
        //Try to get the sale object by the unique ID, it is optional because it may not exist
        self.sales.get(&nft_contract_token)
    }
}