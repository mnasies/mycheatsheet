use crate::tui::app::{FocusedPanel, TuiApp};

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

/// Renders the complete TUI interface for the current frame.
pub fn render(frame: &mut Frame, app: &TuiApp) {
    let size = frame.area();

    // Main vertical layout: Top search bar, middle content area, bottom status/hints
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(5),    // Content panels
            Constraint::Length(1), // Key hint & status line
        ])
        .split(size);

    render_search_bar(frame, app, main_chunks[0]);
    render_content_panels(frame, app, main_chunks[1]);
    render_status_line(frame, app, main_chunks[2]);

    if app.show_help {
        render_help_modal(frame, size);
    }
}

fn render_search_bar(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let lang_badge = match &app.active_lang {
        Some(lang) => format!(" [{}] ", lang.to_uppercase()),
        None => " [ALL] ".to_string(),
    };

    let border_style = match app.focused_panel {
        FocusedPanel::Search => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::DarkGray),
    };

    let search_spans = vec![
        Span::styled(
            lang_badge,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("🔍 "),
        Span::styled(
            &app.query,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        if app.focused_panel == FocusedPanel::Search {
            Span::styled("█", Style::default().fg(Color::Yellow))
        } else {
            Span::raw("")
        },
    ];

    let search_paragraph = Paragraph::new(Line::from(search_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(Span::styled(
                " Search (type query or 'go: term') ",
                Style::default().add_modifier(Modifier::BOLD),
            )),
    );

    frame.render_widget(search_paragraph, area);
}

fn render_content_panels(frame: &mut Frame, app: &TuiApp, area: Rect) {
    // Horizontal split: Left results list (35%), Right note view (65%)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    render_results_list(frame, app, chunks[0]);
    render_note_viewer(frame, app, chunks[1]);
}

fn render_results_list(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let border_style = match app.focused_panel {
        FocusedPanel::Results => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::DarkGray),
    };

    let items: Vec<ListItem> = app
        .results
        .iter()
        .enumerate()
        .map(|(idx, res)| {
            let is_selected = idx == app.selected_index;
            let lang_color = match res.lang.as_str() {
                "go" => Color::Cyan,
                "rust" => Color::Red,
                "cpp" => Color::Blue,
                "js" => Color::Yellow,
                _ => Color::Green,
            };

            let prefix = if is_selected { "▶ " } else { "  " };

            let content = Line::from(vec![
                Span::styled(
                    prefix,
                    if is_selected {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    },
                ),
                Span::styled(
                    format!("{}:{} ", res.lang, res.part),
                    Style::default().fg(lang_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    &res.entry_title,
                    if is_selected {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                ),
            ]);

            let style = if is_selected {
                Style::default().bg(Color::Rgb(40, 44, 52))
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let title = format!(" Results ({}) ", app.results.len());
    let list_widget = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            )),
    );

    frame.render_widget(list_widget, area);
}

fn render_note_viewer(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let border_style = match app.focused_panel {
        FocusedPanel::Note => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::DarkGray),
    };

    let selected_result = app.results.get(app.selected_index);
    let selected_entry = selected_result.and_then(|r| app.find_entry(&r.lang, r.part, &r.entry_id));

    let mut lines = Vec::new();

    if let (Some(res), Some(entry)) = (selected_result, selected_entry) {
        // Header
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", res.lang_name),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("— Part {}: {}", res.part, res.part_title),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
        lines.push(Line::from(""));

        // Entry Title
        lines.push(Line::from(Span::styled(
            &entry.title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )));
        lines.push(Line::from(""));

        // Description
        if !entry.description.is_empty() {
            lines.push(Line::from(Span::styled(
                &entry.description,
                Style::default().fg(Color::Rgb(200, 200, 200)),
            )));
            lines.push(Line::from(""));
        }

        // Code blocks
        for (i, cb) in entry.code_blocks.iter().enumerate() {
            let label = if cb.language.is_empty() {
                format!("── Code Block {} (press 'y' to copy) ──", i + 1)
            } else {
                format!("── {} (press 'y' to copy) ──", cb.language)
            };

            lines.push(Line::from(Span::styled(
                label,
                Style::default().fg(Color::Yellow),
            )));

            for code_line in cb.code.lines() {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        code_line,
                        Style::default()
                            .fg(Color::Rgb(170, 220, 180))
                            .bg(Color::Rgb(25, 28, 34)),
                    ),
                ]));
            }
            lines.push(Line::from(""));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "No note selected.",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Type in the search bar above to query cheatsheets.",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title(Span::styled(
                    " Note Viewer ",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .scroll((app.note_scroll, 0))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_status_line(frame: &mut Frame, app: &TuiApp, area: Rect) {
    let mut spans = vec![
        Span::styled(" [?] ", Style::default().fg(Color::Yellow)),
        Span::raw("Help "),
        Span::styled(" [/] ", Style::default().fg(Color::Yellow)),
        Span::raw("Search "),
        Span::styled(" [↑/↓] ", Style::default().fg(Color::Yellow)),
        Span::raw("Select "),
        Span::styled(" [Enter] ", Style::default().fg(Color::Yellow)),
        Span::raw("View "),
        Span::styled(" [y] ", Style::default().fg(Color::Yellow)),
        Span::raw("Copy "),
        Span::styled(" [Tab] ", Style::default().fg(Color::Yellow)),
        Span::raw("Lang "),
        Span::styled(" [r] ", Style::default().fg(Color::Yellow)),
        Span::raw("Reload "),
        Span::styled(" [Esc] ", Style::default().fg(Color::Yellow)),
        Span::raw("Back "),
        Span::styled(" [Ctrl-C] ", Style::default().fg(Color::Yellow)),
        Span::raw("Quit"),
    ];

    if let Some((msg, _)) = &app.status_message {
        spans.push(Span::raw("  │  "));
        spans.push(Span::styled(
            msg,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let status_bar = Paragraph::new(Line::from(spans));
    frame.render_widget(status_bar, area);
}

fn render_help_modal(frame: &mut Frame, area: Rect) {
    let popup_width = 64.min(area.width.saturating_sub(4));
    let popup_height = 20.min(area.height.saturating_sub(2));

    let popup_rect = Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    frame.render_widget(Clear, popup_rect);

    let help_text = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts & Navigation",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Type / [/]        ", Style::default().fg(Color::Cyan)),
            Span::raw("Focus search input bar"),
        ]),
        Line::from(vec![
            Span::styled("  ↑ / ↓ (Ctrl-n/p)  ", Style::default().fg(Color::Cyan)),
            Span::raw("Navigate results list"),
        ]),
        Line::from(vec![
            Span::styled("  Enter             ", Style::default().fg(Color::Cyan)),
            Span::raw("Focus note viewer panel"),
        ]),
        Line::from(vec![
            Span::styled("  PgUp / PgDn (j/k) ", Style::default().fg(Color::Cyan)),
            Span::raw("Scroll active note content"),
        ]),
        Line::from(vec![
            Span::styled("  Tab               ", Style::default().fg(Color::Cyan)),
            Span::raw("Cycle active language filter (All, Go, Rust...)"),
        ]),
        Line::from(vec![
            Span::styled("  y                 ", Style::default().fg(Color::Cyan)),
            Span::raw("Copy code block to system clipboard (OSC 52 fallback)"),
        ]),
        Line::from(vec![
            Span::styled("  r                 ", Style::default().fg(Color::Cyan)),
            Span::raw("Hot-reload notes from disk without restarting"),
        ]),
        Line::from(vec![
            Span::styled("  ?                 ", Style::default().fg(Color::Cyan)),
            Span::raw("Toggle this help dialog"),
        ]),
        Line::from(vec![
            Span::styled("  Esc               ", Style::default().fg(Color::Cyan)),
            Span::raw("Return to results / Clear search / Close help"),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl-C / q        ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit mycheatsheet"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Query Shortcuts:",
            Style::default().fg(Color::Yellow),
        )),
        Line::from("  'go: defer' or 'lang:rust trait' filters by language directly."),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc or ? to close this window",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let help_block = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Span::styled(
                    " Help & Keybindings ",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Left);

    frame.render_widget(help_block, popup_rect);
}
