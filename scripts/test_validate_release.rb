# frozen_string_literal: true

require "fileutils"
require "minitest/autorun"
require "open3"
require "rbconfig"
require "tmpdir"

class ValidateReleaseTest < Minitest::Test
  VALIDATOR = File.expand_path("validate_release.rb", __dir__)

  def test_matching_tag_packages_and_changelog_pass
    with_fixture do |root|
      notes = File.join(root, "release-notes.md")
      result = run_validator(root, "v0.1.0", notes)

      assert result[:status].success?, result[:output]
      assert_equal "### Added\n\n- First public release.\n", File.read(notes, encoding: "UTF-8")
    end
  end

  def test_package_version_mismatch_fails
    with_fixture(second_version: "0.2.0") do |root|
      result = run_validator(root, "v0.1.0")

      refute result[:status].success?
      assert_includes result[:output], "package versions must all match 0.1.0"
    end
  end

  def test_missing_changelog_release_fails
    with_fixture(changelog_version: "0.2.0") do |root|
      result = run_validator(root, "v0.1.0")

      refute result[:status].success?
      assert_includes result[:output], "CHANGELOG.md has no 0.1.0 release section"
    end
  end

  def test_non_semver_tag_fails
    with_fixture do |root|
      result = run_validator(root, "release-0.1")

      refute result[:status].success?
      assert_includes result[:output], "tag must be v followed by full SemVer"
    end
  end

  private

  def with_fixture(second_version: "0.1.0", changelog_version: "0.1.0")
    Dir.mktmpdir("validate-release-") do |root|
      write(root, "Cargo.toml", <<~TOML)
        [workspace]
        members = ["crates/one", "crates/two"]
        resolver = "3"
      TOML
      write_package(root, "one", "0.1.0")
      write_package(root, "two", second_version)
      write(root, "CHANGELOG.md", <<~MARKDOWN)
        # Changelog

        ## [Unreleased]

        ### Changed

        - Nothing yet.

        ## [#{changelog_version}] - 2026-08-10

        ### Added

        - First public release.

        [Unreleased]: https://example.test/compare/v#{changelog_version}...HEAD
        [#{changelog_version}]: https://example.test/releases/tag/v#{changelog_version}
      MARKDOWN
      _stdout, stderr, status = Open3.capture3("cargo", "generate-lockfile", chdir: root)
      raise stderr unless status.success?

      yield root
    end
  end

  def write_package(root, name, version)
    write(root, "crates/#{name}/Cargo.toml", <<~TOML)
      [package]
      name = "#{name}"
      version = "#{version}"
      edition = "2024"
    TOML
    write(root, "crates/#{name}/src/lib.rs", "pub fn available() -> bool { true }\n")
  end

  def write(root, relative_path, content)
    path = File.join(root, relative_path)
    FileUtils.mkdir_p(File.dirname(path))
    File.write(path, content, encoding: "UTF-8")
  end

  def run_validator(root, tag, notes = nil)
    command = [RbConfig.ruby, VALIDATOR, "--root", root, "--tag", tag]
    command.concat(["--notes", notes]) if notes
    stdout, stderr, status = Open3.capture3(*command)
    { output: stdout + stderr, status: status }
  end
end
