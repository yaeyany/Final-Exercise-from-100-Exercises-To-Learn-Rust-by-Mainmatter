// Trimmed lowercased string ──────────────────────────────────────────────────
pub fn sanitize_string(input: &str) -> String {
    input.trim().to_lowercase()
}