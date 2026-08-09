# frozen_string_literal: true

require "fileutils"
require "minitest/autorun"
require "open3"
require "rbconfig"
require "tmpdir"

class CheckWorkflowContractsTest < Minitest::Test
  CHECKER = File.expand_path("check_workflow_contracts.rb", __dir__)
  WORKFLOW_COMMIT = "21145ce218263e3b30359bab0c748da4702f801b"

  def test_valid_workflows_pass
    with_fixture do |root|
      result = run_checker(root)

      assert result[:status].success?, result[:output]
      assert_includes result[:output], "Workflow contracts passed"
    end
  end

  def test_mutable_reusable_reference_fails
    with_fixture do |root|
      path = File.join(root, ".github/workflows/quality.yml")
      content = File.read(path, encoding: "UTF-8").sub("@#{WORKFLOW_COMMIT}", "@main")
      File.write(path, content, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "must use Tinkora/.github"
    end
  end

  def test_pages_without_all_release_gates_fails
    with_fixture do |root|
      path = File.join(root, ".github/workflows/pages.yml")
      content = File.read(path, encoding: "UTF-8").sub(
        "needs: [quality, documentation, supply-chain]",
        "needs: [quality]"
      )
      File.write(path, content, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "assembly must wait for quality, documentation, and supply-chain"
    end
  end

  def test_quality_without_package_boundary_fails
    with_fixture do |root|
      path = File.join(root, ".github/workflows/quality.yml")
      content = File.read(path, encoding: "UTF-8").sub(
        "  package-contents:\n    steps:\n      - run: cargo package --list\n",
        ""
      )
      File.write(path, content, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "must reject generated Cargo package content"
    end
  end

  def test_unverified_script_install_fails
    with_fixture do |root|
      path = File.join(root, ".github/workflows/ci.yml")
      File.write(path, "name: CI\non: push\njobs:\n  bad:\n    runs-on: ubuntu-latest\n    steps:\n      - run: curl https://example.test/install.sh | sh\n", encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "Unverified pipe-to-shell install is forbidden"
    end
  end

  def test_release_without_version_tag_trigger_fails
    with_fixture do |root|
      path = File.join(root, ".github/workflows/release.yml")
      content = File.read(path, encoding: "UTF-8").sub("tags: ['v*.*.*']", "tags: ['release-*']")
      File.write(path, content, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "must trigger from v*.*.* tags"
    end
  end

  def test_release_without_least_privilege_publication_fails
    with_fixture do |root|
      path = File.join(root, ".github/workflows/release.yml")
      content = File.read(path, encoding: "UTF-8")
        .sub("environment: release", "environment: staging")
        .sub("contents: write", "contents: read")
      File.write(path, content, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "publication job must use the release environment"
      assert_includes result[:output], "publication job alone must receive contents: write"
    end
  end

  def test_release_without_all_quality_gates_fails
    with_fixture do |root|
      path = File.join(root, ".github/workflows/release.yml")
      content = File.read(path, encoding: "UTF-8").sub(
        "needs: [validate, quality, documentation, supply-chain]",
        "needs: [quality]"
      )
      File.write(path, content, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "release build must wait for quality, documentation, and supply-chain"
    end
  end

  private

  def with_fixture
    Dir.mktmpdir("workflow-contracts-") do |root|
      workflows.each do |path, content|
        absolute_path = File.join(root, path)
        FileUtils.mkdir_p(File.dirname(absolute_path))
        File.write(absolute_path, content, encoding: "UTF-8")
      end
      yield root
    end
  end

  def workflows
    reference = "Tinkora/.github/.github/workflows"
    {
      ".github/workflows/ci.yml" => "name: CI\n'on': pull_request\njobs: {}\n",
      ".github/workflows/quality.yml" => <<~YAML,
        name: Quality
        'on':
          workflow_call:
        jobs:
          rust:
            uses: #{reference}/reusable-rust-quality.yml@#{WORKFLOW_COMMIT}
            with:
              msrv: 1.85.0
          wasm:
            uses: #{reference}/reusable-wasm-quality.yml@#{WORKFLOW_COMMIT}
            with:
              playwright-smoke: true
          package-contents:
            steps:
              - run: cargo package --list
      YAML
      ".github/workflows/supply-chain.yml" => <<~YAML,
        name: Supply chain
        'on':
          workflow_call:
        jobs:
          audit:
            uses: #{reference}/reusable-supply-chain.yml@#{WORKFLOW_COMMIT}
      YAML
      ".github/workflows/pages.yml" => <<~YAML,
        name: Pages
        'on': push
        jobs:
          assemble:
            needs: [quality, documentation, supply-chain]
            if: github.ref == 'refs/heads/main'
            steps:
              - env:
                  WASM_ARTIFACT: wasm-package-${{ github.run_id }}-${{ github.run_attempt }}
                  SOURCE_ARTIFACT: pages-source-${{ github.run_id }}-${{ github.run_attempt }}
                run: echo "$WASM_ARTIFACT $SOURCE_ARTIFACT"
          deploy:
            if: github.ref == 'refs/heads/main'
            uses: #{reference}/reusable-pages.yml@#{WORKFLOW_COMMIT}
            with:
              source-artifact-name: pages-source-${{ github.run_id }}-${{ github.run_attempt }}
      YAML
      ".github/workflows/release.yml" => <<~YAML
        name: Release
        'on':
          push:
            tags: ['v*.*.*']
        permissions:
          contents: read
        jobs:
          quality:
            uses: ./.github/workflows/quality.yml
          documentation:
            uses: ./.github/workflows/docs-quality.yml
          supply-chain:
            uses: ./.github/workflows/supply-chain.yml
          build:
            needs: [validate, quality, documentation, supply-chain]
            steps:
              - env:
                  SOURCE_ARTIFACT: release-source-${{ github.run_id }}-${{ github.run_attempt }}
                run: echo "$SOURCE_ARTIFACT"
          evidence:
            needs: build
            uses: #{reference}/reusable-release.yml@#{WORKFLOW_COMMIT}
            with:
              source-artifact-name: release-source-${{ github.run_id }}-${{ github.run_attempt }}
              version: 0.1.0
              publish: false
          verify:
            needs: evidence
            steps:
              - run: echo verified
          release:
            needs: verify
            if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')
            environment: release
            permissions:
              contents: write
            steps:
              - run: gh release create
      YAML
    }
  end

  def run_checker(root)
    stdout, stderr, status = Open3.capture3(RbConfig.ruby, CHECKER, "--root", root)
    { output: stdout + stderr, status: status }
  end
end
