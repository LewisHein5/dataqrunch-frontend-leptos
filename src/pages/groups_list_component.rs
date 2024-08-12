use crate::client_lib::Group;
use crate::pages::group_component::GroupComponent;
use leptos::{component, view, CollectView, IntoView};

#[component]
pub fn GroupsListComponent(groups: Vec<Group>) -> impl IntoView {
    view! {
        <ul>
        {
             groups.into_iter()
            .map(|g| view! {<GroupComponent group=g/>})
            .collect_view()
        }
        </ul>
    }
}