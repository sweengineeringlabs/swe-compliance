use std::rc::Rc;
use rsc_compat::prelude::*;
use crate::features::projects::types::{
    ProjectScope, ProjectType, CreateProjectRequest, UpdateProjectRequest, Project,
};
use crate::features::projects::store;

/// Project creation and editing form component (FR-100, FR-102).
///
/// Renders a FormGroup with fields for project name, root path, scope, and
/// project type. Supports two modes:
///   - Create mode: all fields empty, submits via `create_project`.
///   - Edit mode: fields pre-filled from `project` prop, submits via `update_project`.
///
/// Reference: docs/1-requirements/project_management/project_management.spec
#[component]
pub fn project_form(
    /// The project to edit, or None for creation mode.
    project: Option<Project>,
    /// Whether the form is in editing mode.
    editing: bool,
    /// Callback invoked when the user cancels the form.
    on_cancel: Option<Rc<dyn Fn()>>,
) -> View {
    // Form field signals, initialized from the project prop when editing.
    let name = signal(
        project.as_ref().map(|p| p.name.clone()).unwrap_or_default()
    );
    let root_path = signal(
        project.as_ref().map(|p| p.root_path.clone()).unwrap_or_default()
    );
    let scope = signal(
        project.as_ref()
            .map(|p| p.scope.value().to_string())
            .unwrap_or_else(|| "medium".to_string())
    );
    let project_type = signal(
        project.as_ref()
            .map(|p| p.project_type.value().to_string())
            .unwrap_or_else(|| "open_source".to_string())
    );

    let projects_store = use_context::<store::ProjectsStore>();
    let loading = store::use_loading(&projects_store);
    let error = store::use_error(&projects_store);

    // Derived validation: name and root_path are required.
    let name_v = name.clone();
    let root_path_v = root_path.clone();
    let form_valid = derived(move || {
        !name_v.get().trim().is_empty() && !root_path_v.get().trim().is_empty()
    });

    let has_error = derived({
        let error = error.clone();
        move || error.get().is_some()
    });
    let error_message = derived({
        let error = error.clone();
        move || error.get().unwrap_or_default()
    });

    let scope_submit = scope.clone();
    let project_type_submit = project_type.clone();
    let name_submit = name.clone();
    let root_path_submit = root_path.clone();
    let handle_submit: Rc<dyn Fn()> = Rc::new(move || {
        if !form_valid.get() {
            return;
        }

        let parsed_scope = ProjectScope::from_str(&scope_submit.get())
            .unwrap_or(ProjectScope::Medium);
        let parsed_type = ProjectType::from_str(&project_type_submit.get())
            .unwrap_or(ProjectType::OpenSource);

        if editing {
            if let Some(ref p) = project {
                let req = UpdateProjectRequest {
                    name: Some(name_submit.get().trim().to_string()),
                    scope: Some(parsed_scope),
                    project_type: Some(parsed_type),
                };
                store::update_project(&projects_store, p.id.clone(), req);
            }
        } else {
            let req = CreateProjectRequest {
                name: name_submit.get().trim().to_string(),
                root_path: root_path_submit.get().trim().to_string(),
                scope: parsed_scope,
                project_type: parsed_type,
            };
            store::create_project(&projects_store, req);
        }
    });

    view! {
        style {
            .project-form {
                display: flex;
                flex-direction: column;
                gap: var(--space-4);
            }

            .project-form__title {
                font-size: var(--font-size-lg);
                font-weight: 600;
                color: var(--color-text);
                margin: 0 0 var(--space-2) 0;
            }

            .project-form__error {
                color: var(--color-error);
                font-size: var(--font-size-sm);
                padding: var(--space-2) var(--space-3);
                background: var(--color-error-bg);
                border-radius: var(--radius-md);
            }

            .project-form__actions {
                display: flex;
                gap: var(--space-3);
                justify-content: flex-end;
                padding-top: var(--space-2);
            }
        }

        <div class="project-form" data-testid="project-form">
            <h2 class="project-form__title" data-testid="project-form-title">
                if editing {
                    "Edit Project"
                } else {
                    "New Project"
                }
            </h2>

            if has_error.get() {
                <div aria-live="polite" class="live-region">
                    <div class="project-form__error" data-testid="project-form-error">
                        {error_message.get()}
                    </div>
                </div>
            }

            <div class="form-group">
                <FormField label="Project Name">
                    <Input
                        value={name.clone()}
                        on:input={let n = name.clone(); move |v: String| n.set(v)}
                        placeholder="Enter project name"
                        disabled={loading.get()}
                        data-testid="project-form-name"
                    />
                </FormField>

                <FormField label="Root Path">
                    <Input
                        value={root_path.clone()}
                        on:input={let rp = root_path.clone(); move |v: String| rp.set(v)}
                        placeholder="/path/to/project"
                        disabled={editing || loading.get()}
                        data-testid="project-form-root-path"
                    />
                </FormField>

                <FormField label="Scope">
                    <Select
                        value={scope.clone()}
                        on:change={let sc = scope.clone(); move |v: String| sc.set(v)}
                        disabled={loading.get()}
                        data-testid="project-form-scope"
                    >
                        for s in ProjectScope::all() {
                            <option value={s.value()}>{s.label()}</option>
                        }
                    </Select>
                </FormField>

                <FormField label="Project Type">
                    <Select
                        value={project_type.clone()}
                        on:change={let pt = project_type.clone(); move |v: String| pt.set(v)}
                        disabled={loading.get()}
                        data-testid="project-form-type"
                    >
                        for pt in ProjectType::all() {
                            <option value={pt.value()}>{pt.label()}</option>
                        }
                    </Select>
                </FormField>

                <div class="project-form__actions">
                    <Button
                        label="Cancel"
                        variant="secondary"
                        on:click={let on_cancel = on_cancel.clone(); move || { if let Some(ref cb) = on_cancel { cb() } }}
                        disabled={loading.get()}
                        data-testid="project-form-cancel"
                    />
                    <Button
                        label={if editing { "Save Changes" } else { "Create Project" }}
                        variant="primary"
                        on:click={let hs = handle_submit.clone(); move || hs()}
                        disabled={!form_valid.get() || loading.get()}
                        data-testid="project-form-submit"
                    />
                </div>
            </div>
        </div>
    }
}
