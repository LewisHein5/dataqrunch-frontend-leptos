use crate::client_lib::Group;
use crate::pages::groups_list_component::GroupsListComponent;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::{component, create_signal, view, CollectView, IntoView, Show};

#[component]
pub fn GroupComponent(group: Group) -> impl IntoView{
    let (expanded_signal, set_expanded_signal) = create_signal(false);
    let name = group.name.clone(); //TODO: this is really ugly, there is probably a better way
    let name2 = group.name.clone();
    view! {
        <Show when=move || expanded_signal.get()>
            <li on:click=move |x| {set_expanded_signal.set(!expanded_signal.get());} class="group-expanded">" "{name.clone()}</li>
        </Show>
        <Show when=move || !expanded_signal.get()>
            <li on:click=move |x| {set_expanded_signal.set(!expanded_signal.get());} class="group-collapsed">" "{name2.clone()}</li>
        </Show>
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