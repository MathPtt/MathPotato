// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><a href="versioning.html">Versioning</a></li><li class="chapter-item expanded affix "><a href="contributing.html">Contributing</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="reference/syntax/index.html"><strong aria-hidden="true">1.</strong> Syntax</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="reference/syntax/first.html"><strong aria-hidden="true">1.1.</strong> First item</a></li></ol></li><li class="chapter-item expanded "><a href="reference/control_flow/index.html"><strong aria-hidden="true">2.</strong> Control flow</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="reference/control_flow/first.html"><strong aria-hidden="true">2.1.</strong> First item</a></li></ol></li><li class="chapter-item expanded "><a href="reference/keywords/index.html"><strong aria-hidden="true">3.</strong> Keywords</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="reference/keywords/first.html"><strong aria-hidden="true">3.1.</strong> First item</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Internals</li><li class="chapter-item expanded "><a href="internals/lexer/index.html"><strong aria-hidden="true">4.</strong> Lexer</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="internals/lexer/first.html"><strong aria-hidden="true">4.1.</strong> First</a></li></ol></li><li class="chapter-item expanded "><a href="internals/parser/index.html"><strong aria-hidden="true">5.</strong> Parser</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="internals/parser/first.html"><strong aria-hidden="true">5.1.</strong> First</a></li></ol></li><li class="chapter-item expanded "><a href="internals/interpreter/index.html"><strong aria-hidden="true">6.</strong> Interpreter</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="internals/interpreter/first.html"><strong aria-hidden="true">6.1.</strong> First</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Tools</li><li class="chapter-item expanded "><a href="tools/lsp/index.html"><strong aria-hidden="true">7.</strong> LSP</a></li><li class="chapter-item expanded "><a href="tools/formatter/index.html"><strong aria-hidden="true">8.</strong> Formatter</a></li><li class="chapter-item expanded affix "><li class="part-title">Development Diary</li><li class="chapter-item expanded "><a href="dev_diary/first.html"><strong aria-hidden="true">9.</strong> First item</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
