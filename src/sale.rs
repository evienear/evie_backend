use crate::*;
use near_sdk::promise_result_as_success;

//Estructura que almacena infromación importante acerca de cada sale en el market
//Structure that stores important information about each sale in the market
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
    //owner de la sale
    //owner of the sale
    pub owner_id: AccountId,
    //contrato del market aprobado para transferir el token en nombre del owner
    //market contract's approval ID to transfer the token on behalf of the owner
    pub approval_id: u64,
    //contrato donde se minteó el nft
    //nft contract where the token was minted
    pub nft_contract_id: String,
    //ID del token a la venta
    //actual token ID for sale
    pub token_id: String,
    //precio en yoctoNEAR en el que está listado el token
    //sale price in yoctoNEAR that the token is listed for
    pub sale_conditions: SalePriceInYoctoNear,
}

#[near_bindgen]
impl Contract {
    //remueve una venta del market
    //removes a sale from the market
    #[payable]
    pub fn remove_sale(&mut self, nft_contract_id: AccountId, token_id: String) {
        //Por seguridad verificamos que se haya anezado un solo yocto
        //For security assert one yocto
        assert_one_yocto();
        //Obtenemos el objeto sale como valor de retorno removiendo el sale ID internamente
        //get the sale object as the return value from removing the sale internally
        let sale = self.internal_remove_sale(nft_contract_id.into(), token_id);
        //Obtenemos el predecessor de la llamada y verificamos que sea el owner de la venta
        //get the predecessor of the call and assert that it is the owner of the sale
        let owner_id = env::predecessor_account_id();
        //Si la verificación falla se revierte el remove
        //If the verification fails the remove is reversed
        assert_eq!(owner_id, sale.owner_id, "Only the owner can remove a sale");
    }

    //Actualiza el precio de una venta del market
    //Updates the price of a sale in the market
    #[payable]
    pub fn update_price(&mut self, nft_contract_id: AccountId, token_id: String, price: U128) {
        //Por seguridad verificamos que se haya anezado un solo yocto
        //For security assert one yocto
        assert_one_yocto();
        //Crea el ID unico de la sale (venta) derivado del contrato nft y el token
        //Create the unique sale ID (sale) derived from the nft contract and the token
        let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        //Obtenemos el objeto sale derivado del ID de la sale unico, si no hay token, panic
        //get the sale object derived from the unique sale ID, if no token panic
        let mut sale = self
            .sales
            .get(&contract_and_token_id)
            .expect("No sale found");
        //Obtenemos el predecessor de la llamada y verificamos que sea el owner de la venta
        //get the predecessor of the call and assert that it is the owner of the sale
        let owner_id = env::predecessor_account_id();
        assert_eq!(
            owner_id, sale.owner_id,
            "Only the owner can update the price of a sale"
        );
        //Actualizamos el precio de la venta
        //Update the sale price
        sale.sale_conditions = price;
        //Actualizamos el objeto sale en el market
        //Update the sale object in the market
        self.sales.insert(&contract_and_token_id, &sale);
    }

    //Poner oferta en una venta espacifica, la venta se llevará a cabo siempre que su depósito sea >= al precio de lista
    //Put an offer on a specific sale, the sale will be carried out always that the deposit >= the listed price
    #[payable]
    pub fn offer(&mut self, nft_contract_id: AccountId, token_id: String) {
        //Obtener el deposito adjunto y verificar que sea mayor que cero
        //Get the attached deposit and assert that it is greater than zero
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Deposit must be greater than zero");

        //Convertir nft_contract_id a AccountId
        //Convert nft_contract_id to an AccountId
        let contract_id: AccountId = nft_contract_id.into();
        //Crea el ID unico de la sale (venta) derivado del contrato nft y el token
        //Create the unique sale ID (sale) derived from the nft contract and the token
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
        //Obtenemos el objeto sale derivado del ID de la sale unico, si no hay token, panic
        //get the sale object derived from the unique sale ID, if no token panic
        let sale = self
            .sales
            .get(&contract_and_token_id)
            .expect("No sale found");
        //Obtenemos el predecessor de la llamada y verificamos que no sea el owner de la venta
        //get the predecessor of the call and assert that it is not the owner of the sale
        let buyer_id = env::predecessor_account_id();
        assert_ne!(
            buyer_id, sale.owner_id,
            "I catch you, you can't offer on your own sale."
        );
        //Obtenemos el precio de la venta en u128 (punto 0 convierte de U128 a u128)
        //get the sale price in u128 (dot 0 converts from U128 to u128)
        let price = sale.sale_conditions.0;
        //Verificamos que el deposito sea mayor que el precio de la venta
        //Assert that the deposit is greater than the sale price
        assert!(
            deposit >= price,
            "Deposit must be greater than or equal to the current price: {:?}",
            price
        );

        //Procesamos la compra (Esta función remueve la venta, transfiere dinero y distribuye royalties)
        //Process the purchase (This function removes the sale, transfers money and distributes royalties)
        self.process_purchase(contract_id, token_id, U128(deposit), buyer_id);
    }

