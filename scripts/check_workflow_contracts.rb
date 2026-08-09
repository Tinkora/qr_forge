# frozen_string_literal: true

require "optparse"
require "yaml"

REUSABLE_WORKFLOW_COMMIT = "21145ce218263e3b30359bab0c748da4702f801b"
PAGES_MAIN_CONDITION = "github.ref == 'refs/heads/main'"
PAGES_WASM_ARTIFACT = "wasm-package-${{ github.run_id }}-${{ github.run_attempt }}"
PAGES_SOURCE_ARTIFACT = "pages-source-${{ github.run_id }}-${{ github.run_attempt }}"
RELEASE_PUBLISH_CONDITION = "github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')"
RELEASE_SOURCE_ARTIFACT = "release-source-${{ github.run_id }}-${{ github.run_attempt }}"
EXPECTED_CALLS = {
  ".github/workflows/quality.yml" => {
    "rust" => "Tinkora/.github/.github/workflows/reusable-rust-quality.yml@#{REUSABLE_WORKFLOW_COMMIT}",
    "wasm" => "Tinkora/.github/.github/workflows/reusable-wasm-quality.yml@#{REUSABLE_WORKFLOW_COMMIT}"
  },
  ".github/workflows/supply-chain.yml" => {
    "audit" => "Tinkora/.github/.github/workflows/reusable-supply-chain.yml@#{REUSABLE_WORKFLOW_COMMIT}"
  },
  ".github/workflows/pages.yml" => {
    "deploy" => "Tinkora/.github/.github/workflows/reusable-pages.yml@#{REUSABLE_WORKFLOW_COMMIT}"
  },
  ".github/workflows/release.yml" => {
    "evidence" => "Tinkora/.github/.github/workflows/reusable-release.yml@#{REUSABLE_WORKFLOW_COMMIT}"
  }
}.freeze
PINNED_ACTION = %r{\A[^/\s]+/[^/@\s]+(?:/[^@\s]+)?@[0-9a-f]{40}\z}
PIPE_TO_SHELL = /curl[^\n|]*\|\s*(?:ba)?sh\b/i

options = { root: Dir.pwd }
OptionParser.new do |parser|
  parser.on("--root PATH") { |path| options[:root] = path }
end.parse!

def string_values(value)
  case value
  when Hash
    value.values.flat_map { |child| string_values(child) }
  when Array
    value.flat_map { |child| string_values(child) }
  when String
    [value]
  else
    []
  end
end

root = File.expand_path(options[:root])
errors = []
workflow_paths = Dir.glob(File.join(root, ".github/workflows/*.{yml,yaml}"))

workflow_paths.sort.each do |workflow_path|
  relative_path = workflow_path.delete_prefix("#{root}/")
  content = File.read(workflow_path, encoding: "UTF-8", invalid: :replace, undef: :replace)
  if content.match?(PIPE_TO_SHELL)
    errors << "Unverified pipe-to-shell install is forbidden: #{relative_path}"
  end

  begin
    workflow = YAML.safe_load(content, aliases: false)
    string_values(workflow).grep(/\A[^\s]+@[^\s]+\z/).each do |reference|
      next if reference.start_with?("./")
      next if reference.match?(PINNED_ACTION)

      errors << "Action reference must use a full commit SHA: #{relative_path}: #{reference}"
    end
  rescue Psych::Exception => error
    errors << "Invalid workflow #{relative_path}: #{error.message}"
  end
end

EXPECTED_CALLS.each do |relative_path, expected_jobs|
  workflow_path = File.join(root, relative_path)
  unless File.file?(workflow_path)
    errors << "Missing workflow: #{relative_path}"
    next
  end

  begin
    workflow = YAML.safe_load_file(workflow_path, aliases: false)
    jobs = workflow.fetch("jobs")
    expected_jobs.each do |job_name, expected_reference|
      actual_reference = jobs.dig(job_name, "uses")
      next if actual_reference == expected_reference

      errors << "#{relative_path} job #{job_name} must use #{expected_reference}"
    end

    if relative_path == ".github/workflows/quality.yml"
      unless jobs.dig("wasm", "with", "playwright-smoke") == true
        errors << "#{relative_path} must enable the Playwright WASM smoke test"
      end
      unless jobs.dig("rust", "with", "msrv") == "1.85.0"
        errors << "#{relative_path} must verify the declared Rust 1.85.0 MSRV"
      end
      unless jobs.key?("package-contents")
        errors << "#{relative_path} must reject generated Cargo package content"
      end
    end

    case relative_path
    when ".github/workflows/pages.yml"
      unless %w[assemble deploy].all? { |job_name| jobs.dig(job_name, "if") == PAGES_MAIN_CONDITION }
        errors << "#{relative_path} must restrict assembly and deployment to main"
      end
      release_gates = Array(jobs.dig("assemble", "needs")).sort
      unless release_gates == %w[documentation quality supply-chain]
        errors << "#{relative_path} assembly must wait for quality, documentation, and supply-chain"
      end
      job_values = string_values(jobs)
      unless job_values.include?(PAGES_WASM_ARTIFACT) && job_values.count(PAGES_SOURCE_ARTIFACT) >= 2
        errors << "#{relative_path} artifact names must include github.run_attempt"
      end
    when ".github/workflows/release.yml"
      triggers = workflow.fetch("on")
      unless triggers.dig("push", "tags") == ["v*.*.*"] && triggers.keys == ["push"]
        errors << "#{relative_path} must trigger from v*.*.* tags only"
      end
      unless workflow["permissions"] == {"contents" => "read"}
        errors << "#{relative_path} must default to contents: read"
      end
      release_gates = Array(jobs.dig("build", "needs")).sort
      unless release_gates == %w[documentation quality supply-chain validate]
        errors << "#{relative_path} release build must wait for quality, documentation, and supply-chain"
      end
      unless jobs.dig("evidence", "with", "publish") == false
        errors << "#{relative_path} reusable release evidence must remain dry-run"
      end
      publication = jobs.fetch("release", {})
      unless publication["if"] == RELEASE_PUBLISH_CONDITION
        errors << "#{relative_path} publication job must require a pushed version tag"
      end
      unless publication["environment"] == "release"
        errors << "#{relative_path} publication job must use the release environment"
      end
      writers = jobs.filter_map do |job_name, job|
        job_name if job.dig("permissions", "contents") == "write"
      end
      unless writers == ["release"]
        errors << "#{relative_path} publication job alone must receive contents: write"
      end
      unless Array(publication["needs"]).include?("verify")
        errors << "#{relative_path} publication must wait for verified release assets"
      end
      job_values = string_values(jobs)
      unless job_values.count(RELEASE_SOURCE_ARTIFACT) >= 2
        errors << "#{relative_path} release artifact names must include github.run_attempt"
      end
    end
  rescue KeyError, Psych::Exception => error
    errors << "Invalid workflow #{relative_path}: #{error.message}"
  end
end

if errors.empty?
  puts "Workflow contracts passed (Tinkora bundle #{REUSABLE_WORKFLOW_COMMIT})."
  exit 0
end

errors.uniq.each { |error| warn error }
exit 1
