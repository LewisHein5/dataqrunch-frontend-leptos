use crate::client_lib::Group;
use crate::pages::groups_list_component::GroupsListComponent;
use leptos::{component, view, CollectView, IntoView};

#[component]
pub fn GroupComponent(group: Group) -> impl IntoView{
    view! {
        <li>{group.name}</li>
        <GroupsListComponent groups=group.subgroups/>

        <ul>
        {
            let vec = group.datasets;
            vec.into_iter()
            .map(|d| view! {<li>{d.name}</li>})
            .collect_view()
        }
        </ul>
    }
}