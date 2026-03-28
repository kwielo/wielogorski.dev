use crate::data::{CONTACT_LINKS, PROJECTS, WIFE_LINKS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    About,
    Projects,
    Contact,
    Wife,
}

impl Section {
    pub const ALL: &'static [Section] = &[
        Section::About,
        Section::Projects,
        Section::Contact,
        Section::Wife,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Section::About => "About",
            Section::Projects => "Pet Projects",
            Section::Contact => "Contact",
            Section::Wife => "Wife <3",
        }
    }

    /// Number of selectable items in the right panel for this section.
    pub fn item_count(self) -> usize {
        match self {
            Section::About => 0,
            Section::Projects => PROJECTS.len(),
            Section::Contact => CONTACT_LINKS.len(),
            Section::Wife => WIFE_LINKS.len(),
        }
    }

    /// URL for the currently selected item (if any).
    pub fn selected_url(self, item: usize) -> Option<&'static str> {
        match self {
            Section::About => None,
            Section::Projects => PROJECTS.get(item).map(|p| p.url),
            Section::Contact => CONTACT_LINKS.get(item).map(|l| l.url),
            Section::Wife => WIFE_LINKS.get(item).map(|l| l.url),
        }
    }
}

pub struct App {
    pub section_index: usize, // index into Section::ALL
    pub item_index: usize,    // selected item within right panel
    pub should_quit: bool,
    pub status_msg: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            section_index: 0,
            item_index: 0,
            should_quit: false,
            status_msg: None,
        }
    }

    pub fn current_section(&self) -> Section {
        Section::ALL[self.section_index]
    }

    pub fn nav_up(&mut self) {
        let sec = self.current_section();
        if sec.item_count() == 0 {
            // navigate sections
            if self.section_index > 0 {
                self.section_index -= 1;
                self.item_index = 0;
                self.status_msg = None;
            }
        } else if self.item_index > 0 {
            self.item_index -= 1;
            self.status_msg = None;
        }
    }

    pub fn nav_down(&mut self) {
        let sec = self.current_section();
        if sec.item_count() == 0 {
            if self.section_index + 1 < Section::ALL.len() {
                self.section_index += 1;
                self.item_index = 0;
                self.status_msg = None;
            }
        } else if self.item_index + 1 < sec.item_count() {
            self.item_index += 1;
            self.status_msg = None;
        }
    }

    pub fn nav_section_up(&mut self) {
        if self.section_index > 0 {
            self.section_index -= 1;
            self.item_index = 0;
            self.status_msg = None;
        }
    }

    pub fn nav_section_down(&mut self) {
        if self.section_index + 1 < Section::ALL.len() {
            self.section_index += 1;
            self.item_index = 0;
            self.status_msg = None;
        }
    }

    pub fn open_selected(&mut self) {
        let sec = self.current_section();
        if let Some(url) = sec.selected_url(self.item_index) {
            match open::that(url) {
                Ok(_) => {
                    self.status_msg = Some(format!("Opened: {url}"));
                }
                Err(e) => {
                    self.status_msg = Some(format!("Failed to open URL: {e}"));
                }
            }
        }
    }
}
