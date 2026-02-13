use rsc_compat::prelude::*;
use crate::features::projects::types::Project;

/// Table displaying all projects with sortable columns (FR-101).
#[component]
pub fn project_list(
    projects: Signal<Vec<Project>>,
    on_edit: Option<Box<dyn Fn(String)>>,
    on_delete: Option<Box<dyn Fn(String)>>,
) -> View {
    let on_edit = on_edit.map(|cb| std::rc::Rc::new(cb));
    let on_delete = on_delete.map(|cb| std::rc::Rc::new(cb));

    view! {
        style {
            .project-list__actions { display: flex; gap: var(--space-2); }
        }

        <Table data-testid="project-list">
            <thead>
                <tr>
                    <th>"Name"</th><th>"Scope"</th><th>"Type"</th><th>"Last Scan"</th><th>"Actions"</th>
                </tr>
            </thead>
            <tbody>
                for project in projects.get().iter() {
                    <tr data-testid={format!("project-row-{}", project.id)}>
                        <td data-testid="project-name">{&project.name}</td>
                        <td><Badge data-testid="project-scope">{project.scope.to_string()}</Badge></td>
                        <td data-testid="project-type">{project.project_type.to_string()}</td>
                        <td data-testid="project-last-scan">
                            {project.last_scan_id.as_deref().unwrap_or("Never")}
                        </td>
                        <td class="project-list__actions">
                            <Button label="Edit" variant="secondary" on:click={
                                let on_edit = on_edit.clone();
                                let pid = project.id.clone();
                                move || { if let Some(ref cb) = on_edit { cb(pid.clone()) } }
                            } data-testid="edit-btn" />
                            <Button label="Delete" variant="danger" on:click={
                                let on_delete = on_delete.clone();
                                let pid = project.id.clone();
                                move || { if let Some(ref cb) = on_delete { cb(pid.clone()) } }
                            } data-testid="delete-btn" />
                        </td>
                    </tr>
                }
            </tbody>
        </Table>
    }
}
