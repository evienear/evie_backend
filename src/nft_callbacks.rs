use crate::*;

//Callbacks aprobados de los Contratos NFT
//NFT callbacks approved by the NFT contracts

//Estructura para mantener registro de las condiciones de la venta
//Structure to keep track of the conditions of the sale
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    pub sale_conditions: SalePriceInYoctoNear,
}

//Trait para el callback del Contrato NFT
//Trait for the NFT contract callback
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

//Implementación del trait para el callback del Contrato NFT
//Implementation of the trait for the NFT contract callback
#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    //Aquí añadimos la venta porque sabemos que el nft owner solamente puede llamar nft_approve
    //Here we add the sale because we know that the nft owner only can call nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) {
        //Obtenemos el ID del contrato que es el predecessor
        //Get the ID of the contract that is the predecessor
        let nft_contract_id = env::predecessor_account_id();
        //Obtenemos el signer que es la persona que inicia la transacción
        //Get the signer that is the person who initiates the transaction
        let signer_id = env::signer_account_id();

        //Comprobamos que el signer no sea el predecessor
        //Check that the signer is not the predecessor
        //Esto es una llamada cross-contract
        //This is a cross-contract call
        assert_ne!(
            nft_contract_id,
            signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        //Nos aseguramos de que el signer sea el owner del nft
        //We make sure that the signer is the owner of the nft
        assert_eq!(
            owner_id,
            signer_id,
            "nft_on_approve should only be called by the owner of the nft"
        );
    }
    
}