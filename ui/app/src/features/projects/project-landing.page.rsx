use rsc_ui::prelude::*;
use crate::features::projects::store::{self, ProjectsStore};
use crate::features::projects::project_form::ProjectForm;
use crate::features::projects::project_list::ProjectList;

/// Projects management page (FR-100..104).
component ProjectsLanding() {
    let s = use_context::<ProjectsStore>();

    { let s = s.clone(); effect(move || { store::load_projects(&s); }); }

    style {
        .projects { display: flex; flex-direction: column; gap: var(--space-4); }
        .projects__header { display: flex; justify-content: space-between; align-items: center; }
    }

    render {
        <div class="projects" data-testid="projects-landing">
            <div class="projects__header">
                <h2>"Projects"</h2>
                <Button label="New Project" variant="primary" on:click={{ let s = s.clone(); move || store::open_create_form(&s) }} data-testid="new-project-btn" />
            </div>
            @if store::use_form_open(&s).get() {
                <ProjectForm
                    project={store::use_selected_project(&s).get()}
                    editing={store::use_editing(&s).get()}
                    on_cancel={Some(Box::new({ let s = s.clone(); move || store::close_form(&s) }))}
                />
            }
            <ProjectList
                projects={store::use_projects(&s)}
            />
        </div>
    }
}
