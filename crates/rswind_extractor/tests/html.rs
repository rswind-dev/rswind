use rswind_extractor::html::HtmlExtractor;

fn run(input: &str) -> Vec<&str> {
    HtmlExtractor::new(input).collect()
}

#[test]
fn test_normal_tag() {
    let input = r#"<a></a>"#;
    let actual = run(input);
    assert_eq!(actual.len(), 0);
}

#[test]
fn test_normal_tag_class() {
    let input = r#"<a class="flex"></a>"#;
    let actual = run(input);
    assert_eq!(actual, ["flex"]);
}

#[test]
fn test_normal_tag_multi_class() {
    let input = r#"<a class="flex" class="f"></a>"#;
    let actual = run(input);
    assert_eq!(actual, ["flex", "f"]);
}

#[test]
fn test_fragment_tag() {
    let input = r#"<></>"#;
    let actual = run(input);
    assert_eq!(actual.len(), 0);
}

#[test]
fn test_fragment_tag_combine() {
    let input = r#"<><div class="f"></div></>"#;
    let actual = run(input);
    assert_eq!(actual, ["f"]);
}

#[test]
fn test_self_close_tag() {
    let input = r#"<div />"#;
    let actual = run(input);
    assert_eq!(actual.len(), 0);
}

#[test]
fn test_script_tag() {
    let input = r#"<script>'hello'</script>"#;
    let actual = run(input);
    assert_eq!(actual, ["hello"]);
}

#[test]
fn test_script_tag_with_normal() {
    let input = r#"
    <div class="f">
    <script>'hello'</script>
    </div>
    "#;
    let actual = run(input);
    assert_eq!(actual, ["f", "hello"]);
}

#[test]
fn test_multi_tag() {
    let input = r#"<div class="f1"></div><div class="f2"></div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f1", "f2"]);
}

#[test]
fn test_nested_tag() {
    let input = r#"<div class="f1"><div class="f2"></div></div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f1", "f2"]);
}

#[test]
fn test_tag_with_content() {
    let input = r#"<div class="f1"><div class="f2">"f3" f4</div></div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f1", "f2"]);
}

#[test]
fn test_tag_with_content_2() {
    let input = r#"<div class="f1"><div class="f2">f4</div></div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f1", "f2"]);
}

// --- error case ---

#[test]
fn test_error_tag() {
    let input = r#"<div"#;
    let actual = run(input);
    assert_eq!(actual.len(), 0);
}

#[test]
fn test_error_tag_2() {
    let input = r#"<div c"#;
    let actual = run(input);
    assert_eq!(actual.len(), 0);
}

#[test]
fn test_error_tag_3() {
    assert_eq!(run(r#"<div c"#).len(), 0);
}

#[test]
fn test_error_tag_4() {
    assert_eq!(
        run(r#"<
    div
    "#)
        .len(),
        0
    );
}

#[test]
fn test_error_tag_5() {
    assert_eq!(run(r#"< div class="red" >"#).len(), 0);
}

#[test]
fn test_error_tag_6() {
    assert_eq!(run(r#"< div class="red" >"#).len(), 0);
}

// error recovery

#[test]
fn test_error_recovery() {
    let input = r#"<div class="f1"><div class="f2">"f3" f4</div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f1", "f2"]);
}

#[test]
fn test_error_recovery_2() {
    let input = r#"< div class="f1"><div class="f2">"f3" f4</div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f2"]);
}

#[test]
fn test_error_recovery_3() {
    let input = r#"< div class=><div class="f2">"f3" f4</div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f2"]);
}

#[test]
fn test_error_recovery_4() {
    let input = r#"< div class =
    ><div class="f2">"f3" f4</div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f2"]);
}

#[test]
fn test_error_recovery_5() {
    let input = r#"<div class = ><div " ' class="f2 f3">"f3" f4</div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f2 f3"]);
}

#[test]
fn test_error_recovery_6() {
    let input = r#"<div class = ><div " ' class="f2 f3"><script>"f3" f4</script></div>"#;
    let actual = run(input);
    assert_eq!(actual, ["f2 f3", "f3"]);
}
