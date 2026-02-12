use rsc_ui::prelude::*;
use crate::features::struct_engine::struct_engine_type::CrateNode;

/// Recursive accordion tree showing CrateNode hierarchy (FR-1101).
/// Each crate node expands to show its child crates and module list.
/// Displays kind Badge and module count for each node.
component CrateLayout(tree: CrateNode) {
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
    }

    render {
        <Card data-testid="crate-layout">
            <div class="crate-layout">
                <CrateNodeView node={tree} depth={0} />
            </div>
        </Card>
    }
}

/// Recursive crate node renderer (FR-1101).
/// Renders each CrateNode as an Accordion item with its modules and children.
component CrateNodeView(node: CrateNode, depth: usize) {
    let summary = format!("{} ({} modules)", node.name, node.total_modules());
    let kind_variant = match node.kind.as_str() {
        "bin" => "primary",
        "lib" => "info",
        "proc-macro" => "warning",
        _ => "secondary",
    };

    render {
        <div class="crate-layout__node" data-testid={format!("crate-node-{}", node.name)}>
            <Accordion title={summary} data-testid={format!("crate-accordion-{}", node.name)}>
                <div class="crate-layout__header">
                    <Badge variant={kind_variant} data-testid={format!("crate-kind-{}", node.name)}>
                        {&node.kind}
                    </Badge>
                    <span class="crate-layout__path" data-testid={format!("crate-path-{}", node.name)}>
                        {&node.path}
                    </span>
                </div>

                @if !node.modules.is_empty() {
                    <div class="crate-layout__modules" data-testid={format!("crate-modules-{}", node.name)}>
                        @for module in node.modules.iter() {
                            <div class="crate-layout__module" data-testid={format!("module-{}", module)}>
                                {module}
                            </div>
                        }
                    </div>
                }

                @if !node.children.is_empty() {
                    <div class="crate-layout__children" data-testid={format!("crate-children-{}", node.name)}>
                        @for child in node.children.iter() {
                            <CrateNodeView node={child.clone()} depth={depth + 1} />
                        }
                    </div>
                }

                @if node.modules.is_empty() && node.children.is_empty() {
                    <div class="crate-layout__empty" data-testid="crate-empty">
                        "No modules or child crates."
                    </div>
                }
            </Accordion>
        </div>
    }
}
