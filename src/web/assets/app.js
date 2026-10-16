/**
 * mycheatsheet — Client-Side Application Script
 * Zero external dependencies. Vanilla JS for syntax highlighting,
 * copy-to-clipboard, live search, sticky TOC observer, and keepalive heartbeat.
 */

(function () {
  'use strict';

  // =========================================================================
  // 1. Theme Management (Light / Dark)
  // =========================================================================
  function initTheme() {
    const themeBtn = document.getElementById('theme-toggle');
    let currentTheme = 'light';

    try {
      const saved = localStorage.getItem('mycheatsheet_theme');
      if (saved === 'dark' || saved === 'light') {
        currentTheme = saved;
      } else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        currentTheme = 'dark';
      }
    } catch (_) {
      // Ignore localStorage failure in strict sandboxes
    }

    applyTheme(currentTheme);

    if (themeBtn) {
      themeBtn.addEventListener('click', function () {
        const next = document.documentElement.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
        applyTheme(next);
        try {
          localStorage.setItem('mycheatsheet_theme', next);
        } catch (_) {}
      });
    }
  }

  function applyTheme(theme) {
    const themeBtn = document.getElementById('theme-toggle');
    if (theme === 'dark') {
      document.documentElement.setAttribute('data-theme', 'dark');
      if (themeBtn) themeBtn.textContent = '☀️ Light';
    } else {
      document.documentElement.removeAttribute('data-theme');
      if (themeBtn) themeBtn.textContent = '🌙 Dark';
    }
  }

  // =========================================================================
  // 2. Syntax Highlighting (Regex Tokenizer)
  // =========================================================================
  function escapeHtml(str) {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
  }

  const KEYWORDS = new Set([
    'as', 'async', 'await', 'auto', 'break', 'case', 'catch', 'chan', 'class',
    'const', 'continue', 'crate', 'default', 'defer', 'delete', 'do', 'dyn',
    'else', 'enum', 'export', 'extern', 'fallthrough', 'false', 'finally',
    'fn', 'for', 'from', 'func', 'function', 'go', 'goto', 'if', 'impl', 'import',
    'in', 'instanceof', 'interface', 'let', 'loop', 'macro_rules', 'map',
    'match', 'mod', 'move', 'mut', 'namespace', 'new', 'nil', 'null', 'of',
    'operator', 'package', 'pub', 'range', 'ref', 'return', 'select', 'self',
    'Self', 'static', 'struct', 'super', 'switch', 'template', 'this', 'throw',
    'trait', 'true', 'try', 'type', 'typedef', 'typeof', 'undefined', 'union',
    'unsafe', 'use', 'using', 'var', 'virtual', 'void', 'where', 'while', 'yield'
  ]);

  const TYPES = new Set([
    'bool', 'byte', 'char', 'double', 'error', 'f32', 'f64', 'float', 'float32',
    'float64', 'i8', 'i16', 'i32', 'i64', 'i128', 'int', 'int8', 'int16', 'int32',
    'int64', 'intptr', 'isize', 'rune', 'size_t', 'string', 'String', 'u8', 'u16',
    'u32', 'u64', 'u128', 'uint', 'uint8', 'uint16', 'uint32', 'uint64', 'uintptr',
    'usize', 'Vec', 'Option', 'Result', 'Box', 'Rc', 'Arc', 'any', 'number',
    'boolean', 'Array', 'Map', 'Set', 'Promise'
  ]);

  function highlightCode(codeText) {
    // Tokenization patterns
    // 1: Comments (single line & multi-line)
    // 2: Strings (double, single, backtick, raw)
    // 3: Numbers (hex, float, int)
    // 4: Identifiers / Words
    // 5: Other chars
    const tokenRegex = /(\/\/[^\n]*|\/\*[\s\S]*?\*\/)|("(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\.|[^`\\])*`)|(\b0x[0-9a-fA-F]+\b|\b\d+(?:\.\d+)?(?:[eE][+-]?\d+)?\b)|([a-zA-Z_][a-zA-Z0-9_]*)|([\s\S])/g;

    let html = '';
    let match;

    while ((match = tokenRegex.exec(codeText)) !== null) {
      const [full, comment, string, number, ident] = match;

      if (comment) {
        html += `<span class="syn-com">${escapeHtml(comment)}</span>`;
      } else if (string) {
        html += `<span class="syn-str">${escapeHtml(string)}</span>`;
      } else if (number) {
        html += `<span class="syn-num">${escapeHtml(number)}</span>`;
      } else if (ident) {
        if (KEYWORDS.has(ident)) {
          html += `<span class="syn-kw">${escapeHtml(ident)}</span>`;
        } else if (TYPES.has(ident)) {
          html += `<span class="syn-typ">${escapeHtml(ident)}</span>`;
        } else {
          html += escapeHtml(ident);
        }
      } else {
        html += escapeHtml(full);
      }
    }

    return html;
  }

  function initSyntaxHighlighting() {
    const codeBlocks = document.querySelectorAll('pre.code-block code');
    codeBlocks.forEach(function (codeEl) {
      const rawText = codeEl.textContent;
      codeEl.innerHTML = highlightCode(rawText);
    });
  }

  // =========================================================================
  // 3. Code Copy Buttons (with Fallback)
  // =========================================================================
  function initCopyButtons() {
    const copyBtns = document.querySelectorAll('.copy-btn');
    copyBtns.forEach(function (btn) {
      btn.addEventListener('click', function () {
        const wrapper = btn.closest('.code-wrapper');
        if (!wrapper) return;
        const codeEl = wrapper.querySelector('pre.code-block code');
        if (!codeEl) return;

        const codeToCopy = codeEl.textContent;
        copyText(codeToCopy, function () {
          const originalText = btn.textContent;
          btn.textContent = 'Copied!';
          btn.classList.add('copied');
          setTimeout(function () {
            btn.textContent = originalText;
            btn.classList.remove('copied');
          }, 2000);
        });
      });
    });
  }

  function copyText(text, callback) {
    if (navigator.clipboard && window.isSecureContext) {
      navigator.clipboard.writeText(text).then(callback).catch(function () {
        fallbackCopyText(text, callback);
      });
    } else {
      fallbackCopyText(text, callback);
    }
  }

  function fallbackCopyText(text, callback) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.top = '-9999px';
    textArea.style.left = '-9999px';
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    try {
      document.execCommand('copy');
      if (callback) callback();
    } catch (err) {
      console.error('Copy fallback failed:', err);
    }
    document.body.removeChild(textArea);
  }

  // =========================================================================
  // 4. Live Search & Keyboard Shortcuts
  // =========================================================================
  function initSearch() {
    const searchInput = document.getElementById('search-input');
    const overlay = document.getElementById('search-overlay');
    if (!searchInput || !overlay) return;

    let debounceTimer = null;
    let selectedIndex = -1;

    // Keyboard shortcut: '/' focuses search input; 'Escape' dismisses
    window.addEventListener('keydown', function (e) {
      if (e.key === '/' && document.activeElement !== searchInput) {
        if (!['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName)) {
          e.preventDefault();
          searchInput.focus();
          searchInput.select();
        }
      } else if (e.key === 'Escape') {
        overlay.classList.remove('open');
        overlay.innerHTML = '';
        searchInput.blur();
      }
    });

    // Arrow navigation in dropdown
    searchInput.addEventListener('keydown', function (e) {
      const items = overlay.querySelectorAll('.search-result-item');
      if (!overlay.classList.contains('open') || items.length === 0) return;

      if (e.key === 'ArrowDown') {
        e.preventDefault();
        selectedIndex = (selectedIndex + 1) % items.length;
        updateSelection(items, selectedIndex);
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        selectedIndex = (selectedIndex - 1 + items.length) % items.length;
        updateSelection(items, selectedIndex);
      } else if (e.key === 'Enter') {
        if (selectedIndex >= 0 && selectedIndex < items.length) {
          e.preventDefault();
          items[selectedIndex].click();
        }
      }
    });

    function updateSelection(items, idx) {
      items.forEach((item, i) => {
        if (i === idx) {
          item.classList.add('selected');
          item.scrollIntoView({ block: 'nearest' });
        } else {
          item.classList.remove('selected');
        }
      });
    }

    searchInput.addEventListener('input', function () {
      clearTimeout(debounceTimer);
      const query = searchInput.value.trim();
      if (!query) {
        overlay.classList.remove('open');
        overlay.innerHTML = '';
        selectedIndex = -1;
        return;
      }

      debounceTimer = setTimeout(function () {
        fetchSearch(query);
      }, 150);
    });

    // Close when clicking outside
    document.addEventListener('click', function (e) {
      if (!searchInput.contains(e.target) && !overlay.contains(e.target)) {
        overlay.classList.remove('open');
      }
    });

    function fetchSearch(query) {
      const activeLang = searchInput.getAttribute('data-lang') || '';
      let url = `/api/search?q=${encodeURIComponent(query)}`;
      if (activeLang) {
        url += `&lang=${encodeURIComponent(activeLang)}`;
      }

      fetch(url)
        .then(function (res) { return res.json(); })
        .then(function (results) {
          renderSearchResults(results, query);
        })
        .catch(function (err) {
          console.error('Search request failed:', err);
        });
    }

    function renderSearchResults(results, query) {
      selectedIndex = -1;
      if (!results || results.length === 0) {
        overlay.innerHTML = `<div style="padding: 1rem; color: var(--text-muted); font-size: 0.95rem; text-align: center;">No matching notes found for "${escapeHtml(query)}"</div>`;
        overlay.classList.add('open');
        return;
      }

      const html = results.map(function (res) {
        const langDisplay = res.lang.toUpperCase();
        const href = `/lang/${encodeURIComponent(res.lang)}#entry-${encodeURIComponent(res.entry_id)}`;
        return `
          <a class="search-result-item" href="${href}">
            <div class="search-result-header">
              <span>${escapeHtml(langDisplay)} &bull; Part ${res.part}: ${escapeHtml(res.part_title)}</span>
              <span>Score: ${res.score}</span>
            </div>
            <div class="search-result-title">${escapeHtml(res.entry_title)}</div>
            <div class="search-result-desc">${escapeHtml(res.snippet)}</div>
          </a>
        `;
      }).join('');

      overlay.innerHTML = html;
      overlay.classList.add('open');
    }
  }

  // =========================================================================
  // 5. Sticky TOC IntersectionObserver
  // =========================================================================
  function initTocObserver() {
    const tocLinks = document.querySelectorAll('.toc-link');
    const articles = document.querySelectorAll('.part-article');
    if (tocLinks.length === 0 || articles.length === 0) return;

    const observer = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (entry.isIntersecting) {
            const id = entry.target.getAttribute('id');
            tocLinks.forEach(function (link) {
              if (link.getAttribute('href') === `#${id}`) {
                link.classList.add('active');
              } else {
                link.classList.remove('active');
              }
            });
          }
        });
      },
      {
        rootMargin: '-10% 0px -70% 0px',
        threshold: 0
      }
    );

    articles.forEach(function (article) {
      observer.observe(article);
    });
  }

  // =========================================================================
  // 6. Keepalive Ping Heartbeat
  // =========================================================================
  function initKeepalive() {
    // Send a keepalive ping every 60 seconds
    setInterval(function () {
      fetch('/api/keepalive', { method: 'POST' }).catch(function () {});
    }, 60000);
  }

  // =========================================================================
  // Document Ready Initialization
  // =========================================================================
  document.addEventListener('DOMContentLoaded', function () {
    initTheme();
    initSyntaxHighlighting();
    initCopyButtons();
    initSearch();
    initTocObserver();
    initKeepalive();
  });
})();
