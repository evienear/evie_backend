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

        //Verificar si hay storage suficiente
        //Check if there is enough storage

        //Obtenemos el storage (Recordemos que .o convierte de U128 a u128)
        //Get the storage (Remember that .o converts from U128 to u128)
        let storage_amount: u128 = self.storage_minimum_balance().0;
        //Obtenemos el storage pagado por el owner
        //Get the storage paid by the owner
        let owner_paid_storage: u128 = self.storage_deposits.get(&signer_id).unwrap_or(0);
        //Obtener el storage requerido (storage por (numero de ventas mas 1))
        //Get the storage required (storage by (number of sales plus 1))
        let signer_storage_required = (self.get_supply_by_owner_id(signer_id).0+1) as u128 * storage_amount;

        //Comprobamos que el storage pagado por el owner sea >= suficiente
        //Check that the owner paid storage is >= sufficient
        assert!(
            owner_paid_storage >= signer_storage_required,
            "The owner paid storage is not sufficient: {}, for {} sales at {} rate of per sale",
            owner_paid_storage, signer_storage_required / STORAGE_PER_SALE, STORAGE_PER_SALE
        );

        //Si todo fue correcto, añadimos la venta
        //If everything was correct, add the sale
        let SaleArgs { sale_conditions } =
            //Las condiciones vienen del msg, el market asume que el usuario ha pasado msg correcto, si no panic
            //The conditions come from the msg, the market assumes that the user has passed a correct msg, if not panic
            near_sdk::serde_json::from_str(&msg).expect("Failed to deserialize msg, not valid");
        
        //Creamos el ID unico de la venta (contract + DELIMETER + token_id)
        //Create the unique ID of the sale (contract + DELIMETER + token_id)
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);

        //Insertamos el valor en el mapa de ventas, la llave es el ID unico, value es el objeto Sale
        //Insert the value in the map of sales, the key is the unique ID, value is the object Sale
        self.sales.insert(
            &contract_and_token_id,
            &Sale {
                owner_id: owner_id.clone(), //Owner = Dueño
                approval_id, //Approval ID = ID de aprobación
                nft_contract_id: nft_contract_id.to_string(), //NFT Contract ID = ID del contrato NFT
                token_id: token_id.clone(),//Token ID = ID del token
                sale_conditions, //Sale Conditions = Condiciones de la venta
            },
        );

        //Funciones extras para view
        //Extra functions para view

        //Obtener las ventas por el owner ID, si no hay creamos un set vacio
        //Get the sales by the owner ID, if there is no create an empty set
        let mut by_owner_id = self.by_owner_id.get(&owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByOwnerIdInner {
                    //obtenemos un prefijo unico para la colección de un hash del owner
                    //get a unique prefix for the collection of a hash of the owner
                    account_id_hash: hash_account_id(&owner_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //Insertar el ID del token en el set
        //Insert the token ID in the set
        //by_owner_id.insert(&token_id);
        by_owner_id.insert(&contract_and_token_id);
        //insertamos el set de vuelta a la coleccion por el contract ID de NFT
        //insert the set back to the collection by the NFT contract ID
        self.by_owner_id.insert(&nft_contract_id, &by_owner_id);

        //Obtener el token ID del contrato nft dado, si no hay creamos un set vacio
        //get the token IDs for the given nft contract ID. If there are none, we create a new empty set
        let mut by_nft_contract_id = self
        .by_nft_contract_id
        .get(&nft_contract_id)
        .unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByNFTContractIdInner {
                    //Obtenemos un prefijo unico para la coleccion hashing el owner
                    //Get a unique prefix for the collection hashing the owner
                    account_id_hash: hash_account_id(&nft_contract_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //Insertar el ID del token en el set
        //Insert the token ID in the set
        by_nft_contract_id.insert(&token_id);
        //Insertamos el set de vuelta a la coleccion por el contract ID de NFT
        //Insert the set back to the collection by the NFT contract ID
        self.by_nft_contract_id.insert(&nft_contract_id, &by_nft_contract_id);
    }
    
}