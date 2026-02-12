use rsc_ui::prelude::*;
use crate::features::projects::projects_type::{
    ProjectScope, ProjectType, CreateProjectRequest, UpdateProjectRequest, Project,
};
use crate::features::projects::projects_store;

/// Project creation and editing form component (FR-100, FR-102).
///
/// Renders a FormGroup with fields for project name, root path, scope, and
/// project type. Supports two modes:
///   - Create mode: all fields empty, submits via `create_project`.
///   - Edit mode: fields pre-filled from `project` prop, submits via `update_project`.
///
/// Reference: docs/1-requirements/project_management/project_management.spec
component ProjectForm(
    /// The project to edit, or None for creation mode.
    project: Option<Project>,
    /// Whether the form is in editing mode.
    editing: bool,
    /// Callback invoked when the user cancels the form.
    on_cancel: Box<dyn Fn()>,
) {
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

    let loading = projects_store::use_loading();
    let error = projects_store::use_error();

    // Derived validation: name and root_path are required.
    let form_valid = derived(|| {
        !name.get().trim().is_empty() && !root_path.get().trim().is_empty()
    });

    let handle_submit = move || {
        if !form_valid.get() {
            return;
        }

        let parsed_scope = ProjectScope::from_str(&scope.get())
            .unwrap_or(ProjectScope::Medium);
        let parsed_type = ProjectType::from_str(&project_type.get())
            .unwrap_or(ProjectType::OpenSource);

        if editing {
            if let Some(ref p) = project {
                let req = UpdateProjectRequest {
                    name: Some(name.get().trim().to_string()),
                    scope: Some(parsed_scope),
                    project_type: Some(parsed_type),
                };
                projects_store::update_project(p.id.clone(), req);
            }
        } else {
            let req = CreateProjectRequest {
                name: name.get().trim().to_string(),
                root_path: root_path.get().trim().to_string(),
                scope: parsed_scope,
                project_type: parsed_type,
            };
            projects_store::create_project(req);
        }
    };

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

    render {
        <div class="project-form" data-testid="project-form">
            <h2 class="project-form__title" data-testid="project-form-title">
                @if editing {
                    "Edit Project"
                } @else {
                    "New Project"
                }
            </h2>

            @if let Some(err) = error.get() {
                <LiveRegion>
                    <div class="project-form__error" data-testid="project-form-error">
                        {err}
                    </div>
                </LiveRegion>
            }

            <FormGroup>
                <FormField label="Project Name">
                    <Input
                        value={name}
                        on:input={move |v: String| name.set(v)}
                        placeholder="Enter project name"
                        disabled={loading.get()}
                        data-testid="project-form-name"
                    />
                </FormField>

                <FormField label="Root Path">
                    <Input
                        value={root_path}
                        on:input={move |v: String| root_path.set(v)}
                        placeholder="/path/to/project"
                        disabled={editing || loading.get()}
                        data-testid="project-form-root-path"
                    />
                </FormField>

                <FormField label="Scope">
                    <Select
                        value={scope}
                        on:change={move |v: String| scope.set(v)}
                        disabled={loading.get()}
                        data-testid="project-form-scope"
                    >
                        @for s in ProjectScope::all() {
                            <option value={s.value()}>{s.label()}</option>
                        }
                    </Select>
                </FormField>

                <FormField label="Project Type">
                    <Select
                        value={project_type}
                        on:change={move |v: String| project_type.set(v)}
                        disabled={loading.get()}
                        data-testid="project-form-type"
                    >
                        @for pt in ProjectType::all() {
                            <option value={pt.value()}>{pt.label()}</option>
                        }
                    </Select>
                </FormField>

                <div class="project-form__actions">
                    <Button
                        label="Cancel"
                        variant="secondary"
                        on:click={move || (on_cancel)()}
                        disabled={loading.get()}
                        data-testid="project-form-cancel"
                    />
                    <Button
                        label={if editing { "Save Changes" } else { "Create Project" }}
                        variant="primary"
                        on:click={handle_submit}
                        disabled={!form_valid.get() || loading.get()}
                        data-testid="project-form-submit"
                    />
                </div>
            </FormGroup>
        </div>
    }
}
