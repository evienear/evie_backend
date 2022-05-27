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

//Añade un deposito para storage a la cuenta que se le pase o al caller si no se le pasa una
//Attach a deposit for storage to a passed account ID, and if no Account ID it takes the caller
storage_deposit(account_id: Option<AccountId>) //El AccountID es opcional, si no se manda toma el caller - optional, if not passed take the caller

//Permitimos a los usuarios retirar el storage depositado en exceso
//Allow users to withdraw the storage deposit excess
storage_withdraw() //Sin parametros extra, solo el que llama - No extra parameter, takes the caller

//Retorna el balance minimo requerido para un NFT listado
//Returns the minimum balance required for an NFT listed
storage_minimum_balance()

__________________________________________________________________________________________________________________________________________________

//Funciones que listan NFTs a la venta en el Marketplace
//Functions that list NFTs on sale on the Marketplace

//Retorna el número de ventas que tiene el contrato (Devuelve U64 (String))
//Returns the number of sales the contract has (Return U64 (String))
get_supply_sales()

//Retorna el número de ventas que tiene una cuenta dada (El resultado es un String)
//Returns the number of sales an account has (The result is a String)
get_supply_by_owner_id(account_id: AccountId)

//Retorna una lista paginada de objectos Sale por una cuenta dada (El resultado es un Vector)
//Returns a paginated list of Sale objects by an account (The result is a Vector)

todo!()//TODO - JEPH: Revisar que funcionen correctamente los parametros opcionales
get_sales_by_owner_id(account_id: AccountId, from_index: Option<U128>, limit: Option<u64>,)

//Retornamos numero de ventas por contrato nft (retorna String)
//Returns the number of sales by contract nft (Return String)
get_supply_by_nft_contract_id(nft_contract_id: AccountId)

//Retornamos numero de ventas por contrato nft (retorna Vector de sales)
//Returns the number of sales by contract nft (Return Vector of sales)
get_sales_by_nft_contract_id(nft_contract_id: AccountId, from_index: Option<U128>, limit: Option<u64>,)

//Obtener información de la venta por ID unico (contract + DELIMETER + token ID)
//Get information of the sale by unique ID (contract + DELIMETER + token ID)
todo!()//TODO - JEPH: Corregir Typo
get_sale(nft_contract_token: ContractAndTokenId)
__________________________________________________________________________________________________________________________________________________

//Listando NFT
//Listing NFT

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

//Nada
//internal_remove_sale(nft_contract_id: AccountId, token_id: String)
