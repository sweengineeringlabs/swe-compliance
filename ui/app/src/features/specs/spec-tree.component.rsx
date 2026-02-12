use rsc_ui::prelude::*;
use crate::features::specs::specs_type::{SpecDirectory, SpecFile};

/// Recursive accordion tree displaying the SpecDirectory hierarchy (FR-1001).
/// Each directory expands to show child directories and files.
/// Files display a kind Badge and are clickable to select.
component SpecTree(
    tree: Signal<Option<SpecDirectory>>,
    on_select: Fn(SpecFile),
) {
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
    }

    render {
        <div class="spec-tree" data-testid="spec-tree">
            @if let Some(ref root) = tree.get() {
                <SpecTreeNode directory={root} on_select={on_select} depth={0} />
            } @else {
                <div class="spec-tree__empty" data-testid="spec-tree-empty">
                    "No spec tree loaded."
                </div>
            }
        </div>
    }
}

/// Recursive node component for a single directory within the spec tree.
component SpecTreeNode(
    directory: &SpecDirectory,
    on_select: Fn(SpecFile),
    depth: usize,
) {
    render {
        <Accordion
            title={format!("{} ({})", directory.name, directory.total_files())}
            data-testid={format!("spec-tree-dir-{}", directory.name)}
        >
            @for child in directory.children.iter() {
                <SpecTreeNode directory={child} on_select={on_select} depth={depth + 1} />
            }
            @for file in directory.files.iter() {
                <div
                    class="spec-tree__file"
                    on:click={let f = file.clone(); move || on_select(f.clone())}
                    data-testid={format!("spec-tree-file-{}", file.name)}
                >
                    <Badge
                        variant={file.kind_variant()}
                        data-testid={format!("spec-tree-badge-{}", file.name)}
                    >
                        {file.kind_label()}
                    </Badge>
                    <span class="spec-tree__file-name" data-testid="spec-tree-file-name">
                        {&file.name}
                    </span>
                </div>
            }
        </Accordion>
    }
}
