use super::*;

#[test]
fn test_minify_html_removes_whitespace() {
    let html = "<html>  <body>   <p>  hello  </p>  </body>  </html>";
    let result = minify_html_str(html);
    assert!(result.len() < html.len());
    assert!(result.contains("hello"));
}

#[test]
fn test_minify_html_empty_string() {
    let result = minify_html_str("");
    assert!(result.is_empty());
}

#[test]
fn test_minify_html_preserves_content() {
    let html = "<p>Important content</p>";
    let result = minify_html_str(html);
    assert!(result.contains("Important content"));
}

#[test]
fn test_minify_html_handles_css() {
    let html = "<style>  body {  color: red;  }  </style>";
    let result = minify_html_str(html);
    assert!(result.contains("color"));
}

#[test]
fn test_minify_html_handles_js() {
    let html = "<script>  var x = 1;  </script>";
    let result = minify_html_str(html);
    assert!(result.contains("x"));
}
