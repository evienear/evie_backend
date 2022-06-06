//#![deny(warnings)] // deny warnings in the code
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault, Promise,
};
use std::collections::HashMap;

use crate::cross_contract_calls::*;
use crate::external::*;
use crate::internal::*;
use crate::sale::*;

use near_sdk::env::STORAGE_PRICE_PER_BYTE;

mod cross_contract_calls;
mod external;
mod internal;
mod nft_callbacks;
mod sale;
mod sale_views;

//Constantes de gas para las llamadas
//Gas consts for the calls
const GAS_FOR_CROSS_CONTRACT_CALL: Gas = Gas(5_000_000_000_000);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);
const GAS_FOR_ROYALTIES: Gas = Gas(115_000_000_000_000);
const NO_DEPOSIT: Balance = 0;
const MAX_ROYALTIES_ACCOUNTS: u8 = 10;

const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE;

//TODO: Change Typo DELIMETER to DELIMITER
static DELIMETER: &str = ".";

//Tipos personalizados para facilidad de lectura
//Custom types for readability
pub type SalePriceInYoctoNear = U128;
pub type TokenId = String;
pub type FungibleTokenId = AccountId;
pub type ContractAndTokenId = String;

//definimos el tipo Payout del contrato NFT que usaremos como estandar para las regalías
//defines the payout type we'll be parsing from the NFT contract as a part of the royalty standard.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //dueño del contrato
    //owner of the contract
    pub owner_id: AccountId,
    //Para mantener una lista de cada venta mapearemos el ContractAndTokenId a una venta
    //In order to maintain a list of every sale, well map the ContractAndTokenId to the sale
    //Este está hecho de `contract ID + DELIMETER + token ID`
    //It is made up of the `contract ID + DELIMETER + token ID`
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,
    //Lista de todas las SaleIds creadas para cada Account ID
    //List of all the SaleIds that have been created for each Account ID
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,
    //Lista de todas las SaleIds creadas para cada el Contrato
    //List of all the SaleIds that have been created for Contract
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,
    //Mantemenos seguimiento del storage pagado
    //Maintain track of the storage paid
    pub storage_deposits: LookupMap<AccountId, Balance>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByOwnerIdInner { account_id_hash: CryptoHash },
    ByNFTContractId,
    ByNFTContractIdInner { account_id_hash: CryptoHash },
    ByNFTTokenType,
    ByNFTTokenTypeInner { token_type_hash: CryptoHash },
    FTTokenIds,
    StorageDeposits,
}

