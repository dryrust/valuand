Gem::Specification.new do |gem|
  gem.version            = File.read('VERSION').chomp
  gem.date               = File.mtime('VERSION').strftime('%Y-%m-%d')

  gem.name               = "valuand"
  gem.homepage           = "https://github.com/dryrust/valuand"
  gem.license            = "Unlicense"
  gem.summary            = "Valuand for Ruby"
  gem.description        = "The value to be carried."
  gem.metadata           = {
    'bug_tracker_uri'   => "https://github.com/dryrust/valuand/issues",
    'changelog_uri'     => "https://github.com/dryrust/valuand/blob/master/CHANGES.md",
    'documentation_uri' => "https://rubydoc.info/gems/valuand",
    'homepage_uri'      => "https://github.com/dryrust/valuand",
    'source_code_uri'   => "https://github.com/dryrust/valuand",
  }

  gem.author             = "Arto Bendiken"
  gem.email              = "arto@bendiken.net"

  gem.platform           = Gem::Platform::RUBY
  gem.files              = %w(AUTHORS CHANGES.md README.md UNLICENSE VERSION) + Dir.glob('lib/**/*.rb')
  gem.bindir             = %q(bin)
  gem.executables        = %w()

  gem.required_ruby_version = '>= 4.0'
  gem.add_development_dependency 'rspec', '~> 3.13'
  gem.add_development_dependency 'yard' , '~> 0.9'
end
