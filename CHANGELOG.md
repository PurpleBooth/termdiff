# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v4.1.0 - 2025-05-05
#### Bug Fixes
- update test comment and mark failing test for `diff_with_algorithm` - (48c3b28) - Billie Thompson
- remove whitespace in diff test assertions - (612e761) - Billie Thompson
- update Myers algorithm tests to match ArrowsTheme formatting - (4488f9c) - Billie Thompson (aider)
- Adjust Myers diff algorithm spacing to match Similar algorithm - (d0ba842) - Billie Thompson (aider)
- normalize diff algorithm test comparisons - (868720b) - Billie Thompson (aider)
- remove space after diff theme arrow prefixes in tests - (6cc1529) - Billie Thompson
- Refactor Myers diff algorithm inline changes implementation - (c912e5b) - Billie Thompson (aider)
- resolve borrowing issues in Myers diff algorithm - (9c6b663) - Billie Thompson (aider)
- improve newline handling in Myers diff algorithm - (7270110) - Billie Thompson (aider)
- resolve borrow checker issues in Myers diff algorithm - (db16d3e) - Billie Thompson (aider)
- correct Myers diff algorithm equal operation index handling - (4744bee) - Billie Thompson
- update similar::DiffOp::Equal to use correct field names - (ef1b3bc) - Billie Thompson (aider)
- Align Myers diff algorithm with Similar algorithm's behavior - (41735c9) - Billie Thompson (aider)
- Correct indentation in myers.rs to fix brace mismatch - (81d6662) - Billie Thompson (aider)
- Remove duplicate closing braces in MyersDiff implementation - (74851e9) - Billie Thompson (aider)
- correct unbalanced braces in myers diff implementation - (4ca95c0) - Billie Thompson (aider)
- Correct mismatched braces in MyersDiff implementation - (76a84c6) - Billie Thompson (aider)
- Correct control flow and initialize Change struct in Myers diff - (36a8784) - Billie Thompson (aider)
- update test assertions for arrow theme spacing - (c8d1f64) - Billie Thompson
- Correct test assertions and remove AI comments - (430f6eb) - Billie Thompson (aider)
- Correct test assertions to match arrow theme spacing - (7306ca0) - Billie Thompson (aider)
- Correct spacing in diff output assertions - (42ebac3) - Billie Thompson (aider)
- Correct test assertions for ArrowsTheme prefixes and spacing - (f88927d) - Billie Thompson (aider)
- Correct spacing in HTML tag assertion test - (7f95e5c) - Billie Thompson (aider)
- Update test assertions to match ArrowsTheme prefix spacing - (0cd807e) - Billie Thompson (aider)
- Correct test assertions and remove AI comments - (652c03d) - Billie Thompson (aider)
- correct test assertions and remove AI comments - (71e7f02) - Billie Thompson (aider)
- comment out broken test assertions in arrows theme tests - (059d5b9) - Billie Thompson
- Correct and enhance test assertions in draw_diff.rs - (941f5e2) - Billie Thompson (aider)
- Add space after arrow prefixes in ArrowsTheme - (8d32c71) - Billie Thompson (aider)
- Remove leading space from Myers algorithm output to match expected test values - (9093fad) - Billie Thompson (aider)
- add spacing after diff markers in Myers algorithm output - (f8c74a1) - Billie Thompson (aider)
- Align Myers diff tests with line-level diff expectations - (a5dba35) - Billie Thompson (aider)
#### Documentation
- Add project conventions and update diff rendering style - (b1fbf27) - Billie Thompson
#### Features
- add default target and update mutate command with all-features flag - (602d9c9) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin dependencies - (3121db6) - Solace System Renovate Fox
- add mutants.toml configuration for test mutation exclusions - (9f0a43e) - Billie Thompson
- update Justfile to ignore specific mutation tests - (3c19012) - Billie Thompson (aider)
- update mutate command with specific mutation test ignores - (0100066) - Billie Thompson (aider)
- Add comment to Myers diff algorithm module - (d3f846d) - Billie Thompson
- Add TODO comments to broken test cases - (da62f36) - Billie Thompson
- Add TODO comments to broken test cases in draw_diff.rs - (c97bb8b) - Billie Thompson
- Add placeholder comments to broken test cases - (7f52fb5) - Billie Thompson
- Remove AI comment from myers.rs - (035ec97) - Billie Thompson (aider)
- Add cargo mutest comment to myers diff algorithm - (8772486) - Billie Thompson
#### Refactoring
- remove explicit ignore flags from mutate command in Justfile - (8cb156b) - Billie Thompson (aider)
- remove specific mutant test ignores in cargo mutants command - (ae3441b) - Billie Thompson
- simplify boolean comparison and improve test readability - (622efc8) - Billie Thompson
- Improve mutation resilience in algorithm availability checks - (d493fe6) - Billie Thompson (aider)
- remove unused Myers diff implementation code - (71c2cce) - Billie Thompson (aider)
- use Rust 1.58+ string formatting with `{line}` syntax - (4082bf0) - Billie Thompson
- remove redundant test for algorithm availability - (1ea7c39) - Billie Thompson (aider)
- remove unnecessary test case for unavailable algorithm - (f4a1994) - Billie Thompson
- Remove normalization function and directly compare diff outputs - (9bcfb91) - Billie Thompson (aider)
- simplify normalize_diff_output function and remove commented code - (92fbf8f) - Billie Thompson
- simplify Myers diff algorithm implementation and improve code style - (860170e) - Billie Thompson
- simplify Myers diff algorithm inline changes iteration logic - (f5831cc) - Billie Thompson
- Improve Myers diff algorithm implementation and test coverage - (5d30205) - Billie Thompson
- improve code formatting for Myers diff algorithm - (84b74e3) - Billie Thompson
- improve code formatting and readability in Myers diff algorithm - (1f036ab) - Billie Thompson
- Fix type mismatches and improve Myers diff algorithm implementation - (5bdff99) - Billie Thompson (aider)
- Completely rewrite Myers diff algorithm implementation - (d49c91f) - Billie Thompson (aider)
- rewrite Myers diff algorithm with O(ND) implementation - (fd85ba7) - Billie Thompson (aider)
- fix indentation in MyersDiff implementation - (87041bf) - Billie Thompson
- Improve diff operation merging and inline change handling - (1f8c2bf) - Billie Thompson (aider)
- Adjust spacing in Myers diff test assertions - (9292360) - Billie Thompson
- Remove comment and update error message in diff test - (a12f82e) - Billie Thompson (aider)
- improve test assertions with better error messages - (e636edc) - Billie Thompson (aider)
- improve test assertions with descriptive error messages - (b04ecb6) - Billie Thompson (aider)
- remove spaces after diff markers in test assertions - (d54106e) - Billie Thompson
- Remove spaces from diff prefixes in ArrowsTheme - (6f7d668) - Billie Thompson
- enhance test_myers_iter_inline_changes with custom theme - (5e7cf2a) - Billie Thompson (aider)
- simplify expected diff output in Myers test - (b7616e3) - Billie Thompson (aider)
- improve test assertions with exact output matching in Myers algorithm - (88823b4) - Billie Thompson (aider)
- Add comment for mutation testing in Myers algorithm - (9008f85) - Billie Thompson
- improve Myers diff test assertions and remove AI comments - (57bba93) - Billie Thompson (aider)
- add comment to highlight escaped mutants in Myers algorithm - (a500a94) - Billie Thompson
#### Style
- Format test assertions for better readability - (599a302) - Billie Thompson
- remove trailing whitespaces in Myers diff algorithm - (be02f22) - Billie Thompson (aider)
- Remove trailing whitespace and improve line formatting in Myers diff algorithm - (3282f0d) - Billie Thompson (aider)
- Fix indentation and formatting in MyersDiff implementation - (58a877c) - Billie Thompson
- Fix indentation in MyersDiff implementation - (d7cea73) - Billie Thompson (aider)
- Remove spaces after diff prefixes in test assertions - (63da87a) - Billie Thompson
#### Tests
- add test for `has_available_algorithms` with no features - (c8c02a4) - Billie Thompson (aider)
- improve test for diff algorithm availability condition - (46e7423) - Billie Thompson (aider)
- add tests for diff algorithm availability and error handling - (f1db7c3) - Billie Thompson (aider)
- add tests to verify algorithm availability and diff behavior - (2252fe8) - Billie Thompson (aider)
- add test case for Myers algorithm with added lines - (bde3b1c) - Billie Thompson
- Remove spaces in diff assertion strings - (2168a7b) - Billie Thompson
- Add comments to broken test cases in diff algorithm tests - (862a5ee) - Billie Thompson
- add comment to broken test in Myers algorithm - (ba8e717) - Billie Thompson

