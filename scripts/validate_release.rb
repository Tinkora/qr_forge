# frozen_string_literal: true

require "json"
require "open3"
require "optparse"

SEMVER = /(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-((?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*))?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?/.freeze

options = {root: Dir.pwd}
OptionParser.new do |parser|
  parser.on("--root PATH") { |path| options[:root] = path }
  parser.on("--tag TAG") { |tag| options[:tag] = tag }
  parser.on("--notes PATH") { |path| options[:notes] = path }
end.parse!

begin
  root = File.realpath(options.fetch(:root))
  tag = options.fetch(:tag)
  match = /\Av(#{SEMVER})\z/.match(tag)
  raise "tag must be v followed by full SemVer" unless match

  version = match[1]
  stdout, stderr, status = Open3.capture3(
    "cargo", "metadata", "--format-version", "1", "--no-deps", "--locked", chdir: root
  )
  raise "cargo metadata failed: #{stderr.strip}" unless status.success?

  metadata = JSON.parse(stdout)
  members = metadata.fetch("workspace_members")
  packages = metadata.fetch("packages").select { |package| members.include?(package.fetch("id")) }
  versions = packages.to_h { |package| [package.fetch("name"), package.fetch("version")] }
  raise "workspace contains no release packages" if versions.empty?
  unless versions.values.all? { |package_version| package_version == version }
    details = versions.sort.map { |name, package_version| "#{name}=#{package_version}" }.join(", ")
    raise "package versions must all match #{version}: #{details}"
  end

  changelog = File.read(File.join(root, "CHANGELOG.md"), encoding: "UTF-8")
  header = /^## \[#{Regexp.escape(version)}\] - \d{4}-\d{2}-\d{2}$/
  header_match = changelog.match(header)
  raise "CHANGELOG.md has no #{version} release section" unless header_match

  body_start = header_match.end(0)
  next_boundary = changelog.match(/^(?:## |\[[^\]]+\]:)/, body_start)
  notes = changelog[body_start...(next_boundary&.begin(0) || changelog.length)].strip
  raise "CHANGELOG.md #{version} release section is empty" if notes.empty? || notes == "- Nothing yet."

  if options[:notes]
    notes_path = File.expand_path(options[:notes])
    raise "release notes path must not be a symbolic link" if File.symlink?(notes_path)

    File.write(notes_path, "#{notes}\n", encoding: "UTF-8")
  end

  puts "Release #{tag} validated for #{versions.keys.sort.join(', ')}."
rescue Errno::ENOENT, JSON::ParserError, KeyError, OptionParser::ParseError, RuntimeError => error
  warn error.message
  exit 1
end
