//! # Input Validation
//!
//! This module provides comprehensive validation for all user inputs,
//! ensuring data quality and security for our GraphQL API.

use crate::errors::{AppError, AppResult};
use uuid::Uuid;

/// Validation constraints
pub struct ValidationRules;

impl ValidationRules {
    pub const TITLE_MIN_LENGTH: usize = 1;
    pub const TITLE_MAX_LENGTH: usize = 200;
    // TODO: Consider adding content max length limit later for production use
    // Examples of limits to consider:
    // - 50KB for regular notes
    // - 100KB for detailed documentation
    // - 1MB for articles/long-form content
    // - Database field size limits (TEXT vs LONGTEXT in MySQL, etc.)
    // - Memory usage considerations for large content processing
    // - Network transfer optimization
}

/// Validate note title
pub fn validate_title(title: &str) -> AppResult<()> {
    let trimmed = title.trim();

    if trimmed.is_empty() {
        return Err(AppError::InvalidTitle {
            message: "Title cannot be empty or contain only whitespace".to_string(),
        });
    }

    if trimmed.len() < ValidationRules::TITLE_MIN_LENGTH {
        return Err(AppError::InvalidTitle {
            message: format!(
                "Title must be at least {} character(s), got {}",
                ValidationRules::TITLE_MIN_LENGTH,
                trimmed.len()
            ),
        });
    }

    if trimmed.len() > ValidationRules::TITLE_MAX_LENGTH {
        return Err(AppError::InvalidTitle {
            message: format!(
                "Title must be at most {} characters, got {}",
                ValidationRules::TITLE_MAX_LENGTH,
                trimmed.len()
            ),
        });
    }

    Ok(())
}

/// Validate note content (minimal validation with TODO for max length)
pub fn validate_content(content: &str) -> AppResult<()> {
    let trimmed = content.trim();

    if trimmed.is_empty() {
        return Err(AppError::InvalidContent {
            message: "Content cannot be empty or contain only whitespace".to_string(),
        });
    }

    // TODO: Consider adding content max length limit later for production use
    // Examples of limits to consider:
    // - 50KB for regular notes
    // - 100KB for detailed documentation
    // - 1MB for articles/long-form content
    // - Database field size limits (TEXT vs LONGTEXT in MySQL, etc.)
    // - Memory usage considerations for large content processing
    // - Network transfer optimization

    // Uncomment when ready to add limits:
    // if trimmed.len() > CONTENT_MAX_LENGTH {
    //     return Err(AppError::InvalidContent {
    //         message: format!(
    //             "Content must be at most {} characters, got {}",
    //             CONTENT_MAX_LENGTH,
    //             trimmed.len()
    //         ),
    //     });
    // }

    Ok(())
}

/// Validate UUID format
pub fn validate_uuid(uuid_str: &str) -> AppResult<Uuid> {
    match Uuid::parse_str(uuid_str) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(AppError::InvalidUuid {
            uuid: uuid_str.to_string(),
        }),
    }
}

/// Extract title from content while preserving the complete content
pub fn extract_title_from_content(content: &str) -> String {
    let content = content.trim();

    // Strategy 1: Split by first period followed by space
    if let Some(period_pos) = content.find(". ") {
        let title = content[..period_pos].trim();
        if !title.is_empty() && title.len() <= ValidationRules::TITLE_MAX_LENGTH {
            return title.to_string();
        }
    }

    // Strategy 2: Split by first exclamation mark followed by space
    if let Some(excl_pos) = content.find("! ") {
        let title = content[..excl_pos + 1].trim(); // Include the !
        if !title.is_empty() && title.len() <= ValidationRules::TITLE_MAX_LENGTH {
            return title.to_string();
        }
    }

    // Strategy 3: Split by first question mark followed by space
    if let Some(quest_pos) = content.find("? ") {
        let title = content[..quest_pos + 1].trim(); // Include the ?
        if !title.is_empty() && title.len() <= ValidationRules::TITLE_MAX_LENGTH {
            return title.to_string();
        }
    }

    // Strategy 4: First line (split by newline)
    if let Some(newline_pos) = content.find('\n') {
        let title = content[..newline_pos].trim();
        if !title.is_empty() && title.len() <= ValidationRules::TITLE_MAX_LENGTH {
            return title.to_string();
        }
    }

    // Strategy 5: Truncate long content intelligently
    if content.len() > ValidationRules::TITLE_MAX_LENGTH {
        let truncate_at = ValidationRules::TITLE_MAX_LENGTH.min(50); // Max 50 chars for auto-title
        let title_candidate = &content[..truncate_at];

        // Try to break at word boundary
        if let Some(space_pos) = title_candidate.rfind(' ') {
            let clean_title = title_candidate[..space_pos].trim();
            return format!("{}...", clean_title);
        }

        return format!("{}...", title_candidate.trim());
    }

    // Strategy 6: Use entire content as title (for short content)
    content.to_string()
}

