// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 02_get_address_balance --release
//! In this example we will get the outputs of an address that have no additional unlock conditions and sum the amounts
//! and native tokens.

use iota_client::{
    block::output::{NativeTokensBuilder, Output},
    node_api::indexer::query_parameters::QueryParameter,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {

    dotenv::dotenv().ok();

    // Se obtiene el URL del nodo desde una variable de entorno
    let node_url = std::env::var("NODE_URL").unwrap();

    // Se crea una instancia del cliente (mediante un builder).
    let client = Client::builder()
        .with_node(&node_url)?
        .finish()?;
    println!("Cliente:\n{client:#?}\n");

    // Se obtiene la string con 24 palabras (el nmotecnico) desde una variable de entorno.
    let mnemonic = std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap();

    // Se crea una instancia de SecretManager 'MnemonicSecretManager.
    let secret_manager_mnemonic =
        MnemonicSecretManager::try_from_mnemonic(&mnemonic)?;

    // Se crea la variante de 'iota_client::secret::SecretManager' de tipo 'iota_client::secret::mnemonic::Mnemonic'
    let secret_manager = SecretManager::Mnemonic(secret_manager_mnemonic);

    // Mediante un 'iota_client::api::address::GetAddressesBuilder' al cual se le proporciona el
    // 'iota_client::secret::SecretManager' se genera la primera direccion (solo se pide la primera)
    let addresses = client
        .get_addresses(&secret_manager)
//        .with_account_index(0)
//        .with_range(0..1)
        .finish()
        .await?;
    println!("Direcciones:\n{addresses:#?}\n");

    // Se obtienen los identificadores de las salida que pueden ser controladas por la direccion sin
    // restricciones de bloqueo.
    //
    // Como hay que transferir la propiedad de la direccion por legibilidad se clona la posicion del array
    // en su propia variable.
    let address = addresses[0].clone();
    let query_parameters = vec![
        QueryParameter::Address(address),
        QueryParameter::HasExpiration(false),
        QueryParameter::HasTimelock(false),
        QueryParameter::HasStorageDepositReturn(false),
    ];
    let output_ids_response = client
        .basic_output_ids(query_parameters)  
        .await?;
    println!("Ids de las salida de la primera direccion:\n{output_ids_response:#?}\n");

    // Mediante los ids se extrae las salidas.
    let outputs_responses = client.get_outputs(output_ids_response.items).await?;

    // Obtiene el suministro de tokens del nodo al que nos estamos conectando.
    let token_supply = client.get_token_supply().await?;

    // Calcula el importe total de los tockens nativos.
    let mut total_amount = 0;
    let mut total_native_tokens = NativeTokensBuilder::new();
    for output_response in outputs_responses {
        let output = Output::try_from_dto(&output_response.output, token_supply)?;

        if let Some(native_tokens) = output.native_tokens() {
            total_native_tokens.add_native_tokens(native_tokens.clone())?;
        }
        total_amount += output.amount();
    }

    // Se muestra el resultado.
    println!(
        "Outputs controlled by {} have: {:?}i and native tokens: {:?}",
        addresses[0],
        total_amount,
        total_native_tokens.finish_vec()?
    );

    Ok(())

}
