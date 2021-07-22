// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! See also https://www.w3.org/TR/did-core/#did-resolution and https://www.w3.org/TR/did-core/#did-url-dereferencing
//!
//! cargo run --example resolution

mod create_did;

use identity::core::SerdeInto;
use identity::did::resolution;
use identity::did::resolution::Dereference;
use identity::did::resolution::InputMetadata;
use identity::did::resolution::Resolution;
use identity::did::resolution::Resource;
use identity::did::resolution::SecondaryResource;
use identity::iota::ClientMap;
use identity::iota::IotaDID;
use identity::iota::Receipt;
use identity::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (document, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // ===========================================================================
  // DID Resolution
  // ===========================================================================

  let doc_did: &IotaDID = document.id();
  let did_url: &str = doc_did.as_str();

  // Retrieve the published DID Document from the Tangle.
  let input: InputMetadata = Default::default();
  let output: Resolution = resolution::resolve(did_url, input, &client).await?;

  println!("Resolution > {:#?}", output);

  // The resolved Document should be the same as what we published.
  assert_eq!(output.document.unwrap(), document.serde_into().unwrap());

  // ===========================================================================
  // DID Dereferencing
  // ===========================================================================

  let resource_did: IotaDID = doc_did.join("#authentication")?;
  let resource_url: &str = resource_did.as_str();

  // Retrieve a subset of the published DID Document properties.
  let input: InputMetadata = Default::default();
  let output: Dereference = resolution::dereference(resource_url, input, &client).await?;

  println!("Dereference > {:#?}", output);

  // The resolved resource should be the DID Document authentication method.
  match output.content.unwrap() {
    Resource::Secondary(SecondaryResource::VerificationKey(method)) => {
      assert_eq!(method, **document.authentication());
    }
    resource => {
      panic!("Invalid Resource Dereference > {:#?}", resource);
    }
  }

  Ok(())
}
