use crate::*;

//Llamadas a cross contracts
//Call to cross contracts

//Iniciamos llamada cross contrato, tranferimos nft al comprador y retornamos un objeto payout.
//Start cross contract call, transfer nft to the buyer and return a payout object.
#[ext_contract(ext_contract)]
trait ExtContract {
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId, //Comprador = Buyer
        token_id: TokenId, //Token ID a transferir = Token ID to transfer
        approval_id: u64, // ID de aprobación del market = Market Approval ID
        memo: String, //Memo = Memo
        balance: U128, //El precio al que se compro el token = The price to buy the token
        max_len_payout: u32, //Longitud máxima del payout = Max length of payout
    );
}

// #[ext_contract(ext_nft)]
// trait ExtNft {
//     fn nft_tokens_for_owner(
//         &self,
//         account_id: AccountId,
//         from_index: Option<u32>,
//         limit: Option<u64>,
//     );
// }