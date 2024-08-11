use tonic::Response;
use std::clone;
use leptos::{server_fn::error::NoCustomError,*};
use leptos::leptos_dom::Directive;
use leptos::svg::view;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use crate::client_lib;
use crate::client_lib::*;
use tonic_web_wasm_client::Client;
use super::group_component::*;

#[derive(Serialize, Deserialize, Clone)]
enum GablorpError{
    TonicError(String)
}

impl From<Status> for GablorpError{
    fn from(status: Status) -> Self {
        GablorpError::TonicError(status.to_string())
    }
}
pub async fn check_server_alive() -> ReadSignal<Option<Result<tonic::Response<OperationSuccessModel>, Status>>> {
    let (grr, grrr) = create_signal::<Option<Result<Response<OperationSuccessModel>, Status>>>(None);
    let effect = create_effect(move |_| async move
    {
        let mut client = client_lib::data_qrunch_service_client::DataQrunchServiceClient::new(Client::new("localhost:10000".to_string()));
        let request = client_lib::Empty {};
        let response = client.server_alive(request).await as Result<tonic::Response<OperationSuccessModel>, Status>;
        grrr.set(Some(response));
    });

    grr
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
    let (count, set_count) = create_signal(0);

    let double_count = move || count.get() * 2;
    view! {
        <Suspense fallback = move || view! {<b>"GRRRR"</b>}>
        <ul>
        {
            let vec = groups_resource.get().unwrap_or(vec![]);
            vec.into_iter()
            .map(|g| view! {<GroupComponent group=g/>})
            .collect_view()
        }
        </ul>
        </Suspense>
    }
}
