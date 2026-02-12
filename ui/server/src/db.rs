use std::path::Path;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

/// Thread-safe database handle.
#[derive(Debug, Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

/// Project record stored in SQLite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRow {
    pub id: String,
    pub name: String,
    pub root_path: String,
    pub scope: String,
    pub project_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
    pub last_scan_id: Option<String>,
}

/// Scan record stored in SQLite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRow {
    pub id: String,
    pub project_id: String,
    pub engine: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub report_json: Option<String>,
    pub config_json: Option<String>,
}

/// SRS content record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrsRow {
    pub project_id: String,
    pub content: String,
    pub updated_at: String,
}

impl Db {
    /// Open or create the SQLite database and run migrations (NFR-502).
    pub fn open(path: &Path) -> Result<Self, AppError> {
        let conn = Connection::open(path)
            .map_err(|e| AppError::Internal(format!("failed to open database: {e}")))?;

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .map_err(|e| AppError::Internal(format!("pragma failed: {e}")))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                root_path TEXT NOT NULL,
                scope TEXT NOT NULL DEFAULT 'Small',
                project_type TEXT NOT NULL DEFAULT 'OpenSource',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0,
                last_scan_id TEXT
            );

            CREATE TABLE IF NOT EXISTS scans (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id),
                engine TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'queued',
                started_at TEXT NOT NULL,
                finished_at TEXT,
                report_json TEXT,
                config_json TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_scans_project ON scans(project_id, started_at DESC);

