# frozen_string_literal: true

require "bundler/gem_tasks"
task default: %i[]

require "rake/extensiontask"

Rake::ExtensionTask.new("add") do |ext|
  ext.lib_dir = "lib/add"
  ext.source_pattern = "*.{rs,toml}"
end