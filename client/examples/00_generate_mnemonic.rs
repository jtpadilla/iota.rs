// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example generate_mnemonic --release

// Se define en el BIP 39 (Bitcoin Improvement Proposal) https://en.bitcoin.it/wiki/BIP_0039
// como se genera la frase mnemotectnica de 24 palabras la cual se podra convertir
// en una semilla binaria (para despues obtener el wallet).

// En el ejemplo se utiliza una funcion asociada a la estructura 'iota_client::Client'
// que en realidad invoca a la function 'iota_client::utils::generate_mnemonic()'.


use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //let mnemonic = Client::generate_mnemonic()?;
    let mnemonic = iota_client::utils::generate_mnemonic()?;

    println!("Mnemonic: {mnemonic}");

    Ok(())
}