            CREATE TABLE IF NOT EXISTS srs_content (
                project_id TEXT PRIMARY KEY REFERENCES projects(id),
                content TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );",
        )
        .map_err(|e| AppError::Internal(format!("migration failed: {e}")))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    // ── Projects ──

    pub fn create_project(
        &self,
        name: &str,
        root_path: &str,
        scope: &str,
        project_type: &str,
    ) -> Result<ProjectRow, AppError> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO projects (id, name, root_path, scope, project_type, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, name, root_path, scope, project_type, now, now],
        )?;

        Ok(ProjectRow {
            id,
            name: name.into(),
            root_path: root_path.into(),
            scope: scope.into(),
            project_type: project_type.into(),
            created_at: now.clone(),
            updated_at: now,
            deleted: false,
            last_scan_id: None,
        })
    }

    pub fn list_projects(&self) -> Result<Vec<ProjectRow>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, root_path, scope, project_type, created_at, updated_at, deleted, last_scan_id
             FROM projects WHERE deleted = 0 ORDER BY created_at DESC",
        )?;

        let rows = stmt
            .query_map([], |row| {
                Ok(ProjectRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    root_path: row.get(2)?,
                    scope: row.get(3)?,
                    project_type: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    deleted: row.get(7)?,
                    last_scan_id: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    pub fn get_project(&self, id: &str) -> Result<ProjectRow, AppError> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, root_path, scope, project_type, created_at, updated_at, deleted, last_scan_id
             FROM projects WHERE id = ?1 AND deleted = 0",
            params![id],
            |row| {
                Ok(ProjectRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    root_path: row.get(2)?,
                    scope: row.get(3)?,
                    project_type: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    deleted: row.get(7)?,
                    last_scan_id: row.get(8)?,
                })
            },
        )
        .map_err(|_| AppError::NotFound(format!("project {id} not found")))
    }

    pub fn update_project(
        &self,
        id: &str,
        name: Option<&str>,
        scope: Option<&str>,
        project_type: Option<&str>,
    ) -> Result<ProjectRow, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        if let Some(name) = name {
            conn.execute(
                "UPDATE projects SET name = ?1, updated_at = ?2 WHERE id = ?3 AND deleted = 0",
                params![name, now, id],
            )?;
        }
        if let Some(scope) = scope {
            conn.execute(
                "UPDATE projects SET scope = ?1, updated_at = ?2 WHERE id = ?3 AND deleted = 0",
                params![scope, now, id],
            )?;
        }
        if let Some(pt) = project_type {
            conn.execute(
                "UPDATE projects SET project_type = ?1, updated_at = ?2 WHERE id = ?3 AND deleted = 0",
                params![pt, now, id],
            )?;
        }
        drop(conn);

        self.get_project(id)
    }

    pub fn delete_project(&self, id: &str) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let affected = conn.execute(
            "UPDATE projects SET deleted = 1, updated_at = ?1 WHERE id = ?2 AND deleted = 0",
            params![now, id],
        )?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("project {id} not found")));
        }
        Ok(())
    }

    // ── Scans ──

    pub fn create_scan(
        &self,
        project_id: &str,
        engine: &str,
        config_json: Option<&str>,
    ) -> Result<ScanRow, AppError> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO scans (id, project_id, engine, status, started_at, config_json)
             VALUES (?1, ?2, ?3, 'running', ?4, ?5)",
            params![id, project_id, engine, now, config_json],
        )?;

        Ok(ScanRow {
            id,
            project_id: project_id.into(),
            engine: engine.into(),
            status: "running".into(),
            started_at: now,
            finished_at: None,
            report_json: None,
            config_json: config_json.map(String::from),
        })
    }

    pub fn finish_scan(
        &self,
        scan_id: &str,
        status: &str,
        report_json: Option<&str>,
    ) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE scans SET status = ?1, finished_at = ?2, report_json = ?3 WHERE id = ?4",
            params![status, now, report_json, scan_id],
        )?;

        // Update project's last_scan_id
        conn.execute(
            "UPDATE projects SET last_scan_id = ?1, updated_at = ?2
             WHERE id = (SELECT project_id FROM scans WHERE id = ?3)",
            params![scan_id, now, scan_id],
        )?;

        Ok(())
    }

    pub fn get_scan(&self, scan_id: &str) -> Result<ScanRow, AppError> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, project_id, engine, status, started_at, finished_at, report_json, config_json
             FROM scans WHERE id = ?1",
            params![scan_id],
            |row| {
                Ok(ScanRow {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    engine: row.get(2)?,
                    status: row.get(3)?,
                    started_at: row.get(4)?,
                    finished_at: row.get(5)?,
                    report_json: row.get(6)?,
                    config_json: row.get(7)?,
                })
            },
        )
        .map_err(|_| AppError::NotFound(format!("scan {scan_id} not found")))
    }

    pub fn list_scans_for_project(&self, project_id: &str) -> Result<Vec<ScanRow>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, engine, status, started_at, finished_at, report_json, config_json
             FROM scans WHERE project_id = ?1 ORDER BY started_at DESC",
        )?;

        let rows = stmt
            .query_map(params![project_id], |row| {
                Ok(ScanRow {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    engine: row.get(2)?,
                    status: row.get(3)?,
                    started_at: row.get(4)?,
                    finished_at: row.get(5)?,
                    report_json: row.get(6)?,
                    config_json: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    /// Aggregate pass/fail/skip counts over a time period (FR-202).
    pub fn get_trends(
        &self,
        project_id: &str,
        since: Option<&str>,
    ) -> Result<Vec<TrendPoint>, AppError> {
        let conn = self.conn.lock().unwrap();
        let since_val = since.unwrap_or("1970-01-01T00:00:00Z");

        let mut stmt = conn.prepare(
            "SELECT id, started_at, report_json
             FROM scans
             WHERE project_id = ?1 AND status = 'completed' AND started_at >= ?2
             ORDER BY started_at ASC",
        )?;

        let rows = stmt
            .query_map(params![project_id, since_val], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut trends = Vec::new();
        for (scan_id, timestamp, report_json) in rows {
            if let Some(json_str) = report_json {
                if let Ok(report) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    let summary = &report["summary"];
                    trends.push(TrendPoint {
                        scan_id,
                        timestamp,
                        passed: summary["passed"].as_u64().unwrap_or(0) as u32,
                        failed: summary["failed"].as_u64().unwrap_or(0) as u32,
                        skipped: summary["skipped"].as_u64().unwrap_or(0) as u32,
                    });
                }
            }
        }

        Ok(trends)
    }

    // ── SRS Content ──

    pub fn get_srs(&self, project_id: &str) -> Result<Option<SrsRow>, AppError> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT project_id, content, updated_at FROM srs_content WHERE project_id = ?1",
            params![project_id],
            |row| {
                Ok(SrsRow {
                    project_id: row.get(0)?,
                    content: row.get(1)?,
                    updated_at: row.get(2)?,
                })
            },
        );

        match result {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Internal(format!("db error: {e}"))),
        }
    }

    pub fn save_srs(&self, project_id: &str, content: &str) -> Result<SrsRow, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO srs_content (project_id, content, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(project_id) DO UPDATE SET content = ?2, updated_at = ?3",
            params![project_id, content, now],
        )?;

        Ok(SrsRow {
            project_id: project_id.into(),
            content: content.into(),
            updated_at: now,
        })
    }
}

/// A single data point for trend charts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub scan_id: String,
    pub timestamp: String,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
}
