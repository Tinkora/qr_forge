# frozen_string_literal: true

require "fileutils"
require "minitest/autorun"
require "open3"
require "rbconfig"
require "tmpdir"

class CheckPublicTreeTest < Minitest::Test
  CHECKER = File.expand_path("check_public_tree.rb", __dir__)
  PAIRS = [
    %w[README.md README.zh-CN.md],
    %w[CONTRIBUTING.md CONTRIBUTING.zh-CN.md],
    %w[CODE_OF_CONDUCT.md CODE_OF_CONDUCT.zh-CN.md],
    %w[SECURITY.md SECURITY.zh-CN.md],
    %w[SUPPORT.md SUPPORT.zh-CN.md],
    %w[docs/PRODUCT_SCOPE.md docs/PRODUCT_SCOPE.zh-CN.md],
    %w[docs/CONTRACT.md docs/CONTRACT.zh-CN.md],
    %w[docs/MATURITY.md docs/MATURITY.zh-CN.md],
    %w[docs/RELEASE_CHECKLIST.md docs/RELEASE_CHECKLIST.zh-CN.md]
  ].freeze

  def test_valid_repository_passes
    with_fixture do |root|
      result = run_checker(root)

      assert result[:status].success?, result[:output]
      assert_includes result[:output], "Public tree checks passed"
    end
  end

  def test_missing_required_file_fails
    with_fixture(remove: ["MAINTAINERS.md"]) do |root|
      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "Missing required file: MAINTAINERS.md"
    end
  end

  def test_missing_language_entry_link_fails
    with_fixture(overrides: { "README.md" => "# Project\n" }) do |root|
      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "Missing Chinese entry link in README.md"
    end
  end

  def test_legacy_organization_reference_fails
    legacy_name = %w[Ag ent].join + %w[Com mons].join
    with_fixture(overrides: { "CHANGELOG.md" => "# #{legacy_name}\n" }) do |root|
      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "Legacy organization reference is forbidden"
    end
  end

  def test_unimplemented_skill_fails
    with_fixture(overrides: { "skills/tool.json" => "{}\n" }) do |root|
      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "Unimplemented Agent Skill must not be public"
    end
  end

  def test_generated_artifact_fails
    with_fixture(overrides: { "crates/qr_forge_web/static/pkg/generated.js" => "export {};\n" }) do |root|
      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "Generated artifact must not be public"
    end
  end

  def test_non_english_code_comment_fails
    with_fixture(overrides: { "src/lib.rs" => "// \u68c0\u67e5\u8f93\u5165\nfn main() {}\n" }) do |root|
      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "Code comment must be English: src/lib.rs:1"
    end
  end

  def test_utf8_bom_and_invalid_utf8_fail
    with_fixture(
      overrides: {
        "CHANGELOG.md" => "\xEF\xBB\xBF# Changelog\n".b,
        "src/lib.rs" => "fn main() {}\n\xFF".b
      }
    ) do |root|
      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "UTF-8 BOM is not allowed: CHANGELOG.md"
      assert_includes result[:output], "Invalid UTF-8: src/lib.rs"
    end
  end

  private

  def with_fixture(remove: [], overrides: {})
    Dir.mktmpdir("check-public-tree-") do |root|
      files = fixture_files.merge(overrides)
      remove.each { |path| files.delete(path) }
      files.each do |path, content|
        absolute_path = File.join(root, path)
        FileUtils.mkdir_p(File.dirname(absolute_path))
        File.binwrite(absolute_path, content)
      end
      run_git(root, "init", "--quiet")
      run_git(root, "add", "--all")
      yield root
    end
  end

  def fixture_files
    files = {
      "LICENSE" => "MIT License\n",
      "CHANGELOG.md" => "# Changelog\n",
      "Cargo.toml" => "[workspace]\n",
      "Cargo.lock" => "version = 4\n",
      "deny.toml" => "[advisories]\nversion = 2\n",
      "MAINTAINERS.md" => "# Maintainers\n",
      ".github/CODEOWNERS" => "* @maintainer\n"
    }
    PAIRS.each do |english, chinese|
      files[english] = "# English\n\n[Chinese](#{File.basename(chinese)})\n"
      files[chinese] = "# Chinese\n\n[English](#{File.basename(english)})\n"
    end
    files
  end

  def run_checker(root)
    stdout, stderr, status = Open3.capture3(RbConfig.ruby, CHECKER, "--root", root)
    { output: stdout + stderr, status: status }
  end

  def run_git(root, *arguments)
    _stdout, stderr, status = Open3.capture3("git", "-C", root, *arguments)
    raise stderr unless status.success?
  end
end
