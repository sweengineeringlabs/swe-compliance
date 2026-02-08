pub mod structure;
pub mod naming;
pub mod content;
pub mod navigation;
pub mod cross_ref;
pub mod adr;

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

        _ => None,
    }
}
