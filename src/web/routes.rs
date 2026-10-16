//! HTML page generators and API response builders for the web UI.

use crate::notes::model::NotesDatabase;
use crate::search::index::SearchIndex;
use crate::web::render::html_escape;
use serde::Serialize;

pub const STYLE_CSS: &str = include_str!("assets/style.css");
pub const APP_JS: &str = include_str!("assets/app.js");

/// Common navigation header HTML.
fn render_header(active_lang: Option<&str>) -> String {
    let data_lang_attr = match active_lang {
        Some(l) => format!(" data-lang=\"{}\"", html_escape(l)),
        None => String::new(),
    };

    let placeholder = match active_lang {
        Some(l) => format!("Search {} syntax... (Press '/' to focus)", l),
        None => "Search all languages... (e.g. 'go: defer', press '/')".to_string(),
    };

    format!(
        "<header class=\"site-header\">\
           <div class=\"header-container\">\
             <a class=\"logo-area\" href=\"/\">\
               <span class=\"logo-title\">mycheatsheet</span>\
               <span class=\"logo-edition\">Editorial Edition</span>\
             </a>\
             <div class=\"search-container\">\
               <div class=\"search-input-wrap\">\
                 <span class=\"search-icon\" aria-hidden=\"true\">&#128269;</span>\
                 <input type=\"search\" id=\"search-input\" class=\"search-input\"{data_lang_attr} \
                   placeholder=\"{placeholder}\" autocomplete=\"off\" spellcheck=\"false\" />\
                 <kbd class=\"search-kbd\">/</kbd>\
               </div>\
               <div id=\"search-overlay\" class=\"search-results-overlay\" role=\"listbox\"></div>\
             </div>\
             <div class=\"header-controls\">\
               <button type=\"button\" id=\"theme-toggle\" class=\"theme-btn\" aria-label=\"Toggle dark mode\">&#127769; Dark</button>\
             </div>\
           </div>\
         </header>",
        data_lang_attr = data_lang_attr,
        placeholder = html_escape(&placeholder)
    )
}

/// HTML wrapper template.
fn render_page_frame(title: &str, body_content: &str, active_lang: Option<&str>) -> String {
    format!(
        "<!DOCTYPE html>\
         <html lang=\"en\">\
         <head>\
           <meta charset=\"utf-8\" />\
           <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\
           <title>{}</title>\
           <style>{}</style>\
         </head>\
         <body>\
           {}\
           {}\
           <script>{}</script>\
         </body>\
         </html>",
        html_escape(title),
        STYLE_CSS,
        render_header(active_lang),
        body_content,
        APP_JS
    )
}

/// Render the index overview page listing language cards.
pub fn render_index_html(db: &NotesDatabase) -> String {
    let mut cards_html = String::new();

    // Sort languages by configured order
    let mut langs = db.languages.clone();
    langs.sort_by_key(|l| l.order);

    for lang in &langs {
        let parts = db.get_parts_for_lang(&lang.id);
        let part_count = parts.len();
        let entry_count: usize = parts.iter().map(|p| p.entries.len()).sum();
        let version_str = lang.version.as_deref().unwrap_or("");

        cards_html.push_str(&format!(
            "<a class=\"lang-card\" href=\"/lang/{lang_id}\">\
               <div class=\"card-order\">{order:02}</div>\
               <h2 class=\"card-title\">{name}</h2>\
               <div class=\"card-version\">{version}</div>\
               <div class=\"card-footer\">\
                 <span>{part_count} parts</span>\
                 <span>{entry_count} entries &rarr;</span>\
               </div>\
             </a>",
            lang_id = html_escape(&lang.id),
            order = lang.order,
            name = html_escape(&lang.name),
            version = html_escape(version_str),
            part_count = part_count,
            entry_count = entry_count
        ));
    }

    let body = format!(
        "<main>\
           <div class=\"home-hero\">\
             <h1 class=\"home-headline\">Programming Syntax Reference</h1>\
             <p class=\"home-subhead\">\
               A concise, offline reference for idiomatic syntax patterns and language primitives.\
             </p>\
             <div class=\"magazine-divider\"></div>\
           </div>\
           <div class=\"languages-grid\">\
             {cards}\
           </div>\
         </main>",
        cards = cards_html
    );

    render_page_frame("mycheatsheet — Editorial Syntax Reference", &body, None)
}

