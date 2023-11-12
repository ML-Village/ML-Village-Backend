use starknet::{
    accounts::{Account, Call, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, BlockTag, FieldElement, InvokeTransactionResult},
        utils::get_selector_from_name,
    },
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};
use std::env;

pub async fn register_subscription(
    owner_address: FieldElement,
    model_id: FieldElement,
    subscription_end_timestamp: FieldElement,
) -> InvokeTransactionResult {
    dotenvy::dotenv().ok();

    let private_key_hex = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let account_contract_address_hex =
        env::var("ACCOUNT_CONTRACT_ADDRESS").expect("ACCOUNT_CONTRACT_ADDRESS must be set");
    let contract_address_hex = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(&private_key_hex).unwrap(),
    ));
    let address = FieldElement::from_hex_be(&account_contract_address_hex).unwrap();
    let contract_address = FieldElement::from_hex_be(&contract_address_hex).unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let result = account
        .execute(vec![Call {
            to: contract_address,
            selector: get_selector_from_name("register_subscription").unwrap(),
            calldata: vec![owner_address, model_id, subscription_end_timestamp],
        }])
        .send()
        .await
        .unwrap();

    return result;
}

pub async fn register_model(model_id: FieldElement) -> InvokeTransactionResult {
    dotenvy::dotenv().ok();

    let private_key_hex = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let account_contract_address_hex =
        env::var("ACCOUNT_CONTRACT_ADDRESS").expect("ACCOUNT_CONTRACT_ADDRESS must be set");
    let contract_address_hex = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(&private_key_hex).unwrap(),
    ));
    let address = FieldElement::from_hex_be(&account_contract_address_hex).unwrap();
    let contract_address = FieldElement::from_hex_be(&contract_address_hex).unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let result = account
        .execute(vec![Call {
            to: contract_address,
            selector: get_selector_from_name("register_model").unwrap(),
            calldata: vec![model_id],
        }])
        .send()
        .await
        .unwrap();

    return result;
}
