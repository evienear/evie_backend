use crate::*;

#[ext_contract(ext_paras)]
trait ExtParas {
    fn nft_total_supply(&self) -> U128;
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token>;
    fn nft_approve(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        msg: Option<String>
    );
}

#[ext_contract(ext_nft_dos)]
trait ExtNftDos {
    fn on_nft_total_supply(&mut self) -> U128;
    fn on_nft_tokens_for_owner(&mut self) -> Vec<Token>;
}