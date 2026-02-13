use rsc_compat::prelude::*;
use crate::features::projects::types::{
    Project, CreateProjectRequest, UpdateProjectRequest,
};
use crate::features::projects::service;

/// Reactive state store for the Projects feature (FR-100..FR-104).
///
/// Manages the project list, selection state, form visibility, and async
/// loading indicators. All mutations go through named action functions
/// that call the service layer and update signals accordingly.
///
/// Reference: docs/3-design/project_management/project_management.arch
#[derive(Clone)]
pub struct ProjectsStore {
    /// All loaded projects from the API.
    pub projects: Signal<Vec<Project>>,

    /// The currently selected project for viewing or editing.
    pub selected_project: Signal<Option<Project>>,

    /// Whether the project form (create/edit) is currently open.
    pub form_open: Signal<bool>,

    /// Whether the form is in editing mode (true) or creation mode (false).
    pub editing: Signal<bool>,

    /// Whether an async operation is in progress (loading, saving, deleting).
    pub loading: Signal<bool>,

    /// The most recent error message from a failed operation, or None if no error.
    pub error: Signal<Option<String>>,
}

impl ProjectsStore {
    /// Creates a new store with default empty/idle state.
    pub fn new() -> Self {
        Self {
            projects: signal(Vec::new()),
            selected_project: signal(None),
            form_open: signal(false),
            editing: signal(false),
            loading: signal(false),
            error: signal(None),
        }
    }
}

/// Returns a read-only reference to the projects signal.
pub fn use_projects(store: &ProjectsStore) -> Signal<Vec<Project>> {
    store.projects.clone()
}

/// Returns a read-only reference to the selected project signal.
pub fn use_selected_project(store: &ProjectsStore) -> Signal<Option<Project>> {
    store.selected_project.clone()
}

/// Returns a read-only reference to the form open signal.
pub fn use_form_open(store: &ProjectsStore) -> Signal<bool> {
    store.form_open.clone()
}

/// Returns a read-only reference to the editing signal.
pub fn use_editing(store: &ProjectsStore) -> Signal<bool> {
    store.editing.clone()
}

/// Returns a read-only reference to the loading signal.
pub fn use_loading(store: &ProjectsStore) -> Signal<bool> {
    store.loading.clone()
}

/// Returns a read-only reference to the error signal.
pub fn use_error(store: &ProjectsStore) -> Signal<Option<String>> {
    store.error.clone()
}

/// Open the project form in creation mode with no pre-selected project.
pub fn open_create_form(store: &ProjectsStore) {
    store.selected_project.set(None);
    store.editing.set(false);
    store.form_open.set(true);
    store.error.set(None);
}

/// Open the project form in editing mode with the given project pre-filled.
pub fn open_edit_form(store: &ProjectsStore, project: Project) {
    store.selected_project.set(Some(project));
    store.editing.set(true);
    store.form_open.set(true);
    store.error.set(None);
}

/// Close the project form and reset selection state.
pub fn close_form(store: &ProjectsStore) {
    store.form_open.set(false);
    store.editing.set(false);
    store.selected_project.set(None);
    store.error.set(None);
}

/// Clear the current error.
pub fn clear_error(store: &ProjectsStore) {
    store.error.set(None);
}

/// Fetch all projects from the API and update the store.
/// `GET /api/v1/projects` (FR-101)
pub fn load_projects(store: &ProjectsStore) {
    store.loading.set(true);
    store.error.set(None);

    let projects_sig = store.projects.clone();
    let error_sig = store.error.clone();
    let loading_sig = store.loading.clone();

    spawn(async move {
        match service::list_projects().await {
            Ok(list) => {
                projects_sig.set(list);
            }
            Err(err) => {
                error_sig.set(Some(err.message));
            }
        }
        loading_sig.set(false);
    });
}

/// Create a new project via the API and add it to the store.
/// `POST /api/v1/projects` (FR-100)
///
/// On success, the new project is appended to the projects list and the form
/// is closed. On failure, the error is displayed in the form.
pub fn create_project(store: &ProjectsStore, req: CreateProjectRequest) {
    store.loading.set(true);
    store.error.set(None);

    let projects_sig = store.projects.clone();
    let form_open_sig = store.form_open.clone();
    let editing_sig = store.editing.clone();
    let selected_sig = store.selected_project.clone();
    let error_sig = store.error.clone();
    let loading_sig = store.loading.clone();

    spawn(async move {
        match service::create_project(&req).await {
            Ok(project) => {
                projects_sig.update(|list| list.push(project));
                form_open_sig.set(false);
                editing_sig.set(false);
                selected_sig.set(None);
                error_sig.set(None);
            }
            Err(err) => {
                error_sig.set(Some(err.message));
            }
        }
        loading_sig.set(false);
    });
}

/// Update an existing project via the API and refresh the store.
/// `PATCH /api/v1/projects/{id}` (FR-102)
///
/// On success, the project is replaced in the list and the form is closed.
/// On failure, the error is displayed in the form.
pub fn update_project(store: &ProjectsStore, id: String, req: UpdateProjectRequest) {
    store.loading.set(true);
    store.error.set(None);

    let projects_sig = store.projects.clone();
    let form_open_sig = store.form_open.clone();
    let editing_sig = store.editing.clone();
    let selected_sig = store.selected_project.clone();
    let error_sig = store.error.clone();
    let loading_sig = store.loading.clone();

    spawn(async move {
        match service::update_project(&id, &req).await {
            Ok(updated) => {
                projects_sig.update(|list| {
                    if let Some(pos) = list.iter().position(|p| p.id == id) {
                        list[pos] = updated;
                    }
                });
                form_open_sig.set(false);
                editing_sig.set(false);
                selected_sig.set(None);
                error_sig.set(None);
            }
            Err(err) => {
                error_sig.set(Some(err.message));
            }
        }
        loading_sig.set(false);
    });
}

/// Delete a project via the API and remove it from the store.
/// `DELETE /api/v1/projects/{id}` (FR-103)
///
/// On success, the project is removed from the projects list.
/// On failure, the error message is set.
pub fn delete_project(store: &ProjectsStore, id: String) {
    store.loading.set(true);
    store.error.set(None);

    let projects_sig = store.projects.clone();
    let selected_sig = store.selected_project.clone();
    let error_sig = store.error.clone();
    let loading_sig = store.loading.clone();

    spawn(async move {
        match service::delete_project(&id).await {
            Ok(()) => {
                projects_sig.update(|list| list.retain(|p| p.id != id));
                // Clear selection if the deleted project was selected
                if let Some(ref selected) = selected_sig.get() {
                    if selected.id == id {
                        selected_sig.set(None);
                    }
                }
            }
            Err(err) => {
                error_sig.set(Some(err.message));
            }
        }
        loading_sig.set(false);
    });
}
