use rsc_compat::prelude::*;
use crate::features::struct_engine::types::CrateNode;

/// Recursive tree showing CrateNode hierarchy (FR-1101).
/// Each crate node expands to show its child crates and module list.
/// Displays kind Badge and module count for each node.
#[component]
pub fn crate_layout(tree: CrateNode) -> View {
    view! {
        style {
            .crate-layout { display: flex; flex-direction: column; gap: var(--space-2); }
            .crate-layout__node { padding: var(--space-2) 0; }
            .crate-layout__header { display: flex; align-items: center; gap: var(--space-2); }
            .crate-layout__name { font-weight: 600; font-size: var(--font-size-sm); }
            .crate-layout__path { font-size: var(--font-size-xs); color: var(--color-text-secondary); font-family: var(--font-mono); }
            .crate-layout__modules { padding-left: var(--space-4); }
            .crate-layout__module { font-size: var(--font-size-xs); color: var(--color-text-secondary); font-family: var(--font-mono); padding: var(--space-1) 0; }
            .crate-layout__children { padding-left: var(--space-4); }
            .crate-layout__empty { font-size: var(--font-size-sm); color: var(--color-text-secondary); padding: var(--space-2); }
            .crate-layout__accordion { border: 1px solid var(--color-border); border-radius: var(--radius-sm); margin-bottom: var(--space-1); }
            .crate-layout__accordion-header { display: flex; align-items: center; padding: var(--space-2) var(--space-3); cursor: pointer; font-weight: 600; font-size: var(--font-size-sm); user-select: none; }
            .crate-layout__accordion-header:hover { background: var(--color-bg-hover); }
            .crate-layout__accordion-body { padding: var(--space-2) var(--space-3); }
        }
        <div class="card" data-testid="crate-layout">
            <div class="crate-layout">
                {crate_node_view(tree, 0)}
            </div>
        </div>
    }
}

/// Recursive crate node renderer (FR-1101).
/// Renders each CrateNode as an expandable item with its modules and children.
#[component]
pub fn crate_node_view(node: CrateNode, depth: usize) -> View {
    let expanded = signal(false);
    let summary = format!("{} ({} modules)", node.name, node.total_modules());
    let kind_variant = match node.kind.as_str() {
        "bin" => "primary",
        "lib" => "info",
        "proc-macro" => "warning",
        _ => "secondary",
    };

    let module_views = node.modules.iter().map(|module| {
        view! {
            <div class="crate-layout__module" data-testid={format!("module-{}", module)}>
                {module}
            </div>
        }
    }).collect::<Vec<_>>();

    let child_views = node.children.iter().map(|child| {
        crate_node_view(child.clone(), depth + 1)
    }).collect::<Vec<_>>();

    let expanded_click = expanded.clone();
    view! {
        <div class="crate-layout__node" data-testid={format!("crate-node-{}", node.name)}>
            <div class="crate-layout__accordion" data-testid={format!("crate-accordion-{}", node.name)}>
                <div
                    class="crate-layout__accordion-header"
                    on:click={let e = expanded_click.clone(); move || { let v = e.get(); e.set(!v); }}
                >
                    {&summary}
                </div>
                {if expanded.get() {
                    view! {
                        <div class="crate-layout__accordion-body">
                            <div class="crate-layout__header">
                                <span class={format!("badge badge--{}", kind_variant)} data-testid={format!("crate-kind-{}", node.name)}>
                                    {&node.kind}
                                </span>
                                <span class="crate-layout__path" data-testid={format!("crate-path-{}", node.name)}>
                                    {&node.path}
                                </span>
                            </div>

                            {if !node.modules.is_empty() {
                                view! {
                                    <div class="crate-layout__modules" data-testid={format!("crate-modules-{}", node.name)}>
                                        {module_views}
                                    </div>
                                }
                            } else {
                                view! {}
                            }}

                            {if !node.children.is_empty() {
                                view! {
                                    <div class="crate-layout__children" data-testid={format!("crate-children-{}", node.name)}>
                                        {child_views}
                                    </div>
                                }
                            } else {
                                view! {}
                            }}

                            {if node.modules.is_empty() && node.children.is_empty() {
                                view! {
                                    <div class="crate-layout__empty" data-testid="crate-empty">
                                        "No modules or child crates."
                                    </div>
                                }
                            } else {
                                view! {}
                            }}
                        </div>
                    }
                } else {
                    view! {}
                }}
            </div>
        </div>
    }
}
