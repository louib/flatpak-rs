pub fn remove_comments_from_json(json_content: &str) -> String {
    let mut json_content_without_comments = "".to_string();
    let mut is_in_a_comment = false;
    for manifest_line in json_content.split('\n') {
        if manifest_line.trim().starts_with("/*") && manifest_line.trim().ends_with("*/") {
            continue;
        }
        if manifest_line.trim().starts_with("/*") && !is_in_a_comment {
            is_in_a_comment = true;
            continue;
        }
        if manifest_line.trim().ends_with("*/") && is_in_a_comment {
            is_in_a_comment = false;
            continue;
        }
        if is_in_a_comment {
            continue;
        }
        // TODO should we also filter out comments at the end of the lines?
        json_content_without_comments += manifest_line;
        json_content_without_comments += "\n";
    }
    return json_content_without_comments;
}
