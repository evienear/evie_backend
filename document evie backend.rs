//Iniciando el contrato
//Init the contract

//Crea las colecciones
//Create the collections

new(owner_id: AccountId) 

//Recibe el contrato de minteo del nft y el id del token y llama a internal_remove_sale
//Receive the contract of the NFT and the token id and calls internal_remove_sale
remove_sale(nft_contract_id: AccountId, token_id: String)

//Recibe el contrato de minteo del nft, el id del token y un nuevo precio (Y lo actualiza)
//Receive the contract of the NFT, the token id and a new price (and update)
update_price(nft_contract_id: AccountId, token_id: String, price: U128)

//Recibe el contrato de minteo del nft y el id del token y procede a la compra llamando a process_purchase
//Receive the contract of the NFT and the token id and make the buy calling to process_purchase
offer(nft_contract_id: AccountId, token_id: String)














//Al momento de listar un NFT deben llamarse 2 funciones, la primera es storage_deposit Y SE LE ANEXA UN DEPOSITO
//At the moment of listing an NFT we should call 2 functions, the first is storage_deposit A WE ATTACH A DEPOSIT
//Esta es al contrato actual del marketplace
//This is to the actual contract - marketplace
storage_deposit(account_id: Option<AccountId>) //El AccountID es opcional, si no se manda toma el caller - optional, if not passed take the caller

//Y la segunda es:
//And the second is:
//Para listar un NFT debe llamarse a la función nft_approve del contrato original en el que se minteó el nft
//In order to list an NFT there should be called the funcion nft_approve in the orginal nft contract
nft_approve(token_id: TokenId/*String*/, account_id: AccountId, msg: Option<String> /*Required*/)

//internal_remove_sale(nft_contract_id: AccountId, token_id: String)