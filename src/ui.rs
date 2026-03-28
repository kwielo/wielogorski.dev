use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Section};
use crate::data::{BIO_PARAGRAPHS, CONTACT_LINKS, LANGUAGES, PROJECTS, TOOLS, WIFE_LINKS};

const ACCENT: Color = Color::Cyan;
const DIM: Color = Color::DarkGray;
const HIGHLIGHT_BG: Color = Color::Rgb(30, 50, 60);

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // ── outer vertical split: title / body / status ──────────────────────
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title bar
            Constraint::Min(0),    // main area
            Constraint::Length(2), // status / help bar
        ])
        .split(area);

    draw_title(frame, root[0]);

    // ── main area: left nav (22 cols) | right content ────────────────────
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(0)])
        .split(root[1]);

    draw_nav(frame, app, main[0]);
    draw_content(frame, app, main[1]);
    draw_status(frame, app, root[2]);
}

// ── Title bar ────────────────────────────────────────────────────────────────

fn draw_title(frame: &mut Frame, area: ratatui::layout::Rect) {
    let title = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::styled(
                "Krzysztof Wielogórski",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  —  Senior Software Engineer",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(Span::styled(
            "> perfect is the enemy of good done!",
            Style::default().fg(DIM).add_modifier(Modifier::ITALIC),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(ACCENT)),
    )
    .alignment(Alignment::Center);

    frame.render_widget(title, area);
}

// ── Left nav panel ───────────────────────────────────────────────────────────

fn draw_nav(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = Section::ALL
        .iter()
        .enumerate()
        .map(|(i, sec)| {
            let selected = i == app.section_index;
            let style = if selected {
                Style::default()
                    .fg(ACCENT)
                    .bg(HIGHLIGHT_BG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if selected { "> " } else { "  " };
            ListItem::new(format!("{prefix}{}", sec.label())).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM))
            .title(Span::styled(" Menu ", Style::default().fg(ACCENT))),
    );

    frame.render_widget(list, area);
}

// ── Right content panel ──────────────────────────────────────────────────────

fn draw_content(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(DIM))
        .title(Span::styled(
            format!(" {} ", app.current_section().label()),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    match app.current_section() {
        Section::About => draw_about(frame, inner),
        Section::Projects => draw_projects(frame, app, inner),
        Section::Contact => draw_links(frame, app, CONTACT_LINKS, inner),
        Section::Wife => draw_links(frame, app, WIFE_LINKS, inner),
    }
}

fn draw_about(frame: &mut Frame, area: ratatui::layout::Rect) {
    // Split into bio text (top) and skills row (bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(6)])
        .margin(1)
        .split(area);

    // Bio paragraphs
    let bio_text: Vec<Line> = BIO_PARAGRAPHS
        .iter()
        .flat_map(|p| {
            vec![
                Line::from(Span::raw(*p)),
                Line::from(""), // blank line between paragraphs
            ]
        })
        .collect();

    let bio = Paragraph::new(Text::from(bio_text)).wrap(Wrap { trim: true });
    frame.render_widget(bio, chunks[0]);

    // Skills row: languages | tools
    let skills_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let lang_lines: Vec<Line> = std::iter::once(Line::from(Span::styled(
        "Languages",
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
    )))
    .chain(LANGUAGES.iter().map(|l| Line::from(format!("  • {l}"))))
    .collect();

    let tool_lines: Vec<Line> = std::iter::once(Line::from(Span::styled(
        "Tools",
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
    )))
    .chain(TOOLS.iter().map(|t| Line::from(format!("  • {t}"))))
    .collect();

    frame.render_widget(Paragraph::new(lang_lines), skills_row[0]);
    frame.render_widget(Paragraph::new(tool_lines), skills_row[1]);
}

fn draw_projects(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let inner = area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    // Split into list (left ~30%) and detail (right ~70%)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Min(0)])
        .split(inner);

    // Project list
    let items: Vec<ListItem> = PROJECTS
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let selected = i == app.item_index;
            let style = if selected {
                Style::default()
                    .fg(ACCENT)
                    .bg(HIGHLIGHT_BG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if selected { "> " } else { "  " };
            ListItem::new(format!("{prefix}{}", p.name)).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.item_index));

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(DIM)),
    );
    frame.render_stateful_widget(list, cols[0], &mut list_state);

    // Project detail
    if let Some(project) = PROJECTS.get(app.item_index) {
        let detail_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(2)])
            .margin(1)
            .split(cols[1]);

        let desc = Paragraph::new(project.description)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));
        frame.render_widget(desc, detail_chunks[0]);

        let url = Paragraph::new(Span::styled(
            project.url,
            Style::default()
                .fg(ACCENT)
                .add_modifier(Modifier::UNDERLINED),
        ));
        frame.render_widget(url, detail_chunks[1]);
    }
}

fn draw_links(
    frame: &mut Frame,
    app: &App,
    links: &[crate::data::Link],
    area: ratatui::layout::Rect,
) {
    let inner = area.inner(Margin {
        vertical: 1,
        horizontal: 2,
    });

    let items: Vec<ListItem> = links
        .iter()
        .enumerate()
        .map(|(i, link)| {
            let selected = i == app.item_index;
            let label_style = if selected {
                Style::default()
                    .fg(ACCENT)
                    .bg(HIGHLIGHT_BG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            };
            let prefix = if selected { "> " } else { "  " };
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(prefix, label_style),
                    Span::styled(link.label, label_style),
                ]),
                Line::from(Span::styled(
                    format!("     {}", link.description),
                    Style::default().fg(DIM),
                )),
                Line::from(Span::styled(
                    format!("     {}", link.url),
                    Style::default()
                        .fg(ACCENT)
                        .add_modifier(Modifier::UNDERLINED),
                )),
                Line::from(""),
            ])
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.item_index));

    let list = List::new(items);
    frame.render_stateful_widget(list, inner, &mut list_state);
}

// ── Status / help bar ────────────────────────────────────────────────────────

fn draw_status(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let help = vec![
        Span::styled(
            " ↑↓ ",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::raw("navigate  "),
        Span::styled(
            "Tab/Shift+Tab ",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::raw("sections  "),
        Span::styled(
            "Enter/o ",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::raw("open link  "),
        Span::styled(
            "q/Esc ",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::raw("quit"),
    ];

    let status_line = if let Some(msg) = &app.status_msg {
        Line::from(Span::styled(
            msg.as_str(),
            Style::default().fg(Color::Green),
        ))
    } else {
        Line::from(help)
    };

    let status = Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(DIM)),
    );

    frame.render_widget(status, area);
}
