use rsc_ui::prelude::*;
use crate::features::projects::projects_type::{
    Project, CreateProjectRequest, UpdateProjectRequest,
};
use crate::features::projects::projects_service;

/// Reactive state store for the Projects feature (FR-100..FR-104).
///
/// Manages the project list, selection state, form visibility, and async
/// loading indicators. All mutations go through named action functions
/// that call the service layer and update signals accordingly.
///
/// Reference: docs/3-design/project_management/project_management.arch

/// All loaded projects from the API.
static PROJECTS: Signal<Vec<Project>> = signal(Vec::new());

/// The currently selected project for viewing or editing.
static SELECTED_PROJECT: Signal<Option<Project>> = signal(None);

/// Whether the project form (create/edit) is currently open.
static FORM_OPEN: Signal<bool> = signal(false);

/// Whether the form is in editing mode (true) or creation mode (false).
static EDITING: Signal<bool> = signal(false);

/// Whether an async operation is in progress (loading, saving, deleting).
static LOADING: Signal<bool> = signal(false);

/// The most recent error message from a failed operation, or None if no error.
static ERROR: Signal<Option<String>> = signal(None);

/// Returns a read-only reference to the projects signal.
pub fn use_projects() -> Signal<Vec<Project>> {
    PROJECTS
}

/// Returns a read-only reference to the selected project signal.
pub fn use_selected_project() -> Signal<Option<Project>> {
    SELECTED_PROJECT
}

/// Returns a read-only reference to the form open signal.
pub fn use_form_open() -> Signal<bool> {
    FORM_OPEN
}

/// Returns a read-only reference to the editing signal.
pub fn use_editing() -> Signal<bool> {
    EDITING
}

/// Returns a read-only reference to the loading signal.
pub fn use_loading() -> Signal<bool> {
    LOADING
}

/// Returns a read-only reference to the error signal.
pub fn use_error() -> Signal<Option<String>> {
    ERROR
}

/// Open the project form in creation mode with no pre-selected project.
pub fn open_create_form() {
    SELECTED_PROJECT.set(None);
    EDITING.set(false);
    FORM_OPEN.set(true);
    ERROR.set(None);
}

/// Open the project form in editing mode with the given project pre-filled.
pub fn open_edit_form(project: Project) {
    SELECTED_PROJECT.set(Some(project));
    EDITING.set(true);
    FORM_OPEN.set(true);
    ERROR.set(None);
}

/// Close the project form and reset selection state.
pub fn close_form() {
    FORM_OPEN.set(false);
    EDITING.set(false);
    SELECTED_PROJECT.set(None);
    ERROR.set(None);
}

/// Clear the current error.
pub fn clear_error() {
    ERROR.set(None);
}

/// Fetch all projects from the API and update the store.
/// `GET /api/v1/projects` (FR-101)
pub fn load_projects() {
    LOADING.set(true);
    ERROR.set(None);

    spawn(async move {
        match projects_service::list_projects().await {
            Ok(list) => {
                PROJECTS.set(list);
            }
            Err(err) => {
                ERROR.set(Some(err.message));
            }
        }
        LOADING.set(false);
    });
}

/// Create a new project via the API and add it to the store.
/// `POST /api/v1/projects` (FR-100)
///
/// On success, the new project is appended to the projects list and the form
/// is closed. On failure, the error is displayed in the form.
pub fn create_project(req: CreateProjectRequest) {
    LOADING.set(true);
    ERROR.set(None);

    spawn(async move {
        match projects_service::create_project(&req).await {
            Ok(project) => {
                PROJECTS.update(|list| list.push(project));
                close_form();
            }
            Err(err) => {
                ERROR.set(Some(err.message));
            }
        }
        LOADING.set(false);
    });
}

/// Update an existing project via the API and refresh the store.
/// `PATCH /api/v1/projects/{id}` (FR-102)
///
/// On success, the project is replaced in the list and the form is closed.
/// On failure, the error is displayed in the form.
pub fn update_project(id: String, req: UpdateProjectRequest) {
    LOADING.set(true);
    ERROR.set(None);

    spawn(async move {
        match projects_service::update_project(&id, &req).await {
            Ok(updated) => {
                PROJECTS.update(|list| {
                    if let Some(pos) = list.iter().position(|p| p.id == id) {
                        list[pos] = updated;
                    }
                });
                close_form();
            }
            Err(err) => {
                ERROR.set(Some(err.message));
            }
        }
        LOADING.set(false);
    });
}

/// Delete a project via the API and remove it from the store.
/// `DELETE /api/v1/projects/{id}` (FR-103)
///
/// On success, the project is removed from the projects list.
/// On failure, the error message is set.
pub fn delete_project(id: String) {
    LOADING.set(true);
    ERROR.set(None);

    spawn(async move {
        match projects_service::delete_project(&id).await {
            Ok(()) => {
                PROJECTS.update(|list| list.retain(|p| p.id != id));
                // Clear selection if the deleted project was selected
                if let Some(ref selected) = SELECTED_PROJECT.get() {
                    if selected.id == id {
                        SELECTED_PROJECT.set(None);
                    }
                }
            }
            Err(err) => {
                ERROR.set(Some(err.message));
            }
        }
        LOADING.set(false);
    });
}
