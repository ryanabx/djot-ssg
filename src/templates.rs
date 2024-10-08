use clap::ValueEnum;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BuiltInTemplate {
    GithubMarkdown,
    ForceNone,
}

impl BuiltInTemplate {
    pub fn get_template(&self) -> String {
        match self {
            Self::ForceNone => {
                "<!-- {CONTENT} -->".to_string()
            }
            Self::GithubMarkdown => {
                r##"
<!DOCTYPE html>
<html>

<head>
    <style>
        .content-body {
            --base-size-4: 0.25rem;
            --base-size-8: 0.5rem;
            --base-size-16: 1rem;
            --base-text-weight-normal: 400;
            --base-text-weight-medium: 500;
            --base-text-weight-semibold: 600;
            --fontStack-monospace: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;

        }

        @media (prefers-color-scheme: dark) {

            .content-body,
            [data-theme="dark"] {
                /*dark*/
                color-scheme: dark;
                --focus-outlineColor: #1f6feb;
                --fgColor-default: #e6edf3;
                --fgColor-muted: #8d96a0;
                --fgColor-accent: #4493f8;
                --fgColor-success: #3fb950;
                --fgColor-attention: #d29922;
                --fgColor-danger: #f85149;
                --fgColor-done: #ab7df8;
                --bgColor-default: #0d1117;
                --bgColor-muted: #161b22;
                --bgColor-muted-2: #222222;
                --bgColor-neutral-muted: #6e768166;
                --bgColor-attention-muted: #bb800926;
                --borderColor-default: #30363d;
                --borderColor-muted: #30363db3;
                --borderColor-neutral-muted: #6e768166;
                --borderColor-accent-emphasis: #1f6feb;
                --borderColor-success-emphasis: #238636;
                --borderColor-attention-emphasis: #9e6a03;
                --borderColor-danger-emphasis: #da3633;
                --borderColor-done-emphasis: #8957e5;
                --color-prettylights-syntax-comment: #8b949e;
                --color-prettylights-syntax-constant: #79c0ff;
                --color-prettylights-syntax-constant-other-reference-link: #a5d6ff;
                --color-prettylights-syntax-entity: #d2a8ff;
                --color-prettylights-syntax-storage-modifier-import: #c9d1d9;
                --color-prettylights-syntax-entity-tag: #7ee787;
                --color-prettylights-syntax-keyword: #ff7b72;
                --color-prettylights-syntax-string: #a5d6ff;
                --color-prettylights-syntax-variable: #ffa657;
                --color-prettylights-syntax-brackethighlighter-unmatched: #f85149;
                --color-prettylights-syntax-brackethighlighter-angle: #8b949e;
                --color-prettylights-syntax-invalid-illegal-text: #f0f6fc;
                --color-prettylights-syntax-invalid-illegal-bg: #8e1519;
                --color-prettylights-syntax-carriage-return-text: #f0f6fc;
                --color-prettylights-syntax-carriage-return-bg: #b62324;
                --color-prettylights-syntax-string-regexp: #7ee787;
                --color-prettylights-syntax-markup-list: #f2cc60;
                --color-prettylights-syntax-markup-heading: #1f6feb;
                --color-prettylights-syntax-markup-italic: #c9d1d9;
                --color-prettylights-syntax-markup-bold: #c9d1d9;
                --color-prettylights-syntax-markup-deleted-text: #ffdcd7;
                --color-prettylights-syntax-markup-deleted-bg: #67060c;
                --color-prettylights-syntax-markup-inserted-text: #aff5b4;
                --color-prettylights-syntax-markup-inserted-bg: #033a16;
                --color-prettylights-syntax-markup-changed-text: #ffdfb6;
                --color-prettylights-syntax-markup-changed-bg: #5a1e02;
                --color-prettylights-syntax-markup-ignored-text: #c9d1d9;
                --color-prettylights-syntax-markup-ignored-bg: #1158c7;
                --color-prettylights-syntax-meta-diff-range: #d2a8ff;
                --color-prettylights-syntax-sublimelinter-gutter-mark: #484f58;
            }
        }

        @media (prefers-color-scheme: light) {

            .content-body,
            [data-theme="light"] {
                /*light*/
                color-scheme: light;
                --focus-outlineColor: #0969da;
                --fgColor-default: #1f2328;
                --fgColor-muted: #636c76;
                --fgColor-accent: #0969da;
                --fgColor-success: #1a7f37;
                --fgColor-attention: #9a6700;
                --fgColor-danger: #d1242f;
                --fgColor-done: #8250df;
                --bgColor-default: #ffffff;
                --bgColor-muted: #f6f8fa;
                --bgColor-muted-2: #dddddd;
                --bgColor-neutral-muted: #afb8c133;
                --bgColor-attention-muted: #fff8c5;
                --borderColor-default: #d0d7de;
                --borderColor-muted: #d0d7deb3;
                --borderColor-neutral-muted: #afb8c133;
                --borderColor-accent-emphasis: #0969da;
                --borderColor-success-emphasis: #1a7f37;
                --borderColor-attention-emphasis: #bf8700;
                --borderColor-danger-emphasis: #cf222e;
                --borderColor-done-emphasis: #8250df;
                --color-prettylights-syntax-comment: #57606a;
                --color-prettylights-syntax-constant: #0550ae;
                --color-prettylights-syntax-constant-other-reference-link: #0a3069;
                --color-prettylights-syntax-entity: #6639ba;
                --color-prettylights-syntax-storage-modifier-import: #24292f;
                --color-prettylights-syntax-entity-tag: #0550ae;
                --color-prettylights-syntax-keyword: #cf222e;
                --color-prettylights-syntax-string: #0a3069;
                --color-prettylights-syntax-variable: #953800;
                --color-prettylights-syntax-brackethighlighter-unmatched: #82071e;
                --color-prettylights-syntax-brackethighlighter-angle: #57606a;
                --color-prettylights-syntax-invalid-illegal-text: #f6f8fa;
                --color-prettylights-syntax-invalid-illegal-bg: #82071e;
                --color-prettylights-syntax-carriage-return-text: #f6f8fa;
                --color-prettylights-syntax-carriage-return-bg: #cf222e;
                --color-prettylights-syntax-string-regexp: #116329;
                --color-prettylights-syntax-markup-list: #3b2300;
                --color-prettylights-syntax-markup-heading: #0550ae;
                --color-prettylights-syntax-markup-italic: #24292f;
                --color-prettylights-syntax-markup-bold: #24292f;
                --color-prettylights-syntax-markup-deleted-text: #82071e;
                --color-prettylights-syntax-markup-deleted-bg: #ffebe9;
                --color-prettylights-syntax-markup-inserted-text: #116329;
                --color-prettylights-syntax-markup-inserted-bg: #dafbe1;
                --color-prettylights-syntax-markup-changed-text: #953800;
                --color-prettylights-syntax-markup-changed-bg: #ffd8b5;
                --color-prettylights-syntax-markup-ignored-text: #eaeef2;
                --color-prettylights-syntax-markup-ignored-bg: #0550ae;
                --color-prettylights-syntax-meta-diff-range: #8250df;
                --color-prettylights-syntax-sublimelinter-gutter-mark: #8c959f;
            }
        }

        .content-body {
            -ms-text-size-adjust: 100%;
            -webkit-text-size-adjust: 100%;
            margin: 0;
            color: var(--fgColor-default);
            background-color: var(--bgColor-default);
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
            font-size: 16px;
            line-height: 1.5;
            word-wrap: break-word;
            scroll-behavior: auto;
        }

        .content-body .octicon {
            display: inline-block;
            fill: currentColor;
            vertical-align: text-bottom;
        }

        .content-body h1:hover .anchor .octicon-link:before,
        .content-body h2:hover .anchor .octicon-link:before,
        .content-body h3:hover .anchor .octicon-link:before,
        .content-body h4:hover .anchor .octicon-link:before,
        .content-body h5:hover .anchor .octicon-link:before,
        .content-body h6:hover .anchor .octicon-link:before {
            width: 16px;
            height: 16px;
            content: ' ';
            display: inline-block;
            background-color: currentColor;
            -webkit-mask-image: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' version='1.1' aria-hidden='true'><path fill-rule='evenodd' d='M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z'></path></svg>");
            mask-image: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16' version='1.1' aria-hidden='true'><path fill-rule='evenodd' d='M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z'></path></svg>");
        }

        .content-body details,
        .content-body figcaption,
        .content-body figure {
            display: block;
        }

        .content-body summary {
            display: list-item;
        }

        .content-body [hidden] {
            display: none !important;
        }

        .content-body a {
            background-color: transparent;
            color: var(--fgColor-accent);
            text-decoration: none;
        }

        .content-body abbr[title] {
            border-bottom: none;
            -webkit-text-decoration: underline dotted;
            text-decoration: underline dotted;
        }

        .content-body b,
        .content-body strong {
            font-weight: var(--base-text-weight-semibold, 600);
        }

        .content-body dfn {
            font-style: italic;
        }

        .content-body h1 {
            margin: .67em 0;
            font-weight: var(--base-text-weight-semibold, 600);
            padding-bottom: .3em;
            font-size: 2em;
            border-bottom: 1px solid var(--borderColor-muted);
        }

        .content-body mark {
            background-color: var(--bgColor-attention-muted);
            color: var(--fgColor-default);
        }

        .content-body small {
            font-size: 90%;
        }

        .content-body sub,
        .content-body sup {
            font-size: 75%;
            line-height: 0;
            position: relative;
            vertical-align: baseline;
        }

        .content-body sub {
            bottom: -0.25em;
        }

        .content-body sup {
            top: -0.5em;
        }

        .content-body img {
            border-style: none;
            max-width: 100%;
            box-sizing: content-box;
            background-color: var(--bgColor-default);
        }

        .content-body code,
        .content-body kbd,
        .content-body pre,
        .content-body samp {
            font-family: monospace;
            font-size: 1em;
        }

        .content-body figure {
            margin: 1em 40px;
        }

        .content-body hr {
            box-sizing: content-box;
            overflow: hidden;
            background: transparent;
            border-bottom: 1px solid var(--borderColor-muted);
            height: .25em;
            padding: 0;
            margin: 24px 0;
            background-color: var(--borderColor-default);
            border: 0;
        }

        .content-body input {
            font: inherit;
            margin: 0;
            overflow: visible;
            font-family: inherit;
            font-size: inherit;
            line-height: inherit;
        }

        .content-body [type=button],
        .content-body [type=reset],
        .content-body [type=submit] {
            -webkit-appearance: button;
            appearance: button;
        }

        .content-body [type=checkbox],
        .content-body [type=radio] {
            box-sizing: border-box;
            padding: 0;
        }

        .content-body [type=number]::-webkit-inner-spin-button,
        .content-body [type=number]::-webkit-outer-spin-button {
            height: auto;
        }

        .content-body [type=search]::-webkit-search-cancel-button,
        .content-body [type=search]::-webkit-search-decoration {
            -webkit-appearance: none;
            appearance: none;
        }

        .content-body ::-webkit-input-placeholder {
            color: inherit;
            opacity: .54;
        }

        .content-body ::-webkit-file-upload-button {
            -webkit-appearance: button;
            appearance: button;
            font: inherit;
        }

        .content-body a:hover {
            text-decoration: underline;
        }

        .content-body ::placeholder {
            color: var(--fgColor-muted);
            opacity: 1;
        }

        .content-body hr::before {
            display: table;
            content: "";
        }

        .content-body hr::after {
            display: table;
            clear: both;
            content: "";
        }

        .content-body table {
            border-spacing: 0;
            border-collapse: collapse;
            display: block;
            width: max-content;
            max-width: 100%;
            overflow: auto;
        }

        .content-body td,
        .content-body th {
            padding: 0;
        }

        .content-body details summary {
            cursor: pointer;
        }

        .content-body details:not([open])>*:not(summary) {
            display: none;
        }

        .content-body a:focus,
        .content-body [role=button]:focus,
        .content-body input[type=radio]:focus,
        .content-body input[type=checkbox]:focus {
            outline: 2px solid var(--focus-outlineColor);
            outline-offset: -2px;
            box-shadow: none;
        }

        .content-body a:focus:not(:focus-visible),
        .content-body [role=button]:focus:not(:focus-visible),
        .content-body input[type=radio]:focus:not(:focus-visible),
        .content-body input[type=checkbox]:focus:not(:focus-visible) {
            outline: solid 1px transparent;
        }

        .content-body a:focus-visible,
        .content-body [role=button]:focus-visible,
        .content-body input[type=radio]:focus-visible,
        .content-body input[type=checkbox]:focus-visible {
            outline: 2px solid var(--focus-outlineColor);
            outline-offset: -2px;
            box-shadow: none;
        }

        .content-body a:not([class]):focus,
        .content-body a:not([class]):focus-visible,
        .content-body input[type=radio]:focus,
        .content-body input[type=radio]:focus-visible,
        .content-body input[type=checkbox]:focus,
        .content-body input[type=checkbox]:focus-visible {
            outline-offset: 0;
        }

        .content-body kbd {
            display: inline-block;
            padding: 3px 5px;
            font: 11px var(--fontStack-monospace, ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace);
            line-height: 10px;
            color: var(--fgColor-default);
            vertical-align: middle;
            background-color: var(--bgColor-muted);
            border: solid 1px var(--borderColor-neutral-muted);
            border-bottom-color: var(--borderColor-neutral-muted);
            border-radius: 6px;
            box-shadow: inset 0 -1px 0 var(--borderColor-neutral-muted);
        }

        .content-body h1,
        .content-body h2,
        .content-body h3,
        .content-body h4,
        .content-body h5,
        .content-body h6 {
            margin-top: 24px;
            margin-bottom: 16px;
            font-weight: var(--base-text-weight-semibold, 600);
            line-height: 1.25;
        }

        .content-body h2 {
            font-weight: var(--base-text-weight-semibold, 600);
            padding-bottom: .3em;
            font-size: 1.5em;
            border-bottom: 1px solid var(--borderColor-muted);
        }

        .content-body h3 {
            font-weight: var(--base-text-weight-semibold, 600);
            font-size: 1.25em;
        }

        .content-body h4 {
            font-weight: var(--base-text-weight-semibold, 600);
            font-size: 1em;
        }

        .content-body h5 {
            font-weight: var(--base-text-weight-semibold, 600);
            font-size: .875em;
        }

        .content-body h6 {
            font-weight: var(--base-text-weight-semibold, 600);
            font-size: .85em;
            color: var(--fgColor-muted);
        }

        .content-body p {
            margin-top: 0;
            margin-bottom: 10px;
        }

        .content-body blockquote {
            margin: 0;
            padding: 0 1em;
            color: var(--fgColor-muted);
            border-left: .25em solid var(--borderColor-default);
        }

        .content-body ul,
        .content-body ol {
            margin-top: 0;
            margin-bottom: 0;
            padding-left: 2em;
        }

        .content-body ol ol,
        .content-body ul ol {
            list-style-type: lower-roman;
        }

        .content-body ul ul ol,
        .content-body ul ol ol,
        .content-body ol ul ol,
        .content-body ol ol ol {
            list-style-type: lower-alpha;
        }

        .content-body dd {
            margin-left: 0;
        }

        .content-body tt,
        .content-body code,
        .content-body samp {
            font-family: var(--fontStack-monospace, ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace);
            font-size: 12px;
        }

        .content-body pre {
            margin-top: 0;
            margin-bottom: 0;
            font-family: var(--fontStack-monospace, ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace);
            font-size: 12px;
            word-wrap: normal;
        }

        .content-body .octicon {
            display: inline-block;
            overflow: visible !important;
            vertical-align: text-bottom;
            fill: currentColor;
        }

        .content-body input::-webkit-outer-spin-button,
        .content-body input::-webkit-inner-spin-button {
            margin: 0;
            -webkit-appearance: none;
            appearance: none;
        }

        .content-body .mr-2 {
            margin-right: var(--base-size-8, 8px) !important;
        }

        .content-body::before {
            display: table;
            content: "";
        }

        .content-body::after {
            display: table;
            clear: both;
            content: "";
        }

        .content-body>*:first-child {
            margin-top: 0 !important;
        }

        .content-body>*:last-child {
            margin-bottom: 0 !important;
        }

        .content-body a:not([href]) {
            color: inherit;
            text-decoration: none;
        }

        .content-body .absent {
            color: var(--fgColor-danger);
        }

        .content-body .anchor {
            float: left;
            padding-right: 4px;
            margin-left: -20px;
            line-height: 1;
        }

        .content-body .anchor:focus {
            outline: none;
        }

        .content-body p,
        .content-body blockquote,
        .content-body ul,
        .content-body ol,
        .content-body dl,
        .content-body table,
        .content-body pre,
        .content-body details {
            margin-top: 0;
            margin-bottom: 16px;
        }

        .content-body blockquote>:first-child {
            margin-top: 0;
        }

        .content-body blockquote>:last-child {
            margin-bottom: 0;
        }

        .content-body h1 .octicon-link,
        .content-body h2 .octicon-link,
        .content-body h3 .octicon-link,
        .content-body h4 .octicon-link,
        .content-body h5 .octicon-link,
        .content-body h6 .octicon-link {
            color: var(--fgColor-default);
            vertical-align: middle;
            visibility: hidden;
        }

        .content-body h1:hover .anchor,
        .content-body h2:hover .anchor,
        .content-body h3:hover .anchor,
        .content-body h4:hover .anchor,
        .content-body h5:hover .anchor,
        .content-body h6:hover .anchor {
            text-decoration: none;
        }

        .content-body h1:hover .anchor .octicon-link,
        .content-body h2:hover .anchor .octicon-link,
        .content-body h3:hover .anchor .octicon-link,
        .content-body h4:hover .anchor .octicon-link,
        .content-body h5:hover .anchor .octicon-link,
        .content-body h6:hover .anchor .octicon-link {
            visibility: visible;
        }

        .content-body h1 tt,
        .content-body h1 code,
        .content-body h2 tt,
        .content-body h2 code,
        .content-body h3 tt,
        .content-body h3 code,
        .content-body h4 tt,
        .content-body h4 code,
        .content-body h5 tt,
        .content-body h5 code,
        .content-body h6 tt,
        .content-body h6 code {
            padding: 0 .2em;
            font-size: inherit;
        }

        .content-body summary h1,
        .content-body summary h2,
        .content-body summary h3,
        .content-body summary h4,
        .content-body summary h5,
        .content-body summary h6 {
            display: inline-block;
        }

        .content-body summary h1 .anchor,
        .content-body summary h2 .anchor,
        .content-body summary h3 .anchor,
        .content-body summary h4 .anchor,
        .content-body summary h5 .anchor,
        .content-body summary h6 .anchor {
            margin-left: -40px;
        }

        .content-body summary h1,
        .content-body summary h2 {
            padding-bottom: 0;
            border-bottom: 0;
        }

        .content-body ul.no-list,
        .content-body ol.no-list {
            padding: 0;
            list-style-type: none;
        }

        .content-body ol[type="a s"] {
            list-style-type: lower-alpha;
        }

        .content-body ol[type="A s"] {
            list-style-type: upper-alpha;
        }

        .content-body ol[type="i s"] {
            list-style-type: lower-roman;
        }

        .content-body ol[type="I s"] {
            list-style-type: upper-roman;
        }

        .content-body ol[type="1"] {
            list-style-type: decimal;
        }

        .content-body div>ol:not([type]) {
            list-style-type: decimal;
        }

        .content-body ul ul,
        .content-body ul ol,
        .content-body ol ol,
        .content-body ol ul {
            margin-top: 0;
            margin-bottom: 0;
        }

        .content-body li>p {
            margin-top: 16px;
        }

        .content-body li+li {
            margin-top: .25em;
        }

        .content-body dl {
            padding: 0;
        }

        .content-body dl dt {
            padding: 0;
            margin-top: 16px;
            font-size: 1em;
            font-style: italic;
            font-weight: var(--base-text-weight-semibold, 600);
        }

        .content-body dl dd {
            padding: 0 16px;
            margin-bottom: 16px;
        }

        .content-body table th {
            font-weight: var(--base-text-weight-semibold, 600);
        }

        .content-body table th,
        .content-body table td {
            padding: 6px 13px;
            border: 1px solid var(--borderColor-default);
        }

        .content-body table td>:last-child {
            margin-bottom: 0;
        }

        .content-body table tr {
            background-color: var(--bgColor-default);
            border-top: 1px solid var(--borderColor-muted);
        }

        .content-body table tr:nth-child(2n) {
            background-color: var(--bgColor-muted);
        }

        .content-body table img {
            background-color: transparent;
        }

        .content-body img[align=right] {
            padding-left: 20px;
        }

        .content-body img[align=left] {
            padding-right: 20px;
        }

        .content-body .emoji {
            max-width: none;
            vertical-align: text-top;
            background-color: transparent;
        }

        .content-body span.frame {
            display: block;
            overflow: hidden;
        }

        .content-body span.frame>span {
            display: block;
            float: left;
            width: auto;
            padding: 7px;
            margin: 13px 0 0;
            overflow: hidden;
            border: 1px solid var(--borderColor-default);
        }

        .content-body span.frame span img {
            display: block;
            float: left;
        }

        .content-body span.frame span span {
            display: block;
            padding: 5px 0 0;
            clear: both;
            color: var(--fgColor-default);
        }

        .content-body span.align-center {
            display: block;
            overflow: hidden;
            clear: both;
        }

        .content-body span.align-center>span {
            display: block;
            margin: 13px auto 0;
            overflow: hidden;
            text-align: center;
        }

        .content-body span.align-center span img {
            margin: 0 auto;
            text-align: center;
        }

        .content-body span.align-right {
            display: block;
            overflow: hidden;
            clear: both;
        }

        .content-body span.align-right>span {
            display: block;
            margin: 13px 0 0;
            overflow: hidden;
            text-align: right;
        }

        .content-body span.align-right span img {
            margin: 0;
            text-align: right;
        }

        .content-body span.float-left {
            display: block;
            float: left;
            margin-right: 13px;
            overflow: hidden;
        }

        .content-body span.float-left span {
            margin: 13px 0 0;
        }

        .content-body span.float-right {
            display: block;
            float: right;
            margin-left: 13px;
            overflow: hidden;
        }

        .content-body span.float-right>span {
            display: block;
            margin: 13px auto 0;
            overflow: hidden;
            text-align: right;
        }

        .content-body code,
        .content-body tt {
            padding: .2em .4em;
            margin: 0;
            font-size: 85%;
            white-space: break-spaces;
            background-color: var(--bgColor-neutral-muted);
            border-radius: 6px;
        }

        .content-body code br,
        .content-body tt br {
            display: none;
        }

        .content-body del code {
            text-decoration: inherit;
        }

        .content-body samp {
            font-size: 85%;
        }

        .content-body pre code {
            font-size: 100%;
        }

        .content-body pre>code {
            padding: 0;
            margin: 0;
            word-break: normal;
            white-space: pre;
            background: transparent;
            border: 0;
        }

        .content-body .highlight {
            margin-bottom: 16px;
        }

        .content-body .highlight pre {
            margin-bottom: 0;
            word-break: normal;
        }

        .content-body .highlight pre,
        .content-body pre {
            padding: 16px;
            overflow: auto;
            font-size: 85%;
            line-height: 1.45;
            color: var(--fgColor-default);
            background-color: var(--bgColor-muted);
            border-radius: 6px;
        }

        .content-body pre code,
        .content-body pre tt {
            display: inline;
            max-width: auto;
            padding: 0;
            margin: 0;
            overflow: visible;
            line-height: inherit;
            word-wrap: normal;
            background-color: transparent;
            border: 0;
        }

        .content-body .csv-data td,
        .content-body .csv-data th {
            padding: 5px;
            overflow: hidden;
            font-size: 12px;
            line-height: 1;
            text-align: left;
            white-space: nowrap;
        }

        .content-body .csv-data .blob-num {
            padding: 10px 8px 9px;
            text-align: right;
            background: var(--bgColor-default);
            border: 0;
        }

        .content-body .csv-data tr {
            border-top: 0;
        }

        .content-body .csv-data th {
            font-weight: var(--base-text-weight-semibold, 600);
            background: var(--bgColor-muted);
            border-top: 0;
        }

        .content-body [data-footnote-ref]::before {
            content: "[";
        }

        .content-body [data-footnote-ref]::after {
            content: "]";
        }

        .content-body .footnotes {
            font-size: 12px;
            color: var(--fgColor-muted);
            border-top: 1px solid var(--borderColor-default);
        }

        .content-body .footnotes ol {
            padding-left: 16px;
        }

        .content-body .footnotes ol ul {
            display: inline-block;
            padding-left: 16px;
            margin-top: 16px;
        }

        .content-body .footnotes li {
            position: relative;
        }

        .content-body .footnotes li:target::before {
            position: absolute;
            top: -8px;
            right: -8px;
            bottom: -8px;
            left: -24px;
            pointer-events: none;
            content: "";
            border: 2px solid var(--borderColor-accent-emphasis);
            border-radius: 6px;
        }

        .content-body .footnotes li:target {
            color: var(--fgColor-default);
        }

        .content-body .footnotes .data-footnote-backref g-emoji {
            font-family: monospace;
        }

        .content-body .pl-c {
            color: var(--color-prettylights-syntax-comment);
        }

        .content-body .pl-c1,
        .content-body .pl-s .pl-v {
            color: var(--color-prettylights-syntax-constant);
        }

        .content-body .pl-e,
        .content-body .pl-en {
            color: var(--color-prettylights-syntax-entity);
        }

        .content-body .pl-smi,
        .content-body .pl-s .pl-s1 {
            color: var(--color-prettylights-syntax-storage-modifier-import);
        }

        .content-body .pl-ent {
            color: var(--color-prettylights-syntax-entity-tag);
        }

        .content-body .pl-k {
            color: var(--color-prettylights-syntax-keyword);
        }

        .content-body .pl-s,
        .content-body .pl-pds,
        .content-body .pl-s .pl-pse .pl-s1,
        .content-body .pl-sr,
        .content-body .pl-sr .pl-cce,
        .content-body .pl-sr .pl-sre,
        .content-body .pl-sr .pl-sra {
            color: var(--color-prettylights-syntax-string);
        }

        .content-body .pl-v,
        .content-body .pl-smw {
            color: var(--color-prettylights-syntax-variable);
        }

        .content-body .pl-bu {
            color: var(--color-prettylights-syntax-brackethighlighter-unmatched);
        }

        .content-body .pl-ii {
            color: var(--color-prettylights-syntax-invalid-illegal-text);
            background-color: var(--color-prettylights-syntax-invalid-illegal-bg);
        }

        .content-body .pl-c2 {
            color: var(--color-prettylights-syntax-carriage-return-text);
            background-color: var(--color-prettylights-syntax-carriage-return-bg);
        }

        .content-body .pl-sr .pl-cce {
            font-weight: bold;
            color: var(--color-prettylights-syntax-string-regexp);
        }

        .content-body .pl-ml {
            color: var(--color-prettylights-syntax-markup-list);
        }

        .content-body .pl-mh,
        .content-body .pl-mh .pl-en,
        .content-body .pl-ms {
            font-weight: bold;
            color: var(--color-prettylights-syntax-markup-heading);
        }

        .content-body .pl-mi {
            font-style: italic;
            color: var(--color-prettylights-syntax-markup-italic);
        }

        .content-body .pl-mb {
            font-weight: bold;
            color: var(--color-prettylights-syntax-markup-bold);
        }

        .content-body .pl-md {
            color: var(--color-prettylights-syntax-markup-deleted-text);
            background-color: var(--color-prettylights-syntax-markup-deleted-bg);
        }

        .content-body .pl-mi1 {
            color: var(--color-prettylights-syntax-markup-inserted-text);
            background-color: var(--color-prettylights-syntax-markup-inserted-bg);
        }

        .content-body .pl-mc {
            color: var(--color-prettylights-syntax-markup-changed-text);
            background-color: var(--color-prettylights-syntax-markup-changed-bg);
        }

        .content-body .pl-mi2 {
            color: var(--color-prettylights-syntax-markup-ignored-text);
            background-color: var(--color-prettylights-syntax-markup-ignored-bg);
        }

        .content-body .pl-mdr {
            font-weight: bold;
            color: var(--color-prettylights-syntax-meta-diff-range);
        }

        .content-body .pl-ba {
            color: var(--color-prettylights-syntax-brackethighlighter-angle);
        }

        .content-body .pl-sg {
            color: var(--color-prettylights-syntax-sublimelinter-gutter-mark);
        }

        .content-body .pl-corl {
            text-decoration: underline;
            color: var(--color-prettylights-syntax-constant-other-reference-link);
        }

        .content-body [role=button]:focus:not(:focus-visible),
        .content-body [role=tabpanel][tabindex="0"]:focus:not(:focus-visible),
        .content-body button:focus:not(:focus-visible),
        .content-body summary:focus:not(:focus-visible),
        .content-body a:focus:not(:focus-visible) {
            outline: none;
            box-shadow: none;
        }

        .content-body [tabindex="0"]:focus:not(:focus-visible),
        .content-body details-dialog:focus:not(:focus-visible) {
            outline: none;
        }

        .content-body g-emoji {
            display: inline-block;
            min-width: 1ch;
            font-family: "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
            font-size: 1em;
            font-style: normal !important;
            font-weight: var(--base-text-weight-normal, 400);
            line-height: 1;
            vertical-align: -0.075em;
        }

        .content-body g-emoji img {
            width: 1em;
            height: 1em;
        }

        .content-body .task-list-item {
            list-style-type: none;
        }

        .content-body .task-list-item label {
            font-weight: var(--base-text-weight-normal, 400);
        }

        .content-body .task-list-item.enabled label {
            cursor: pointer;
        }

        .content-body .task-list-item+.task-list-item {
            margin-top: var(--base-size-4);
        }

        .content-body .task-list-item .handle {
            display: none;
        }

        .content-body .task-list-item-checkbox {
            margin: 0 .2em .25em -1.4em;
            vertical-align: middle;
        }

        .content-body .contains-task-list:dir(rtl) .task-list-item-checkbox {
            margin: 0 -1.6em .25em .2em;
        }

        .content-body .contains-task-list {
            position: relative;
        }

        .content-body .contains-task-list:hover .task-list-item-convert-container,
        .content-body .contains-task-list:focus-within .task-list-item-convert-container {
            display: block;
            width: auto;
            height: 24px;
            overflow: visible;
            clip: auto;
        }

        .content-body ::-webkit-calendar-picker-indicator {
            filter: invert(50%);
        }

        .content-body .markdown-alert {
            padding: var(--base-size-8) var(--base-size-16);
            margin-bottom: var(--base-size-16);
            color: inherit;
            border-left: .25em solid var(--borderColor-default);
        }

        .content-body .markdown-alert>:first-child {
            margin-top: 0;
        }

        .content-body .markdown-alert>:last-child {
            margin-bottom: 0;
        }

        .content-body .markdown-alert .markdown-alert-title {
            display: flex;
            font-weight: var(--base-text-weight-medium, 500);
            align-items: center;
            line-height: 1;
        }

        .content-body .markdown-alert.markdown-alert-note {
            border-left-color: var(--borderColor-accent-emphasis);
        }

        .content-body .markdown-alert.markdown-alert-note .markdown-alert-title {
            color: var(--fgColor-accent);
        }

        .content-body .markdown-alert.markdown-alert-important {
            border-left-color: var(--borderColor-done-emphasis);
        }

        .content-body .markdown-alert.markdown-alert-important .markdown-alert-title {
            color: var(--fgColor-done);
        }

        .content-body .markdown-alert.markdown-alert-warning {
            border-left-color: var(--borderColor-attention-emphasis);
        }

        .content-body .markdown-alert.markdown-alert-warning .markdown-alert-title {
            color: var(--fgColor-attention);
        }

        .content-body .markdown-alert.markdown-alert-tip {
            border-left-color: var(--borderColor-success-emphasis);
        }

        .content-body .markdown-alert.markdown-alert-tip .markdown-alert-title {
            color: var(--fgColor-success);
        }

        .content-body .markdown-alert.markdown-alert-caution {
            border-left-color: var(--borderColor-danger-emphasis);
        }

        .content-body .markdown-alert.markdown-alert-caution .markdown-alert-title {
            color: var(--fgColor-danger);
        }

        .content-body>*:first-child>.heading-element:first-child {
            margin-top: 0 !important;
        }

        body {
            padding-left: 5%;
            padding-right: 5%;
            padding-top: 2%;
            padding-bottom: 2%;
        }

        .outer-content {
            background-color: var(--bgColor-muted-2);
            color: var(--fgColor-muted);
            padding: 1%;
        }

        .content-border {
            display: flex;
            flex: 1;
            outline: var(--fgColor-muted);
            outline-width: 5px;
            outline-style: solid;
        }

        .content-body {
            padding: 1%;
            flex: 1;
        }

        .table-of-contents {
            flex: 0 1 auto;
            font-size: 12px;
            padding: 1%;
        }

        .my-container {
            display: flex;
            min-height: 100vh;
        }
    </style>
</head>

<body class="outer-content content-body">
    <div class="my-container">
        <div class="table-of-contents">
            Contents:
            <!-- {TABLE_OF_CONTENTS} -->
        </div>
        <div class="content-border">
            <div class="content-body">
                <!-- {CONTENT} -->
            </div>
        </div>
    </div>
</body>
</html>
                "##.to_string()
            }
        }
    }
}
