# spec/coverage_spec.rb
SingleCov.not_covered! # not testing any code in lib/

describe "Coverage" do
  it "does not allow new untested code" do
    # option :tests to pass custom Dir.glob results
    SingleCov.assert_used
  end

  it "does not allow new untested files" do
    # option :tests and :files to pass custom Dir.glob results
    # :untested to get it passing with known untested files
    SingleCov.assert_tested
  end
end
