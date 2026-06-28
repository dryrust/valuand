# This is free and unencumbered software released into the public domain.

module Valuand; end
module Valuand::VERSION; end

module Valuand::VERSION
  FILE = File.expand_path('../../../VERSION', __FILE__)
  STRING = File.read(FILE).chomp.freeze
  MAJOR, MINOR, PATCH, EXTRA = STRING.split('.').map(&:freeze)
end # Valuand::VERSION
