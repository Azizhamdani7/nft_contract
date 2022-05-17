// use std::num::NonZeroU128;

use crate::*;
// use near_sdk::AccountId;
const CALLING_ACCOUNT : &str = "nftcross.testnet";


pub trait Meta{
    fn metadata_insert(&mut self, token_type :String, metadata: TokenMetadata);
    fn nft_custom_mint(&mut self, quantity: u128, receiver_id: AccountId); 
}
#[near_bindgen]
impl Meta for Contract{
    

fn metadata_insert(&mut self, token_type :String, metadata: TokenMetadata) {
    
    assert!(env::predecessor_account_id() == self.owner_id || env::predecessor_account_id().to_string() == CALLING_ACCOUNT, "unauthorized");
    self.token_metadata.insert(&token_type, &metadata);
}
#[payable]
fn nft_custom_mint(&mut self, quantity:u128, receiver_id: AccountId){
    let mut x=0;
    while x < quantity {
        let metadata = self.token_metadata.get(&"Token11".to_string()).unwrap();
        let token_id = self.minting_nonce;
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        //specify the token struct that contains the owner ID 
        let token = Token {
            owner_id: receiver_id.clone(),
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id.to_string(), &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id.to_string(), &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
        
        self.minting_nonce += 1;
        x+=1;
    }
}

}
// pub(crate) fn assert_owner(&self) {
//     assert!(self.is_owner(), "Owner's method");
// }