/// Render a single language reference page with sticky TOC and all parts.
pub fn render_lang_html(db: &NotesDatabase, lang_id: &str) -> Option<String> {
    let lang = db.get_language(lang_id)?;
    let parts = db.get_parts_for_lang(lang_id);

    // Build Table of Contents
    let mut toc_items = String::new();
    for part in &parts {
        toc_items.push_str(&format!(
            "<li class=\"toc-item\">\
               <a class=\"toc-link\" href=\"#part-{part_num}\">\
                 <span class=\"toc-num\">{part_num:02}</span>\
                 <span class=\"toc-text\">{title}</span>\
               </a>\
             </li>",
            part_num = part.part,
            title = html_escape(&part.title)
        ));
    }

    // Build Main Articles
    let mut content_html = String::new();
    for part in &parts {
        let mut tags_html = String::new();
        for tag in &part.tags {
            tags_html.push_str(&format!(
                "<span class=\"tag-badge\">#{}</span>",
                html_escape(tag)
            ));
        }

        let mut entries_html = String::new();
        for entry in &entry_list_sorted(&part.entries) {
            let mut code_blocks_html = String::new();
            for block in &entry.code_blocks {
                let lang_label = if block.language.is_empty() {
                    &lang.id
                } else {
                    &block.language
                };
                code_blocks_html.push_str(&format!(
                    "<div class=\"code-wrapper\">\
                       <div class=\"code-header\">\
                         <span class=\"code-lang\">{}</span>\
                         <button type=\"button\" class=\"copy-btn\">Copy</button>\
                       </div>\
                       <pre class=\"code-block\"><code>{}</code></pre>\
                     </div>",
                    html_escape(lang_label),
                    html_escape(&block.code)
                ));
            }

            entries_html.push_str(&format!(
                "<section class=\"note-entry\" id=\"entry-{entry_id}\">\
                   <h2 class=\"entry-title\">{title}</h2>\
                   <div class=\"entry-desc\">{description}</div>\
                   {code_blocks}\
                 </section>",
                entry_id = html_escape(&entry.id),
                title = html_escape(&entry.title),
                description = html_escape(&entry.description),
                code_blocks = code_blocks_html
            ));
        }

        content_html.push_str(&format!(
            "<article class=\"part-article\" id=\"part-{part_num}\">\
               <div class=\"part-meta\">\
                 <span class=\"part-numeral\">{part_num:02}</span>\
                 <div class=\"part-title-wrap\">\
                   <h1 class=\"part-title\">{title}</h1>\
                   <div class=\"part-tags\">{tags}</div>\
                 </div>\
               </div>\
               {entries}\
             </article>",
            part_num = part.part,
            title = html_escape(&part.title),
            tags = tags_html,
            entries = entries_html
        ));
    }

    let body = format!(
        "<div class=\"lang-layout\">\
           <aside class=\"toc-sidebar\" aria-label=\"Table of Contents\">\
             <div style=\"margin-bottom: 1.25rem;\">\
               <a href=\"/\" style=\"color: var(--accent); text-decoration: none; font-size: 0.95rem; font-weight: 600;\">\
                 &larr; All Languages\
               </a>\
             </div>\
             <div class=\"toc-title\">{lang_name} Contents</div>\
             <ul class=\"toc-list\">\
               {toc_items}\
             </ul>\
           </aside>\
           <main class=\"lang-content\">\
             {content_html}\
           </main>\
         </div>",
        lang_name = html_escape(&lang.name),
        toc_items = toc_items,
        content_html = content_html
    );

    let page_title = format!("{} Cheatsheet — mycheatsheet", lang.name);
    Some(render_page_frame(&page_title, &body, Some(lang_id)))
}

fn entry_list_sorted(
    entries: &[crate::notes::model::NoteEntry],
) -> Vec<&crate::notes::model::NoteEntry> {
    entries.iter().collect()
}

#[derive(Serialize)]
pub struct SearchResultDto {
    pub lang: String,
    pub lang_name: String,
    pub part: u32,
    pub part_title: String,
    pub entry_id: String,
    pub entry_title: String,
    pub snippet: String,
    pub score: i64,
}

/// Execute a search and serialize results to JSON.
pub fn handle_api_search(index: &SearchIndex, query: &str, lang_filter: Option<&str>) -> String {
    let results = index.search(query, lang_filter);
    let dtos: Vec<SearchResultDto> = results
        .into_iter()
        .map(|r| SearchResultDto {
            lang: r.lang,
            lang_name: r.lang_name,
            part: r.part,
            part_title: r.part_title,
            entry_id: r.entry_id,
            entry_title: r.entry_title,
            snippet: r.snippet,
            score: r.score.round() as i64,
        })
        .collect();

    serde_json::to_string(&dtos).unwrap_or_else(|_| "[]".to_string())
}

/// Serialize a specific note part to JSON.
pub fn handle_api_note_part(db: &NotesDatabase, lang_id: &str, part_num: u32) -> Option<String> {
    let part = db.get_part(lang_id, part_num)?;
    serde_json::to_string(&part).ok()
}
