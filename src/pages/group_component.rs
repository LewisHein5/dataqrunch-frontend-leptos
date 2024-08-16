use crate::client_lib::Group;
use crate::pages::groups_list_component::GroupsListComponent;
use leptos::{component, view, IntoView};
use leptos::*;

#[component]
pub fn GroupComponent(group: Group) -> impl IntoView{
    let (expanded_signal, set_expanded_signal) = create_signal(false);
    view! {
        <li
        on:click=move |x| {set_expanded_signal.set(!expanded_signal.get());}
        class:group-expanded=move || expanded_signal.get()
        class:group-collapsed=move || !expanded_signal.get()>

        {group.name}

        </li>
        <GroupsListComponent groups=group.subgroups expanded=expanded_signal/>

        <ul>
        {
            let vec = group.datasets;
            vec.into_iter()
            .map(|d| view! {<li class="dataset">" "{d.name}</li>})
            .collect_view()
        }
        </ul>
    }
}