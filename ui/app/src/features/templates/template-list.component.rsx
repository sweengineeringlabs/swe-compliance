use rsc_ui::prelude::*;
use crate::features::templates::templates_type::TemplateEntry;

/// Table listing available templates with name, category, file count, and tags (FR-600).
component TemplateList(
    templates: Signal<Vec<TemplateEntry>>,
    on_select: Fn(TemplateEntry),
    loading: bool,
) {
    style {
        .template-list { display: flex; flex-direction: column; gap: var(--space-3); }
        .template-list__tags { display: flex; gap: var(--space-1); flex-wrap: wrap; }
    }

    render {
        <div class="template-list" data-testid="template-list">
            @if loading {
                <p data-testid="template-list-loading">"Loading templates..."</p>
            } @else if templates.get().is_empty() {
                <p data-testid="template-list-empty">"No templates available."</p>
            } @else {
                <Table data-testid="template-list-table">
                    <thead>
                        <tr>
                            <th>"Name"</th>
                            <th>"Category"</th>
                            <th>"Files"</th>
                            <th>"Tags"</th>
                        </tr>
                    </thead>
                    <tbody>
                        @for tpl in templates.get().iter() {
                            <tr on:click={let t = tpl.clone(); move || on_select(t.clone())}
                                data-testid={format!("template-row-{}", tpl.name)}>
                                <td data-testid="template-name">{&tpl.name}</td>
                                <td data-testid="template-category">{&tpl.category}</td>
                                <td data-testid="template-file-count">{tpl.file_count}</td>
                                <td>
                                    <div class="template-list__tags">
                                        @for tag in tpl.tags.iter() {
                                            <Badge variant="info" data-testid={format!("template-tag-{}", tag)}>
                                                {tag}
                                            </Badge>
                                        }
                                    </div>
                                </td>
                            </tr>
                        }
                    </tbody>
                </Table>
            }
        </div>
    }
}
