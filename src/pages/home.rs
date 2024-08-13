use crate::client_lib::*;
use crate::pages::groups_list_component::GroupsListComponent;
use leptos::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use tonic_web_wasm_client::Client;

#[derive(Serialize, Deserialize, Clone)]
enum GablorpError{
    TonicError(String)
}

impl From<Status> for GablorpError{
    fn from(status: Status) -> Self {
        GablorpError::TonicError(status.to_string())
    }
}

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    let groups_resource = create_local_resource(|| (), |_| async move {
        let mut client = data_qrunch_service_client::DataQrunchServiceClient::new(Client::new("http://localhost:10000".to_string()));
        let request = ListGroupsRequest { group: None };
        match client.list_groups(request).await{
            Ok(val) => {val.into_inner().groups}
            Err(_) => {vec![]}
        }
    });

    let (expanded_signal, set_expanded_signal) = create_signal(false);
    view! {
        <Suspense fallback = move || view! {<b>"Loading datasets..."</b>}>
        <div on:click=move |_| {set_expanded_signal.set(true)}>Groups</div>
        <GroupsListComponent
        groups=groups_resource.get().unwrap_or(vec![])
        expanded=expanded_signal
        on:click=move |_| {set_expanded_signal.set(true)}
        />
        </Suspense>
    }
}
