use rsc_ui::prelude::*;
use crate::features::projects::projects_store as store;
use crate::features::projects::project_form::ProjectForm;
use crate::features::projects::project_list::ProjectList;

/// Projects management page (FR-100..104).
component ProjectsLanding() {
    effect(|| { store::load_projects(); });

    style {
        .projects { display: flex; flex-direction: column; gap: var(--space-4); }
        .projects__header { display: flex; justify-content: space-between; align-items: center; }
    }

    render {
        <div class="projects" data-testid="projects-landing">
            <div class="projects__header">
                <h2>"Projects"</h2>
                <Button label="New Project" variant="primary" on:click={|| store::form_open.set(true)} data-testid="new-project-btn" />
            </div>
            @if store::form_open.get() {
                <ProjectForm
                    on_submit={|req| store::create_project(req)}
                    on_cancel={|| store::form_open.set(false)}
                />
            }
            <ProjectList
                projects={store::projects.clone()}
                on_edit={|id| store::start_edit(&id)}
                on_delete={|id| store::delete_project(&id)}
            />
        </div>
    }
}
