/// Compliance summary counts for a single project scan.
pub struct ComplianceSummary {
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub total: u32,
}

impl ComplianceSummary {
    /// Returns the compliance percentage (0.0..100.0).
    /// Returns 0.0 when total is zero to avoid division by zero.
    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }
}

/// A project entry as returned by the projects API.
pub struct DashboardProject {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub project_type: String,
    pub compliance_summary: ComplianceSummary,
}

/// A single point in the compliance trend time-series.
pub struct TrendPoint {
    pub scan_id: String,
    pub timestamp: String,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
}

impl TrendPoint {
    /// Total checks for this scan point.
    pub fn total(&self) -> u32 {
        self.passed + self.failed + self.skipped
    }

    /// Compliance percentage at this point in time.
    pub fn percentage(&self) -> f64 {
        let t = self.total();
        if t == 0 {
            0.0
        } else {
            (self.passed as f64 / t as f64) * 100.0
        }
    }
}

/// Breakdown of compliance results by SDLC category.
pub struct CategoryBreakdown {
    pub category: String,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
}

impl CategoryBreakdown {
    /// Total checks in this category.
    pub fn total(&self) -> u32 {
        self.passed + self.failed + self.skipped
    }

    /// Pass rate for this category as a percentage.
    pub fn pass_rate(&self) -> f64 {
        let t = self.total();
        if t == 0 {
            0.0
        } else {
            (self.passed as f64 / t as f64) * 100.0
        }
    }
}
