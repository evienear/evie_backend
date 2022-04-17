//#![deny(warnings)]  // deny warnings in the code
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, EpochHeight,
    Gas, PanicOnDefault, Promise, CryptoHash, BorshStorageKey,
};
use std::collections::HashMap;

 use crate::external::*;
 use crate::internal::*;
 use crate::sale::*;

use near_sdk::env::STORAGE_PRICE_PER_BYTE;

 mod external;
 mod internal;
 mod nft_callbacks;
 mod sale;
 mod sale_views;

//Constantes de gas para las llamadas
//Gas consts for the calls
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

#[derive(BorshDeserialize, BorshSerialize)]
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
}