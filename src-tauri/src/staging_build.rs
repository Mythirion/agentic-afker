//! CI workflow expectations for staging Windows artifacts.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn staging_build_uploads_staged_directory_not_precompressed_zip() {
        let workflow_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(".github")
            .join("workflows")
            .join("staging-build.yml");
        let workflow =
            fs::read_to_string(workflow_path).expect("read staging-build workflow");

        assert!(
            !workflow.contains("Compress-Archive"),
            "workflow should not pre-zip; upload-artifact already compresses downloads"
        );
        assert!(
            workflow.contains("STAGE_DIR"),
            "workflow should upload the staged directory"
        );
        assert!(
            workflow.contains("path: ${{ env.STAGE_DIR }}"),
            "artifact path should be the staged directory"
        );
    }
}
