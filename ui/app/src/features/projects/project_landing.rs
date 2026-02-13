use std::rc::Rc;
use rsc_compat::prelude::*;
use crate::features::projects::store::{self, ProjectsStore};
use crate::features::projects::project_form::project_form;
use crate::features::projects::project_list::project_list;

/// Projects management page (FR-100..104).
#[component]
pub fn projects_landing() -> View {
    let s = use_context::<ProjectsStore>();
    effect({
        let s = s.clone();
        move || { store::load_projects(&s); }
    });

    let s_btn = s.clone();
    let s_form = s.clone();
    let s_cancel = s.clone();
    let s_edit = s.clone();
    let s_delete = s.clone();

    view! {
        style {
            .projects { display: flex; flex-direction: column; gap: var(--space-4); }
            .projects__header { display: flex; justify-content: space-between; align-items: center; }
        }

        <div class="projects" data-testid="projects-landing">
            <div class="projects__header">
                <h2>"Projects"</h2>
                <Button label="New Project" variant="primary" on:click={
                    let s = s_btn.clone();
                    move || store::open_create_form(&s)
                } data-testid="new-project-btn" />
            </div>
            if store::use_form_open(&s_form).get() {
                {project_form(
                    store::use_selected_project(&s_form).get(),
                    store::use_editing(&s_form).get(),
                    Some(Rc::new({
                        let s = s_cancel.clone();
                        move || store::close_form(&s)
                    })),
                )}
            }
            {project_list(
                store::use_projects(&s),
                Some(Box::new({
                    let s = s_edit.clone();
                    move |id: String| {
                        if let Some(p) = s.projects.get().iter().find(|p| p.id == id).cloned() {
                            store::open_edit_form(&s, p);
                        }
                    }
                })),
                Some(Box::new({
                    let s = s_delete.clone();
                    move |id: String| store::delete_project(&s, id)
                })),
            )}
        </div>
    }
}
