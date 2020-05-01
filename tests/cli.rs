use assert_cmd::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn non_existent_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("xml-requirements")?;
    cmd.arg("-c").arg("no/file/here.toml");

    cmd.assert().failure();

    Ok(())
}

#[test]
fn meets_requirements() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = NamedTempFile::new()?;
    write!(
        config,
        r#"
        [LinearLayout]
        required = [ "android:orientation" ]
        "#
    )?;

    let mut xml = NamedTempFile::new()?;
    write!(
        xml,
        r#"
        <LinearLayout
            xmlns:android="http://schemas.android.com/apk/res/android"
            android:layout_width="match_parent"
            android:layout_height="match_parent"
            android:orientation="vertical" />
        "#
    )?;

    let mut cmd = Command::cargo_bin("xml-requirements")?;
    cmd.arg("-c").arg(config.path());
    cmd.arg(xml.path());

    cmd.assert().success();

    Ok(())
}

#[test]
fn violates_requirements() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = NamedTempFile::new()?;
    write!(
        config,
        r#"
        [LinearLayout]
        required = [ "android:orientation" ]
        "#
    )?;

    let mut xml = NamedTempFile::new()?;
    write!(
        xml,
        r#"
        <LinearLayout
            xmlns:android="http://schemas.android.com/apk/res/android"
            android:layout_width="match_parent"
            android:layout_height="match_parent" />
        "#
    )?;

    let mut cmd = Command::cargo_bin("xml-requirements")?;
    cmd.arg("-c").arg(config.path());
    cmd.arg(xml.path());

    cmd.assert().failure();

    Ok(())
}
