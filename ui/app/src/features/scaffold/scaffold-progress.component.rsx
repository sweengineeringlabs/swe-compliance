use rsc_ui::prelude::*;
use crate::features::scaffold::scaffold_type::ScaffoldResult;

/// Scaffold execution results showing created/skipped files (FR-504).
component ScaffoldProgress(result: ScaffoldResult) {
    render {
        <Card data-testid="scaffold-progress">
            <h3>"Scaffold Results"</h3>
            <div>
                <Badge variant="success" data-testid="scaffold-created-count">{format!("{} created", result.created.len())}</Badge>
                <Badge variant="secondary" data-testid="scaffold-skipped-count">{format!("{} skipped", result.skipped.len())}</Badge>
            </div>
            <Steps data-testid="scaffold-steps">
                @for file in result.created.iter() {
                    <div data-testid="scaffold-created-file"><Badge variant="success">"created"</Badge>" "{file}</div>
                }
                @for file in result.skipped.iter() {
                    <div data-testid="scaffold-skipped-file"><Badge variant="secondary">"skipped"</Badge>" "{file}</div>
                }
            </Steps>
        </Card>
    }
}
