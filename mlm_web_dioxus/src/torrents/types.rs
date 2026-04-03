use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TorrentsPageSort {
    Kind,
    Category,
    Title,
    Edition,
    Authors,
    Narrators,
    Series,
    Language,
    Size,
    Linker,
    QbitCategory,
    Linked,
    CreatedAt,
    UploadedAt,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TorrentsPageFilter {
    Kind,
    Category,
    Categories,
    Flags,
    Title,
    Author,
    Narrator,
    Series,
    Language,
    Filetype,
    Linker,
    QbitCategory,
    Linked,
    LibraryMismatch,
    ClientStatus,
    Abs,
    Query,
    Source,
    Metadata,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TorrentsBulkAction {
    Refresh,
    Relink,
    RefreshRelink,
    Clean,
    Remove,
}

impl TorrentsBulkAction {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Refresh => "refresh metadata",
            Self::Relink => "relink",
            Self::RefreshRelink => "refresh metadata and relink",
            Self::Clean => "clean torrent",
            Self::Remove => "remove torrent from MLM",
        }
    }

    fn completed_label(self) -> &'static str {
        match self {
            Self::Refresh => "Refreshed metadata for",
            Self::Relink => "Relinked",
            Self::RefreshRelink => "Refreshed metadata and relinked",
            Self::Clean => "Cleaned",
            Self::Remove => "Removed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TorrentsBulkActionFailure {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TorrentsBulkActionResult {
    pub succeeded_count: usize,
    pub failed: Vec<TorrentsBulkActionFailure>,
}

impl TorrentsBulkActionResult {
    pub(crate) fn all_failed(&self) -> bool {
        self.succeeded_count == 0 && !self.failed.is_empty()
    }

    pub(crate) fn status_message(&self, action: TorrentsBulkAction) -> (String, bool) {
        let mut message = format!(
            "{} {}.",
            action.completed_label(),
            torrent_count_label(self.succeeded_count)
        );

        if !self.failed.is_empty() {
            let failed_titles = self
                .failed
                .iter()
                .map(|failure| failure.title.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            message.push_str(&format!(
                " {} failed: {}.",
                torrent_count_label(self.failed.len()),
                failed_titles
            ));
        }

        (message, self.all_failed())
    }
}

fn torrent_count_label(count: usize) -> String {
    match count {
        1 => "1 torrent".to_string(),
        n => format!("{n} torrents"),
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct TorrentsPageColumns {
    pub category: bool,
    pub categories: bool,
    pub flags: bool,
    pub edition: bool,
    pub authors: bool,
    pub narrators: bool,
    pub series: bool,
    pub language: bool,
    pub size: bool,
    pub filetypes: bool,
    pub linker: bool,
    pub qbit_category: bool,
    pub path: bool,
    pub created_at: bool,
    pub uploaded_at: bool,
}

impl Default for TorrentsPageColumns {
    fn default() -> Self {
        Self {
            category: false,
            categories: false,
            flags: false,
            edition: false,
            authors: true,
            narrators: true,
            series: true,
            language: false,
            size: true,
            filetypes: true,
            linker: false,
            qbit_category: false,
            path: false,
            created_at: true,
            uploaded_at: false,
        }
    }
}

impl TorrentsPageColumns {
    pub(crate) fn table_grid_template(self) -> String {
        let mut cols = vec!["30px", if self.category { "130px" } else { "89px" }];
        if self.categories {
            cols.push("1fr");
        }
        if self.flags {
            cols.push("60px");
        }
        cols.push("2fr");
        if self.edition {
            cols.push("80px");
        }
        if self.authors {
            cols.push("1fr");
        }
        if self.narrators {
            cols.push("1fr");
        }
        if self.series {
            cols.push("1fr");
        }
        if self.language {
            cols.push("100px");
        }
        if self.size {
            cols.push("81px");
        }
        if self.filetypes {
            cols.push("100px");
        }
        if self.linker {
            cols.push("130px");
        }
        if self.qbit_category {
            cols.push("100px");
        }
        cols.push(if self.path { "2fr" } else { "72px" });
        if self.created_at {
            cols.push("157px");
        }
        if self.uploaded_at {
            cols.push("157px");
        }
        cols.join(" ")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TorrentLibraryMismatch {
    NewLibraryDir(String),
    NewPath(String),
    NoLibrary,
}

impl TorrentLibraryMismatch {
    pub(crate) fn filter_value(&self) -> &'static str {
        match self {
            Self::NewLibraryDir(_) => "new_library",
            Self::NewPath(_) => "new_path",
            Self::NoLibrary => "no_library",
        }
    }

    pub(crate) fn title(&self) -> String {
        match self {
            Self::NewLibraryDir(path) => format!("Wanted library dir: {path}"),
            Self::NewPath(path) => format!("Wanted library path: {path}"),
            Self::NoLibrary => "No longer wanted in library".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TorrentsMeta {
    pub title: String,
    pub media_type: String,
    pub cat_name: String,
    pub cat_id: Option<String>,
    pub categories: Vec<String>,
    pub flags: Vec<String>,
    pub edition: Option<String>,
    pub authors: Vec<String>,
    pub narrators: Vec<String>,
    pub series: Vec<crate::dto::Series>,
    pub language: Option<String>,
    pub size: String,
    pub filetypes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TorrentsRow {
    pub id: String,
    pub mam_id: Option<u64>,
    pub meta: TorrentsMeta,
    pub linker: Option<String>,
    pub category: Option<String>,
    pub library_path: Option<String>,
    pub library_mismatch: Option<TorrentLibraryMismatch>,
    pub client_status: Option<String>,
    pub linked: bool,
    pub created_at: String,
    pub uploaded_at: String,
    pub abs_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TorrentsData {
    pub torrents: Vec<TorrentsRow>,
    pub total: usize,
    pub from: usize,
    pub page_size: usize,
    pub abs_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{TorrentsBulkAction, TorrentsBulkActionFailure, TorrentsBulkActionResult};

    #[test]
    fn bulk_action_result_formats_partial_success() {
        let result = TorrentsBulkActionResult {
            succeeded_count: 1,
            failed: vec![TorrentsBulkActionFailure {
                id: "torrent-032".to_string(),
                title: "Test Book 032".to_string(),
            }],
        };

        assert_eq!(
            result.status_message(TorrentsBulkAction::Relink),
            (
                "Relinked 1 torrent. 1 torrent failed: Test Book 032.".to_string(),
                false,
            )
        );
    }

    #[test]
    fn bulk_action_result_marks_full_failure_as_error() {
        let result = TorrentsBulkActionResult {
            succeeded_count: 0,
            failed: vec![TorrentsBulkActionFailure {
                id: "torrent-033".to_string(),
                title: "Test Book 033".to_string(),
            }],
        };

        assert_eq!(
            result.status_message(TorrentsBulkAction::RefreshRelink),
            (
                "Refreshed metadata and relinked 0 torrents. 1 torrent failed: Test Book 033."
                    .to_string(),
                true,
            )
        );
    }
}