/// Validate and process create note input with smart title extraction (content preserved)
pub fn validate_and_process_create_input(
    title: Option<&str>,
    content: &str,
) -> AppResult<(String, String)> {
    // First validate the content (minimal validation now)
    validate_content(content)?;

    let final_title = match title {
        Some(provided_title) => {
            // User provided title - validate and use it
            validate_title(provided_title)?;
            provided_title.trim().to_string()
        }
        None => {
            // Auto-generate title from content
            let extracted_title = extract_title_from_content(content);

            // Validate extracted title
            validate_title(&extracted_title)?;

            extracted_title
        }
    };

    // Content is ALWAYS preserved as-is (just trimmed)
    let final_content = content.trim().to_string();

    Ok((final_title, final_content))
}

/// Validate update note input (optional fields)
pub fn validate_update_input(title: Option<&str>, content: Option<&str>) -> AppResult<()> {
    if let Some(title) = title {
        validate_title(title)?;
    }

    if let Some(content) = content {
        validate_content(content)?; // Minimal validation for now
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_extraction_preserves_content() {
        let content = "Great day today. Had lots of fun coding with unlimited content length!";
        let title = extract_title_from_content(content);
        assert_eq!(title, "Great day today");

        // Test the full processing
        let (final_title, final_content) =
            validate_and_process_create_input(None, content).unwrap();
        assert_eq!(final_title, "Great day today");
        assert_eq!(final_content, content); // PRESERVED COMPLETELY!
    }

    #[test]
    fn test_exclamation_title_extraction() {
        let content = "Wow! This is amazing stuff here.";
        let title = extract_title_from_content(content);
        assert_eq!(title, "Wow!");

        let (final_title, final_content) =
            validate_and_process_create_input(None, content).unwrap();
        assert_eq!(final_title, "Wow!");
        assert_eq!(final_content, content); // COMPLETE CONTENT!
    }

    #[test]
    fn test_question_title_extraction() {
        let content = "How does this work? Let me explain the details.";
        let title = extract_title_from_content(content);
        assert_eq!(title, "How does this work?");

        let (final_title, final_content) =
            validate_and_process_create_input(None, content).unwrap();
        assert_eq!(final_title, "How does this work?");
        assert_eq!(final_content, content); // PRESERVED!
    }

    #[test]
    fn test_newline_title_extraction() {
        let content = "My Note Title\nThis is the content body with more details.";
        let title = extract_title_from_content(content);
        assert_eq!(title, "My Note Title");

        let (final_title, final_content) =
            validate_and_process_create_input(None, content).unwrap();
        assert_eq!(final_title, "My Note Title");
        assert_eq!(final_content, content); // COMPLETE!
    }

    #[test]
    fn test_manual_title_override() {
        let content = "Auto title here. But user wants custom title.";
        let manual_title = "Custom User Title";

        let (final_title, final_content) =
            validate_and_process_create_input(Some(manual_title), content).unwrap();
        assert_eq!(final_title, "Custom User Title"); // User's choice
        assert_eq!(final_content, content); // Content unchanged
    }

    #[test]
    fn test_long_content_no_limit() {
        let very_long_content = "x".repeat(10000); // 10KB content
        assert!(validate_content(&very_long_content).is_ok()); // Should pass with no length limit
    }

    #[test]
    fn test_empty_content_fails() {
        assert!(validate_content("").is_err());
        assert!(validate_content("   ").is_err()); // Only whitespace
    }

    #[test]
    fn test_title_still_has_limits() {
        let long_title = "x".repeat(250);
        assert!(validate_title(&long_title).is_err()); // Title limits still apply
    }

    #[test]
    fn test_long_content_truncation() {
        let content = "This is a very long title that exceeds normal limits and should be truncated intelligently at word boundaries for better readability and user experience.";
        let title = extract_title_from_content(content);
        assert!(title.len() <= 50);
        assert!(title.ends_with("..."));

        let (final_title, final_content) =
            validate_and_process_create_input(None, content).unwrap();
        assert_eq!(final_content, content); // Content always preserved completely!
    }
}
