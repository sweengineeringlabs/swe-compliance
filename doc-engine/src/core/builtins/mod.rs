pub mod structure;
pub mod naming;
pub mod content;
pub mod navigation;
pub mod cross_ref;
pub mod adr;
pub mod traceability;
pub mod module;
pub mod requirements;

use crate::api::types::RuleDef;
use crate::spi::traits::CheckRunner;

pub fn get_handler(name: &str, def: &RuleDef) -> Option<Box<dyn CheckRunner>> {
    match name {
        // Structure handlers
        "module_docs_plural" => Some(Box::new(structure::ModuleDocsPlural { def: def.clone() })),
        "sdlc_phase_numbering" => Some(Box::new(structure::SdlcPhaseNumbering { def: def.clone() })),
        "checklist_completeness" => Some(Box::new(structure::ChecklistCompleteness { def: def.clone() })),
        "open_source_community_files" => Some(Box::new(structure::OpenSourceCommunityFiles { def: def.clone() })),
        "open_source_github_templates" => Some(Box::new(structure::OpenSourceGithubTemplates { def: def.clone() })),

        // Naming handlers
        "snake_lower_case" => Some(Box::new(naming::SnakeLowerCase { def: def.clone() })),
        "guide_naming" => Some(Box::new(naming::GuideNaming { def: def.clone() })),
        "testing_file_placement" => Some(Box::new(naming::TestingFilePlacement { def: def.clone() })),

        // Content handlers
        "tldr_conditional" => Some(Box::new(content::TldrConditional { def: def.clone() })),
        "glossary_format" => Some(Box::new(content::GlossaryFormat { def: def.clone() })),
        "glossary_alphabetized" => Some(Box::new(content::GlossaryAlphabetized { def: def.clone() })),
        "glossary_acronyms" => Some(Box::new(content::GlossaryAcronyms { def: def.clone() })),

        // Navigation handlers
        "w3h_hub" => Some(Box::new(navigation::W3hHub { def: def.clone() })),
        "hub_links_phases" => Some(Box::new(navigation::HubLinksPhases { def: def.clone() })),
        "no_deep_links" => Some(Box::new(navigation::NoDeepLinks { def: def.clone() })),

        // Cross-reference handlers
        "link_resolution" => Some(Box::new(cross_ref::LinkResolution { def: def.clone() })),

        // ADR handlers
        "adr_naming" => Some(Box::new(adr::AdrNaming { def: def.clone() })),
        "adr_index_completeness" => Some(Box::new(adr::AdrIndexCompleteness { def: def.clone() })),

        // Traceability handlers
        "phase_artifact_presence" => Some(Box::new(traceability::PhaseArtifactPresence { def: def.clone() })),
        "design_traces_requirements" => Some(Box::new(traceability::DesignTracesRequirements { def: def.clone() })),
        "plan_traces_design" => Some(Box::new(traceability::PlanTracesDesign { def: def.clone() })),
        "backlog_traces_requirements" => Some(Box::new(traceability::BacklogTracesRequirements { def: def.clone() })),

        // Structure handlers (new)
        "templates_populated" => Some(Box::new(structure::TemplatesPopulated { def: def.clone() })),

        // Navigation handlers (new)
        "w3h_extended" => Some(Box::new(navigation::W3hExtended { def: def.clone() })),

        // Content handlers (new)
        "readme_line_count" => Some(Box::new(content::ReadmeLineCount { def: def.clone() })),

        // Naming handlers (new)
        "fr_naming" => Some(Box::new(naming::FrNaming { def: def.clone() })),

        // Module handlers
        "module_readme_w3h" => Some(Box::new(module::ModuleReadmeW3h { def: def.clone() })),
        "module_examples_tests" => Some(Box::new(module::ModuleExamplesTests { def: def.clone() })),
        "module_toolchain_docs" => Some(Box::new(module::ModuleToolchainDocs { def: def.clone() })),
        "module_deployment_docs" => Some(Box::new(module::ModuleDeploymentDocs { def: def.clone() })),

        // Requirements handlers
        "srs_29148_attributes" => Some(Box::new(requirements::Srs29148Attributes { def: def.clone() })),
        "arch_42010_sections" => Some(Box::new(requirements::Arch42010Sections { def: def.clone() })),
        "test_29119_sections" => Some(Box::new(requirements::Test29119Sections { def: def.clone() })),

        _ => None,
    }
}