- - -

## v4.0.0 - 2025-05-04
#### Documentation
- remove PERFORMANCE_IMPROVEMENTS.md documentation - (265f433) - Billie Thompson
#### Refactoring
- **(src)** move tests to dedicated directory and add benchmarking - (9b040b1) - Billie Thompson
#### Tests
- add some cases found by mutation testing - (320eef3) - Billie Thompson

- - -

## v3.1.6 - 2025-04-24
#### Performance
- Significantly improved Myers algorithm performance for large inputs (up to 37% faster) - See PERFORMANCE_IMPROVEMENTS.md for details

- - -
## v3.1.5 - 2025-04-23
#### Bug Fixes
- **(deps)** update rust crate crossterm to 0.28.0, ||, ^0 - (6c289dd) - Solace System Renovate Fox
#### Continuous Integration
- Switch workflow runners from "docker" to "runner-latest" - (5169558) - Billie Thompson
- run on ubuntu-latest - (aba3139) - PurpleBooth
- Set rangeStrategy to widen for dependencies in Renovate - (92d643d) - Billie Thompson
- Remove GitHub-specific CI/CD configurations and migrate to Forgejo - (305262a) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update https://code.forgejo.org/actions/cache digest to 5a3ec84 - (3aa1647) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/actions/cache digest to d4323d4 - (e9c7bbe) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/actions/cache digest to 0c907a7 - (fee1679) - Solace System Renovate Fox
- Remove rustfmt configuration file - (1f94db7) - Billie Thompson
#### Refactoring
- Update Clippy lint configuration for better clarity - (7378501) - Billie Thompson
- Enforce stricter Clippy lints and allow specific exceptions. - (0672c30) - Billie Thompson
- Fix redundant return and ensure consistent newline handling. - (a6a7d8b) - Billie Thompson

- - -

## v3.1.4 - 2024-08-19
#### Bug Fixes
- **(deps)** update rust crate crossterm to 0.28.0 - (515dc5c) - renovate[bot]
#### Continuous Integration
- **(Mergify)** configuration update - (6770a6c) - Billie Thompson
- Use binstall and cog for releases - (15d0bcc) - Billie Thompson
#### Miscellaneous Chores
- Add renovate.json - (fba500f) - renovate[bot]

- - -

## v3.1.3 - 2024-07-27
#### Bug Fixes
- Bump versions - (878ba7f) - Billie Thompson
#### Continuous Integration
- **(deps)** Bump PurpleBooth/versio-release-action from 0.1.13 to 0.1.18 - (16f5b69) - dependabot[bot]
- Correct typo in ref - (7a10edc) - Billie Thompson
- Switch to cocogitto - (6481874) - Billie Thompson

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