    //Función privada que se encarga de procesar la compra
    //Private function that handles the purchase
    #[private]
    pub fn process_purchase(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        price: U128,
        buyer_id: AccountId,
    ) -> Promise {
        //Obtiene el objeto sale removiendo la venta
        //Get the sale object removing the sale
        let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());
        //Iniciamos una llamada a otro contrato (El contrato del nft), esto transferirá tokens
        //al comprador y regresará un payout al market para distribuir los fondos a las cuentas apropiadas
        //Start a call to another contract (the nft contract), this will transfer tokens
        //to the buyer and return a payout to the market to distribute the funds to the appropriate accounts
        ext_contract::ext(
            nft_contract_id, //NFT Contract ID for start the cross contract call = ID del contrato del nft para iniciar la llamada
        ).with_attached_deposit(
            1 //yoctoNEAR attached = YoctoNEAR adjunto
        ).with_static_gas(
            GAS_FOR_NFT_TRANSFER //Gas for NFT transfer = Gas para transferir el nft
        ).nft_transfer_payout(
            buyer_id.clone(),                      //Purchaser = Comprador
            token_id,                              //Token ID = ID del token
            sale.approval_id,                      //Market Approval ID = ID del market aprobado
            "payout from Evie Market".to_string(), //Memo
            price,                                 //Price = Precio de la venta
            MAX_ROYALTIES_ACCOUNTS.into(), //Maximum Accounts for payout = Máximo de cuentas para el payout
            //TODO: Revisar esto, aumentar cantidad a 30, pero aumentará gas necesario
        )
        //Después de que iniciamos el payout, resolvemos la promesa llamando a nuestra propia función resolve_purchase
        //After starting the payout, resolve the promise calling our own function resolve_purchase
        .then(ext_self::ext(
            env::current_account_id(), //Invoked in this contract = Invocado en este contrato
        ).with_static_gas(
            GAS_FOR_ROYALTIES
        ).resolve_purchase(
            buyer_id, //Este parametro es necesario, en caso de error, para devolver al comprador
            price, //The parameters are necesaries in case of error, in order to refund to the buyer
        )
    
    )
    }

    //Función privada que resuelve la promesa, verifica que no haya habido problema, si todo está correcto paga a las cuentas,
    // y si no, devuelve el dinero al comprador
    //Private function that resolves the promise, verifies that there is no problem, if everything is correct, pays the accounts
    // and if not, returns the money to the buyer
    #[private]
    pub fn resolve_purchase(&mut self, buyer_id: AccountId, price: U128) -> U128 {
        //Verifica la información del payout retornada del metodo nft_transfer_payout
        //Verify the information returned from the nft_transfer_payout method
        let payout_option = promise_result_as_success().and_then(|value| {
            //Si el payout es None significa que algo salió mal y devolvemos el dinero al comprador
            //If the payout is None it means that something went wrong and we return the money to the buyer
            near_sdk::serde_json::from_slice::<Payout>(&value)
                //Convertimos el valor a opcional
                //Convert the value to an optional
                .ok()
                //Retornamos None si el valor es None
                //Return None if the value is None
                .and_then(|payout_object| {
                    //Revisamos si length de payout es > 10 (Máximo de cuentas para el payout) o si es vacio, en ese caso retornamos None
                    //Check if the length of payout is > 10 (Maximum accounts for payout) or if it is empty, in that case return None
                    if payout_object.payout.len() > MAX_ROYALTIES_ACCOUNTS as usize
                        || payout_object.payout.is_empty()
                    {
                        let err_msg_max_roy_amount = format!(
                            "The payout has more than {} accounts",
                            MAX_ROYALTIES_ACCOUNTS
                        );
                        env::log_str(&err_msg_max_roy_amount);
                        None
                    } else {
                        //Si es de un largo correcto
                        //If it is a correct length
                        //Mantendremos el monto de cuanto pagar
                        //We keep the amount to pay
                        let mut remainder = price.0;
                        //Iteramos sobre los ids de las cuentas y restemos los pagos
                        //Iterate over the IDs of the accounts and subtract the payments
                        for &value in payout_object.payout.values() {
                            //Buscamos errores de overflow
                            //Find overflow errors
                            remainder = remainder.checked_sub(value.0)?;
                        }
                        //Si el resto es 0 o 1, significa que todo está correcto
                        //If the remainder is 0 or 1, it means that everything is correct
                        if remainder == 0 || remainder == 1 {
                            //Retornamos el objecto payout porque no hay ningún error
                            //Return the payout object because there are no errors
                            Some(payout_object.payout)
                        } else {
                            //Si no, significa que hay un error, retornamos None
                            //If it is not, it means that there is an error, we return None
                            env::log_str("Payout error, remainder is not 0 or 1");
                            None
                        }
                    }
                })
        });
        //Si devolvimos un valor en payout, seteamos la variable
        //If we return a value in payout, we set the variable
        let payout = if let Some(payout_option) = payout_option {
            payout_option
        } else {
            //Si el payout es None devolvemos el dinero al comprador
            //If the payout is None we return the money to the buyer
            Promise::new(buyer_id).transfer(u128::from(price));
            return price;
        };

        //Payouts
        for (receiver_id, amount) in payout {
            Promise::new(receiver_id).transfer(amount.0);
        }
        price
    }
}

//Aquí va la función que se ejecuta cuando el cross contrato es invocado
//Here is the function that is executed when the cross contract is invoked
#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_purchase(&mut self, buyer_id: AccountId, price: U128) -> Promise;
}

// #[ext_contract(ext_nft)]
// trait ExtNft {
//     fn nft_transfer_payout(
//         &mut self,
//         nft_contract_id: AccountId,
//         token_id: U128,
//         receiver_id: AccountId,
//         amount: U128,
//     ) -> Promise;
// }
