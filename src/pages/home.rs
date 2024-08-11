use std::clone;
use leptos::{server_fn::error::NoCustomError, *};
use leptos_router::*;
use crate::client_lib;
use crate::client_lib::*;
use tonic_web_wasm_client::Client;


pub async fn check_server_alive() -> client_lib::OperationSuccessModel {
    let mut client = client_lib::data_qrunch_service_client::DataQrunchServiceClient::new(Client::new("localhost:8080".to_string()));
    let request = client_lib::Empty{};
    let response = client.server_alive(request).await.unwrap();

    response.into_inner()
}

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count.get() * 2;
    view! {
        <Await
        future=|| check_server_alive()
        let:data
        >
        <p>{data.success} "grr"</p>
        </Await>
    }
}

#[server]
pub async fn update_count() -> Result<(), ServerFnError> {
    println!("Upated count");

    let store = spin_sdk::key_value::Store::open_default()?;

    let count: u64 = store
        .get_json("dataqrunch_frontend_count")
        .map_err(|e| ServerFnError::new(e))?
        .unwrap_or_default();

    let updated_count = count + 1;

    store
        .set_json("dataqrunch_frontend_count", &updated_count)
        .map_err(|e| ServerFnError::new(e))?;
    Ok(())
}

#[server]
pub async fn get_count() -> Result<u64, ServerFnError> {
    let store = spin_sdk::key_value::Store::open_default()?;

    let stored_count: u64 = store
        .get_json("dataqrunch_frontend_count")
        .map_err(|e| ServerFnError::new(e))?
        .ok_or_else(|| {
            ServerFnError::<NoCustomError>::ServerError("Failed to get count".to_string())
        })?;

    println!("Got stored {stored_count}");

    Ok(stored_count)
}