#[near_bindgen]
impl Contract {
    //Función inicial, solo se ejecuta una vez
    //Init function, just call it once
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            owner_id,
            //Para evitar colisión de datos
            //Avoiding data collisions
            sales: UnorderedMap::new(StorageKey::Sales),
            by_owner_id: LookupMap::new(StorageKey::ByOwnerId),
            by_nft_contract_id: LookupMap::new(StorageKey::ByNFTContractId),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
        };
        this
    }

    //Damos permiso a los usuarios par adepositar storage, para cubrir costos del contrato
    //Allow users to deposit storage, to cover the contract's costs
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        //Obtenemos el account ID al que le agregaremos el storage
        //Get the account ID to which we'll add the storage
        let storage_account_id = account_id
            //convertimos el valid account ID en un account ID
            //convert the valid account ID into an account ID
            .map(|a| a.into())
            //Si no especificamos un account ID, usaremos el caller
            //If we don't specify an account ID, we'll use the caller
            .unwrap_or_else(|| env::predecessor_account_id());
        //TODO: REVISAR LINEA DE ARRIBA ||

        //Obtenemos el storage depositado en esta transaccion
        //Get the storage deposit made in this transaction
        let deposit = env::attached_deposit();

        //Revisamos si el deposito es >= el mino storage para una venta
        //Check if the deposit is >= the minimum storage for a sale
        assert!(
            deposit >= STORAGE_PER_SALE,
            "Deposit must be at least {}",
            STORAGE_PER_SALE
        );

        //Obtenemos el balance de la cuenta (Si la cuenta no está mapeada default 0)
        //Get the balance of the account (if the account is not mapped default 0)
        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        //Añadimos el deposito a su balance
        //Add the deposit to its balance
        balance += deposit;
        //Insertamos el balance de vuelta en el mapa para ese account ID
        //Insert the balance back into the map for that account ID
        self.storage_deposits.insert(&storage_account_id, &balance);
    }

    //Permitimos a los usuarios retirar el storage depositado en exceso
    //Allow users to withdraw the storage deposit excess
    pub fn storage_withdraw(&mut self) {
        //Por seguridad verificamos que se anexe 1 yoctoNEAR
        //For security, we verify that 1 yoctoNEAR is attached
        assert_one_yocto();

        //La cuenta para withdraw es siempre el caller
        //The account for withdraw is always the caller
        let owner_id = env::predecessor_account_id();
        //Obtenemos el balance de la cuenta revisando el mapa, si no está mapeado default 0
        //Get the balance of the account checking the map, if not mapped default 0
        let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);

        //Verificamos cuantas ventas tiene el usuario en este momento
        //Check how many sales the user has in this moment
        let sales = self.by_owner_id.get(&owner_id);
        //Obtenemos el length de las ventas
        //Get the length of the sales
        let len = sales.map(|s| s.len()).unwrap_or_default();
        //¿Cuanto NEAR está siendo usado por esas ventas?
        //How much NEAR is being used by those sales?
        let diff = u128::from(len) * STORAGE_PER_SALE;
        //Obtenemos el exceso de storage depositado
        //Get the excess storage deposit
        amount -= diff;

        //Si el exceso es mayor a 0, entonces retiramos el exceso
        //If the excess is greater than 0, we withdraw the excess
        if amount > 0 {
            //Retiramos el exceso
            //Withdraw the excess
            Promise::new(owner_id.clone()).transfer(amount);
        }
        //Despues debemos insertar el storage en uso en el mapa
        //After we insert the storage in use in the map
        if diff > 0 {
            self.storage_deposits.insert(&owner_id, &diff);
        }
    }

    //Views

    //Retornamos el minimo storage para una venta
    //Return the minimum storage for a sale
    pub fn storage_minimum_balance(&self) -> U128 {
        U128(STORAGE_PER_SALE)
    }

    //Retornamos el storage pagado por una cuenta
    //Return the storage paid by an account
    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.storage_deposits.get(&account_id).unwrap_or(0))
    }

    //Llamadas a los contratos externos
    //Cross Contract Calls

    // //Obtener la cantidad total de NFTs minteados del contrato
    // //Get the total amount of NFTs minted by the contract
    pub fn nft_total_supply_marketplace(&self, marketplace_contract_id: AccountId) -> Promise {
        ext_paras::nft_total_supply(
            marketplace_contract_id,
            //AccountId::new_unchecked(String::from("paras-token-v2.testnet")), //contract
            0,                           //yoctoNEAR to attach,
            GAS_FOR_CROSS_CONTRACT_CALL, //gas to attach,
        )
        .then(ext_nft_dos::on_nft_total_supply(
            env::current_account_id(),
            0,                           //yoctoNEAR to attach,
            GAS_FOR_CROSS_CONTRACT_CALL, //gas to attach,
        ))
    }

    // //Obtener los tokens de un usuario
    // //Get the tokens of a user

    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
        marketplace_contract_id: AccountId,
    ) -> Promise {
        ext_paras::nft_tokens_for_owner(
            account_id,
            from_index,
            // Some(from_index.unwrap_or(U128(0))),
            limit,
            marketplace_contract_id,
            0,
            GAS_FOR_CROSS_CONTRACT_CALL,
        )
        .then(ext_nft_dos::on_nft_tokens_for_owner(
            env::current_account_id(),
            0,
            GAS_FOR_CROSS_CONTRACT_CALL,
        ))
    }

    #[payable]
    pub fn nft_approve(
        &mut self,
        token_id: TokenId,
        //account_id: AccountId,
        msg: Option<String>,
        marketplace_contract_id: AccountId,
    ) {
        ext_paras::nft_approve(
            token_id,
            env::current_account_id(),
            msg,
            marketplace_contract_id,
            1,
            GAS_FOR_CROSS_CONTRACT_CALL,
        );
    }

    //Callback de Funciones externas
    //Callback de External Functions

    //Obtener la cantidad total de NFTs minteados del contrato
    //Get the total amount of NFTs minted by the contract
    #[private]
    pub fn on_nft_total_supply(&mut self) -> U128 {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("on_nft_total_supply: result is None");
        }
        let ret = near_sdk::serde_json::from_slice::<U128>(&result.unwrap()).expect("U128");
        return ret;
    }

    //Obtener los tokens de un usuario
    //Get the tokens of a user
    #[private]
    pub fn on_nft_tokens_for_owner(&mut self) -> Vec<Token> {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("on_nft_tokens_for_owner: result is None");
        }
        let ret =
            near_sdk::serde_json::from_slice::<Vec<Token>>(&result.unwrap()).expect("Vec<Token>");
        return ret;
    }
}

//No se puede usar esta funcion
// pub fn market_example_multiple(
//     &self,
//     marketplaces: Vec<AccountId>
// ) {
//     for marketplace in marketplaces {
//         self.nft_total_supply_marketplace(marketplace);
//     }
// }
