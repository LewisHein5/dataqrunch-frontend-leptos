use leptos::{Show, SignalGet};
use crate::client_lib::Group;
use crate::pages::group_component::GroupComponent;
use leptos::{component, view, CollectView, IntoView, ReadSignal};

#[component]
pub fn GroupsListComponent(groups:Vec<Group>, expanded: ReadSignal<bool>) -> impl IntoView {
    let n_groups = groups.len();
    view! {
        <ul class="groups-list">
        <Show when=move || {expanded.get()}
        fallback=move || view!{ }
        >
        {
             groups.clone().into_iter()
            .map(|g| view! {<GroupComponent group=g/>})
            .collect_view()
        }
        </Show>
        </ul>
    }
}