use leptos::{component, view, CollectView, IntoView, ReadSignal};
use crate::client_lib::Group;

#[component]
pub fn GroupComponent(group: Group) -> impl IntoView{
    view! {
        <li>{group.name}</li>
        {
             group.subgroups.into_iter()
            .map(|g| view! {<GroupComponent group=g/>})
            .collect_view()
        }
    }

}