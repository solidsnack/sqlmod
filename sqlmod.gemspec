# coding: utf-8
Gem::Specification.new do |spec|
  spec.name           = "sqlmod"
  spec.version        = "0.1.0"
  spec.authors        = ["Jason Dusek"]
  spec.email          = ["jason.dusek@gmail.com"]
  spec.summary        = "Organize app queries in an annotated SQL file."
  spec.description    = "Organize app queries in an annotated SQL file."
  spec.homepage       = "https://gitlab.com/solidsnack/sqlmod"
  spec.license        = "MIT"
  spec.files          = `git ls-files -z`.split("\x0")
  spec.executables    = spec.files.grep(%r{^bin/}) { |f| File.basename(f) }
  spec.test_files     = spec.files.grep(%r{^(test|spec|features)/})
  spec.require_paths  = ["lib"]

  spec.platform       = Gem::Platform::CURRENT

  spec.add_dependency             "ffi"
  spec.add_development_dependency "rake"

  spec.extensions << "ext/extconf.rb"
end
