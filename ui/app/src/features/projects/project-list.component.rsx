use rsc_ui::prelude::*;
use crate::features::projects::projects_type::Project;

/// Table displaying all projects with sortable columns (FR-101).
component ProjectList(
    projects: Signal<Vec<Project>>,
    on_edit: Fn(String),
    on_delete: Fn(String),
) {
    style {
        .project-list__actions { display: flex; gap: var(--space-2); }
    }

    render {
        <Table data-testid="project-list">
            <thead>
                <tr>
                    <th>"Name"</th><th>"Scope"</th><th>"Type"</th><th>"Last Scan"</th><th>"Actions"</th>
                </tr>
            </thead>
            <tbody>
                @for project in projects.get().iter() {
                    <tr data-testid={format!("project-row-{}", project.id)}>
                        <td data-testid="project-name">{&project.name}</td>
                        <td><Badge data-testid="project-scope">{&project.scope}</Badge></td>
                        <td data-testid="project-type">{&project.project_type}</td>
                        <td data-testid="project-last-scan">
                            {project.last_scan_id.as_deref().unwrap_or("Never")}
                        </td>
                        <td class="project-list__actions">
                            <Button label="Edit" variant="secondary" on:click={|| on_edit(project.id.clone())} data-testid="edit-btn" />
                            <Button label="Delete" variant="danger" on:click={|| on_delete(project.id.clone())} data-testid="delete-btn" />
                        </td>
                    </tr>
                }
            </tbody>
        </Table>
    }
}
