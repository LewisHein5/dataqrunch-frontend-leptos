use crate::client_lib::Group;
use crate::pages::groups_list_component::GroupsListComponent;
use leptos::{component, create_signal, view, CollectView, IntoView};
use leptos::SignalSet;
use leptos::SignalGet;

#[component]
pub fn GroupComponent(group: Group) -> impl IntoView{
    let (expanded_signal, set_expanded_signal) = create_signal(false);
    view! {
        <li on:click=move |x| {set_expanded_signal.set(!expanded_signal.get());}>{group.name}</li>
        <GroupsListComponent groups=group.subgroups expanded=expanded_signal/>

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