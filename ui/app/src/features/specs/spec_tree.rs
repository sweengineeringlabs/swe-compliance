use std::rc::Rc;
use rsc_compat::prelude::*;
use crate::features::specs::types::{SpecDirectory, SpecFile};

/// Recursive tree displaying the SpecDirectory hierarchy (FR-1001).
/// Each directory expands to show child directories and files.
/// Files display a kind Badge and are clickable to select.
#[component]
pub fn spec_tree(
    tree: Signal<Option<SpecDirectory>>,
    on_select: Option<Rc<dyn Fn(SpecFile)>>,
) -> View {
    let tree_content = if let Some(ref root) = tree.get() {
        spec_tree_node(root.clone(), on_select.clone(), 0)
    } else {
        view! {
            <div class="spec-tree__empty" data-testid="spec-tree-empty">
                "No spec tree loaded."
            </div>
        }
    };

    view! {
        style {
            .spec-tree {
                display: flex;
                flex-direction: column;
                gap: var(--space-1);
                overflow-y: auto;
            }

            .spec-tree__empty {
                padding: var(--space-4);
                color: var(--color-text-muted);
                font-size: var(--font-size-sm);
            }

            .spec-tree__file {
                display: flex;
                align-items: center;
                gap: var(--space-2);
                padding: var(--space-2) var(--space-3);
                cursor: pointer;
                border-radius: var(--radius-sm);
                font-size: var(--font-size-sm);
            }

            .spec-tree__file:hover {
                background: var(--color-bg-hover);
            }

            .spec-tree__file-name {
                flex: 1;
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
            }

            .spec-tree__dir-count {
                font-size: var(--font-size-xs);
                color: var(--color-text-muted);
                margin-left: auto;
            }

            .spec-tree__accordion {
                border: 1px solid var(--color-border);
                border-radius: var(--radius-sm);
                margin-bottom: var(--space-1);
            }

            .spec-tree__accordion-header {
                display: flex;
                align-items: center;
                padding: var(--space-2) var(--space-3);
                cursor: pointer;
                font-weight: 600;
                font-size: var(--font-size-sm);
                user-select: none;
            }

            .spec-tree__accordion-header:hover {
                background: var(--color-bg-hover);
            }

            .spec-tree__accordion-body {
                padding-left: var(--space-3);
            }
        }
        <div class="spec-tree" data-testid="spec-tree">
            {tree_content}
        </div>
    }
}

/// Recursive node component for a single directory within the spec tree.
#[component]
pub fn spec_tree_node(
    directory: SpecDirectory,
    on_select: Option<Rc<dyn Fn(SpecFile)>>,
    depth: usize,
) -> View {
    let expanded = signal(false);
    let title = format!("{} ({})", directory.name, directory.total_files());

    let children_views = directory.children.iter().map(|child| {
        let on_select = on_select.clone();
        spec_tree_node(child.clone(), on_select, depth + 1)
    }).collect::<Vec<_>>();

    let file_views = directory.files.iter().map(|file| {
        let f = file.clone();
        let on_select = on_select.clone();
        view! {
            <div
                class="spec-tree__file"
                on:click={let f = f.clone(); let on_select = on_select.clone(); move || { if let Some(ref cb) = on_select { cb(f.clone()) } }}
                data-testid={format!("spec-tree-file-{}", file.name)}
            >
                <span class={format!("badge badge--{}", file.kind_variant())} data-testid={format!("spec-tree-badge-{}", file.name)}>
                    {file.kind_label()}
                </span>
                <span class="spec-tree__file-name" data-testid="spec-tree-file-name">
                    {&file.name}
                </span>
            </div>
        }
    }).collect::<Vec<_>>();

    let expanded_click = expanded.clone();
    view! {
        <div class="spec-tree__accordion" data-testid={format!("spec-tree-dir-{}", directory.name)}>
            <div
                class="spec-tree__accordion-header"
                on:click={let e = expanded_click.clone(); move || { let v = e.get(); e.set(!v); }}
            >
                {&title}
            </div>
            {if expanded.get() {
                view! {
                    <div class="spec-tree__accordion-body">
                        {children_views}
                        {file_views}
                    </div>
                }
            } else {
                view! {}
            }}
        </div>
    }
}
