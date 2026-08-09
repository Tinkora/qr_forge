# frozen_string_literal: true

require "open3"
require "optparse"
require "set"

REQUIRED_FILES = %w[
  README.md
  README.zh-CN.md
  LICENSE
  CONTRIBUTING.md
  CONTRIBUTING.zh-CN.md
  CODE_OF_CONDUCT.md
  CODE_OF_CONDUCT.zh-CN.md
  SECURITY.md
  SECURITY.zh-CN.md
  SUPPORT.md
  SUPPORT.zh-CN.md
  CHANGELOG.md
  MAINTAINERS.md
  Cargo.toml
  Cargo.lock
  deny.toml
  .github/CODEOWNERS
  docs/PRODUCT_SCOPE.md
  docs/PRODUCT_SCOPE.zh-CN.md
  docs/CONTRACT.md
  docs/CONTRACT.zh-CN.md
  docs/MATURITY.md
  docs/MATURITY.zh-CN.md
  docs/RELEASE_CHECKLIST.md
  docs/RELEASE_CHECKLIST.zh-CN.md
].freeze

BILINGUAL_PAIRS = [
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

TEXT_EXTENSIONS = %w[
  .css .html .js .json .lock .md .mjs .rb .rs .toml .yaml .yml
].freeze
TEXT_FILENAMES = %w[.gitignore LICENSE].freeze
SOURCE_EXTENSIONS = %w[.css .html .js .mjs .rb .rs].freeze
GENERATED_PREFIXES = %w[
  .playwright-cli/
  output/
  target/
  crates/qr_forge_web/node_modules/
  crates/qr_forge_web/static/pkg/
  crates/qr_forge_web/test-results/
].freeze
UTF8_BOM = "\xEF\xBB\xBF".b.freeze
CJK = /[\u3400-\u4dbf\u4e00-\u9fff]/
FORBIDDEN_PUBLIC_TEXT = Regexp.new(
  ["ag", "ent", "(?:[\\s_-]*)", "com", "mons"].join,
  Regexp::IGNORECASE
)
COMMENT_LINE = /^\s*(?:\/\/|\/\*|\*|#(?!\[)|<!--)/

options = { root: Dir.pwd }
OptionParser.new do |parser|
  parser.on("--root PATH") { |path| options[:root] = path }
end.parse!

root = File.expand_path(options[:root])
errors = []
stdout, stderr, status = Open3.capture3(
  "git", "-C", root, "ls-files", "--cached", "--others", "--exclude-standard", "-z"
)
unless status.success?
  warn "Unable to list repository files: #{stderr.strip}"
  exit 1
end
repository_files = stdout.split("\0").reject(&:empty?).to_set

REQUIRED_FILES.each do |path|
  unless repository_files.include?(path) && File.file?(File.join(root, path))
    errors << "Missing required file: #{path}"
  end
end

BILINGUAL_PAIRS.each do |english, chinese|
  english_path = File.join(root, english)
  chinese_path = File.join(root, chinese)
  english_exists = repository_files.include?(english) && File.file?(english_path)
  chinese_exists = repository_files.include?(chinese) && File.file?(chinese_path)
  next unless english_exists && chinese_exists

  english_text = File.read(english_path, encoding: "UTF-8", invalid: :replace, undef: :replace)
  chinese_text = File.read(chinese_path, encoding: "UTF-8", invalid: :replace, undef: :replace)
  errors << "Missing Chinese entry link in #{english}" unless english_text.include?(File.basename(chinese))
  errors << "Missing English entry link in #{chinese}" unless chinese_text.include?(File.basename(english))
end

repository_files.sort.each do |path|
  errors << "Generated artifact must not be public: #{path}" if GENERATED_PREFIXES.any? { |prefix| path.start_with?(prefix) }
  errors << "Unimplemented Agent Skill must not be public: #{path}" if path.start_with?("skills/")

  extension = File.extname(path).downcase
  next unless TEXT_EXTENSIONS.include?(extension) || TEXT_FILENAMES.include?(File.basename(path))

  absolute_path = File.join(root, path)
  next unless File.file?(absolute_path)

  bytes = File.binread(absolute_path)
  errors << "UTF-8 BOM is not allowed: #{path}" if bytes.start_with?(UTF8_BOM)
  content = bytes.force_encoding(Encoding::UTF_8)
  unless content.valid_encoding?
    errors << "Invalid UTF-8: #{path}"
    next
  end

  errors << "Legacy organization reference is forbidden: #{path}" if content.match?(FORBIDDEN_PUBLIC_TEXT)
  next unless SOURCE_EXTENSIONS.include?(extension)

  content.each_line.with_index(1) do |line, line_number|
    next unless line.match?(COMMENT_LINE) && line.match?(CJK)

    errors << "Code comment must be English: #{path}:#{line_number}"
  end
rescue SystemCallError => error
  errors << "Unable to read #{path}: #{error.message}"
end

if errors.empty?
  puts "Public tree checks passed (#{repository_files.length} files scanned)."
  exit 0
end

errors.uniq.each { |error| warn error }
exit 1
