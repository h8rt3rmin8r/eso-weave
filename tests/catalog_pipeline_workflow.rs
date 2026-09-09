use std::fs;

#[test]
fn candidate_workflow_is_pinned_read_only_and_cannot_accept_or_release() {
    let workflow = fs::read_to_string(".github/workflows/catalog-candidate.yml").unwrap();
    assert!(workflow.contains("workflow_dispatch:"));
    assert!(workflow.contains("schedule:"));
    assert!(workflow.contains("permissions:\n  contents: read"));
    assert!(!workflow.contains("contents: write"));
    assert!(!workflow.contains("pull-requests: write"));
    assert!(!workflow.contains("actions/checkout@v"));
    assert!(!workflow.contains("actions/upload-artifact@v"));
    assert!(workflow.contains("pipeline-build"));
    assert!(workflow.contains("pipeline-verify"));
    for forbidden in ["git push", "gh pr", "gh release", "cargo release", "accept"] {
        assert!(
            !workflow.contains(forbidden),
            "{forbidden} must remain absent"
        );
    }
}
