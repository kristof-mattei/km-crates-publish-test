## [unreleased]

### 🚀 Features

- Ensure formatting works
- Attest individual and multiplatform images
- Restrict ALL
- I rule(set)
- Use anchors to dedup build
- Pin to trixie, use gcc-14 from trixie
- Separate cache based on target to allow for more efficient caching
- Write output to per-target folder, otherwise caches overwrite each other causing recompilation in the install step

### 🐛 Bug Fixes

- Also ignore samply output
- Simplify license, use MIT license instead of BSD, simplify package.json
- Validate toml & sh & ... formatting as part of PR process
- Ignore Cargo.lock from being formatted
- Restore package.json 2 tab width
- Reorder
- Remove unused env that comes in via variables
- Set correct guard name
- Cleanup unused lints
- We know that sort order when iterating over hash-type isn't guaranteed
- Allow cargo features selection
- FeaTures
- Missing read permissions in test and report
- Clippy
- Use the github token to ensure we can download
- Set full version
- Build multi-platform docker images
- Pre-cache
- Copilot instructions
- Fetch per arch, locked, and explicit import
- Src in registry should not be cached
- Lock fetch
- Bring ARG together
- Disable cache dependencies for docker build, the downloading of ./target takes up too much space, and we're not building anyway
- Shrink what we cache
- Fmt doesn't need target
- Download binstall based on runner arch
- We lost the pr-<number>-latest tag
- Restore missed raw tag
- Separate cache package
- Use semver version.
- Set sha for pnpm
- Admin can bypass
- Image as well because renovate is slow
- New changelog

### ⚙️ Miscellaneous Tasks

- Cleanup
- Fmt
- Also format toml
- Simplify prettierconfig
- Move deps, disable as_conversions, too broad
- Fmt
- Don't cache in lint-configs
- Disable codecov, fails too often
- Remove unneeded newline
- More robust downloading of crane
- Pass download format to binstall for cocogitto
- Disable oldstyle branch protection
- Delete old-style protection
- Use musl all the way
- Remove lldb-prettifier built as part of repo, use shared config
- Fmt
- Remove submodule folder
- Sort
- Typo
## [1.11.0] - 2025-07-11

### ⚙️ Miscellaneous Tasks

- *(version)* V1.11.0
## [1.10.0] - 2025-07-11

### 🚀 Features

- Multiplatform with caching
- Enable codeql
- New public function
- Update publish defaults

### 🐛 Bug Fixes

- Switch to prettier's mjs setup, widen limit for non-json files
- Correct dpkg-architecture architecture check
- *(deps)* Update rust crate openssl to 0.10.73
- *(deps)* Update rust crate color-eyre to 0.6.5
- Install with locked to prevent cargo from updating deps during cargo install
- Cache per arch, as these overwrite each other
- Dir, path, I don't know anymore
- Line continuation
- Uppercase
- Remove no-deps
- Reduce permissions
- Manually build Rust for codeql as per our standard build
- Build-mode `manual` is not supported for Rust
- Pnpm
- Ignore pnpm-lock.yaml format
- Ensure cargo.lock is up to date
- Remove unused script
- Also set style_edition
- Infer edition from Cargo.toml
- Schema

### ⚙️ Miscellaneous Tasks

- Fix strip-components, it caused nothing to be placed
- Move scripts
- Fix typo
- Upgrade before installing
- Fix deprecation warning
- Don't prompt to accept commit when no conflicts
- Caching doesn't need the runner's OS
- Disable telemetry, use oidc
- Add coveralls
- Split command, remove prefix
- Testing
- Cargo binstall defaults to cargo install when not found
- Alphabet
- Formatting
- Shuffle stuff around
- Disable multiple_crate_versions, it's just noise
- Update comment
- Cleanup
- Don't push image cargo build/test/... failed
- Fix dockerfile instruction order
- Cleanup
- Rust doesn't support manual mode, no need to pre-build
- Remove glob from path
- Settings update
- Enable clone_on_ref_ptr
- *(version)* V1.10.0
## [1.3.1] - 2025-05-21

### 🚀 Features

- Allowed for concurrent building of docker container
- Grcov -> tarpaulin
- Use cog
- Use shell
- Rust 1.58.0
- Rust 1.60.0
- Rust 1.61.0
- Upgrade semgrep to latest version
- Devcontainer
- More rust 1.62
- Sync-repo-settings first pass
- Generalize dockerfile
- Multi-platform images
- Codecov
- Docker multiplatform
- Support for releasing crates
- Get rid of semantic release, use cocogitto
- Add cross building

### 🐛 Bug Fixes

- Split build and push for faster overall times
- Reduced parent^2 detection complexity
- Forgot to put back line to add SHA to $env
- --quiet isn't quiet
- Initialize otherwise EXITCODE is not set on success, and then it still fails 😅
- Fake commit
- Centralized names
- Removed submodules, going direct
- Switched to fixed commits for actions
- Fix for too much action...
- Removed needs for docker-build to allow parallel operation
- Remove be
- Reduce docker tag complexity
- Aspiring comments
- ToUpper()
- Added skip-tags to prevent it from pushing tags
- Dry run still verifies push permissions
- Even dry-run wants a token
- Handle script dependency
- Corrected library usage
- Forgot .rest
- Variable correction
- Create reports directory, otherwise tool complains
- Capture the rest, not the value called 'rest'
- Renamed for clarity
- Locked node version
- Reran npm install with npm 8 to update package-lock
- Added npm to dependabot
- A change in build scripts should cause a full rebuild
- Use environment variable to get cargo location
- Corrected environment variable
- Updated cargo cache to use tilde again
- See if we can cache the whole .cargo directory
- Updated concurrency key
- Leverage setup-node's ability to read from .nvmrc
- Added missing data
- Alltargets for tarpaulin
- Install tarpaulin from source
- Synced tool invocation parameters
- Bumped rust version
- Added rust-toolchain, symlinked to rust-toolchain.toml
- Getting toolchain file to work
- Try with profile and toolchain specified here
- Infer components from rust-toolchain.toml
- Corrected build dependencies, all-done work be a success if docker-build failed
- The world is ok
- Fixed wrong needs name
- Set up toolchain manually
- -y to accept defaults
- Don't fail tarpaulin when a test fails
- Rebuild when NPM packages change
- Add commit linting
- Npm cleanup
- Reduced complexity tied to semantic release because cog doesn't need it
- Set git user and email so that cog bump works
- Set the right variable
- Ensure we're logged in to use the registry cache
- Reordered cache restore and toolchain setup
- Try coveralls
- Try coveralls
- Renamed step name
- Remove cache exclusion
- Use built-in rustup
- Removed unneeded linting file
- Merged configuration into 1
- Updated to 2021 edition
- Restored packages configuration
- Missing package
- Lock down sha256 of docker images
- Grcov intstead of tarpaulin
- Filename was wrong + comment update + rust-version bump
- Corrected title
- Merge steps for testbased coverage
- Install llvm tools
- Typo in env variable
- Code climate test reporter
- Use correct subcommand
- Remove dependency on build to run on main
- Tag -> sha
- Filter out lcov
- Also keep tests
- Add linebreak
- Set code climate filter
- No need to bring in action just for this
- Removed spaces
- Auto dependabot merge
- Ignore merge commit linting
- Instrument coverage is stable!
- Switch to auto config
- Remove fixes
- Add user
- Set the right user
- Set permissions
- Try centralized renovate config
- Download binstall to /tmp to avoid additional untracked files
- Binstall now wants stuff with a capital
- Don't copy paste
- Lowercase package fmt
- Fix schema
- Don't require reviews
- Force signed commits
- Update rules
- Set the correct tag
- Deny stuff, nobody reads warnings
- Semgrep from container
- Set tag & sha256
- Group imports
- Quotes
- Comment out nightly function, add match |
- No leading pipes
- Correct cocogitto again with binstall
- Google -> probot
- Allow overrides
- Add other settings
- Set-output is deprecated
- Set correct nextest config
- Use up to date action
- Separate before and after cache
- Settings
- Formatting
- Formatting
- Snake in comments is causing parsing failures
- Use cocogitto bump, and support no new version generated
- Prevent cog throwing an error which causes script termination
- Set latest tag
- Switch to editorconfig
- Allow uninlined format args
- Add update script
- Make clippy more annoying
- Hack version (?)
- Remove version, doesn't work for container > image
- Set rangeStrategy
- Unset rangeStrategy, move to the renovate base config
- Editorconfig settings for shell files
- Pin clippy
- Yeet code climate
- Lock down with version and digest
- More formatting enforcement
- Allow for building / not building docker container
- Ascii idents only to prevent idents starting with characters my keyboard can't handle
- Allow disable container retagging
- Updated cache ids
- Comment indent
- Don't retag when we don't build a container
- Set maximum backtrace
- Also do RUST_BACKTRACE=full for debugging
- Build all with tests too
- Trace for all, not just the app
- Default is to use color-eyre
- Trace for run and test
- Add update-name script
- Coveralls as CodeCov keeps on failing
- Specify version, Renovate will pin it
- Make BUILD_DOCKER_CONTAINER configurable from variables
- Env -> vars
- Get application name automatically
- Remove unneeded newline
- We don't use .idea config
- Better defaults
- Consolidate clippy & rust config on top of main, all the rest causes duplication
- Don't show progress
- Workflow_dispatch does not take a branch
- Fix new version
- Add placeholder for env variable
- Move semantic-release config file as per https://github.com/semantic-release/semantic-release/releases/tag/v23.0.0
- Mixed up config name order
- Simplified tags
- Cleanup
- *(deps)* Update rust crate color-eyre to 0.6.3
- Don't set shell, not needed in semgrep container
- Separate scan and fixup, as the scan container doesn't have bash / jq anymore
- Only upload sarif file itself
- Set unpack folder, not filepath
- Platform name
- Filepaths were wrong
- Use artifact v4 settings
- Correctly build musl
- Try something else
- Report tests to codecov for tracking
- Prettier 3.41.0
- Restore needed workflows
- Build container from scratch
- Remove @actions/tool-cache
- Start tracking lldb debug helper
- Disable clippy 1.87.0 let_and_return
- Add runner.arch to the cache keys
- Set correct cache key for the docker step
- Don't install binstall, cargo-edit doesn't have a package anyway
- Remove incorrec exit

### 🧪 Testing

- Flow
- Print changelog.md
- Failure test
- Trigger build

### ⚙️ Miscellaneous Tasks

- Spelling correction
- Removed unneeded print
- Added comment for future me <insert 88mph joke>
- Renamed files
- Debugging changelog printing issues
- Fix linebreaks
- Forgot dependency
- Fix space
- Added final step
- Update README.md
- Updated name everywhere
- Bumped rust numbers everywhere
- Fixed name
- Consolidated npm usage
- Cleanup un-used script
- Foundation for correct version numbers
- Set version number of Rust binary at build time
- Reduce unneeded builds
- Add dry-run to make sure we don't publish preemptively
- Ensure checkout so that we have a package-lock
- Remove spurious )
- I used the wrong script
- Flip aroud switches until we find the right combination
- Hack the semantic-release
- Semantic-release doesn't like refs/pulls/12/merge, surrounding with quotes to test
- Just trying to get  this to work
- Can't go without --no-ci
- Try get last tag
- Removed duplicate version id
- Better way to check out the head?
- Work around semantic-release restrictions
- Hack around semantic-release some more
- Wrong script name
- Skip tag, we don't want to give this one rights to push
- Give more permissions for dry-run
- Testing new flow
- Fix publish script
- Check for changes
- Restructure, remove unneeded submodule pull
- Fix condition
- Split steps
- Reduced complexity of filter
- Beautified titles
- Updated task name
- Prevent clippy from running twice
- Aligned commandline parameters
- Expanded clippy warnings
- Also run pedantic and cargo test on push
- Also run test-and-report on main to update 'main' coverage
- Also run clippy when merged to main to track progress
- Create LICENSE
- Respect cargo.lock when doing cargo install
- Fixed the name
- Remove outdated comment
- Switch to cog
- Switch to cog wip
- Use registry cache
- Typo, docker -> Docker
- Use built-in rustup
- Set the checkout name
- Fixed rustup update warning about rustfmt and cargo-fmt
- No need to run clippy on push to main
- Updated generated cache name
- Removed todo that's not gonna happen
- Set nice name
- Removed unneeded comments
- Exclude rustfmt and cargo-fmt from the cache as rustup doesn't like that
- Use built-in rustup
- Exclude rustfmt and cargo-fmt from the cache as rustup doesn't like that
- Run prettier
- Don't clean before clippy, not needed
- Don't consider it 'all-done' when anything is cancelled
- Also rebuild on cargo.lock changes
- Updated npm packages
- Also bump rust-toolchain to rust 1.58.1
- Cleaned up rustfmt, added 2 settings
- Set test comment mode
- Formatting!
- Don't create new comment, recycle!
- Fixed title
- Correctly report test failure
- Also update cargo & toolchain
- Also update cargo & toolchain
- Fail done properly
- Made task file rust-analyzer compliant
- Spacing and remove verbose
- Add release test script
- Consolidated extensions
- Add title (name) to step
- Docker images names should always be lowercase
- Add shell name and consolidated format
- Delete unneeded file
- Update packages
- Clean up semgrep file, update package-lock
- Crlf to lf
- Install latest semgrep
- Use token to get more rules
- Try to speed up by using binstall
- And more 1.62
- Fix the binary names
- Spacing, made lint-commits also use binstall
- Updated file property to junit_files as per https://github.com/EnricoMi/publish-unit-test-result-action/pull/285
- Remove duplicate `USER`, not needed
- Enable renovate
- Explicitly set token to avoid failed uploads
- Disable coveralls, it's acting up
- Also include rust-specific configs
- Fixed double update typo
- Testing renovate's custom file updater
- Restored codecov, remove executable modifier
- Disable coveralls & codecov
- Put versions so that renovate can tag correctly
- Remove variable from dockerfile
- Try codecov again
- Node v18
- Bump package-lock.json
- Ensure we take the longest tag, v1.0.0 instead of v1
- Updated devcontainer config
- Move to node 20, make backtrace always compile release as we don't care about their internals
- Use lints
- DENY uninlined format args
- ALLOW uninlined format args
- Remove redundant quotes
- Bump uninlined format args priority
- Add mold, use lints
- Restore backtrace always optimize
- Fix typo
- Pin mold
- Disable function-next-line formatting, it looks weird
- Rename nextversion to next_version
- Ensure run and debug from main add the right LOG settings
- Console isn't useful, updated casing of levels
- No trailing commas in json
- Use internal console, not the terminal for debugging
- Fix startColumn/endColumn being 0. Is invalid. Normalize json file for diffing, ignore output. Diff is expected
- Explicitely set prettierrc's path
- Align title
- Checkout to satisfy the codeql tool
- Rename semgrep job to make it register with semgrep
- Use semgrep action, not container
- Back to container, the action is outdated
- Add category
- Semgrep 1 job
- Fix filename
- Allow warnings in test
- Allow warnings in test
- Set checks with new API
- Fix ] typo
- Try codecov
- Support for ARM64
- Try OCI
- Build with matrix
- Also add rust target to name
- Correct params
- Debugging
- Export docker
- Prettier
- Linker for aarch64
- Copy in linker into docker container
- Arm64
- Minor build changes, formatting
- Fix typo
- Also rebuild when .nvmrc changes
- Always run reporting, even when no changes as reports are mandatory
- Disable codecov running plugins, disable codecov searching
- Add linebreaks in the if statements, otherwise the vscode parser gets upset
- Remove unneeded id
- Change name
- Separate the name so the rename script doesn't update it
- Enforce_admins should be null if you want to disable it...
- Syntax consistency, as -> AS
- Formatting
- Remove unneeded .ci
- Formatting
- Enable more lints
- Ensure we have oras
- Fix title
- Install cargo-binstall from updated url
- Rust 1.85.0
- Format dockerfile
- Fmt also 1.85.0
- Remove oras
- Forgot `push`
- Push by tag, not filepath...
- Add logging, try remove unneeded (?) buildx
- Add template clippy.toml
- Fix for rustup 1.28.0 not installing needed toolchain by default
- Install rust-fmt
- Use working-directory
- Set working-directory
- Ensure we restore symlinks
- Remove incorrect comment
- *(version)* V1.2.0
- Ignore generated changelog
- Clippy 1.86 fixes
- Disable required signatures
- Set default debug visualizer
- Update debug setup
- Change wording
- Formatting
- I686 is 32-bit, we need 64-bit
- Remove need for build & targetplatform in scripts
- More precise coverage, don't include test/**
- Make wget more robust
- Remove customization, packages now work oob with binstall
- Convention: bash variable names are lowercase
- Consolidation of scripts
- Move away from env, use output
- Even more variables
- Fix output
- Group variables in single step
- Set revision explicitely
- *(version)* V1.3.0
- *(version)* V1.3.1
## [0.1.0] - 2021-10-30

### 🚀 Features

- Test release
- Got rid of linebreak
- Authors
- Initial commit
- Added Quz and test
- Use crane as tool instead of elaborate docker setup

### 🐛 Bug Fixes

- Enabled codecov
- Codecov
- Bumped cargo version
- Username.toLowerCase()
- Make sure husky doesn't install on CI servers
- Download grcov from releases, WAY faster
- Don't try extract bz2 as gzip
- Arguments go into an array
- 2nd param is dest, not flags, null to infer destination though
- Remove duplicated login

### 🧪 Testing

- Sign with GPG signature
- Sign with GPG signature

### ⚙️ Miscellaneous Tasks

- Copy tags with docker tags again
## [1.9.0] - 2025-01-04

### ⚙️ Miscellaneous Tasks

- *(version)* V1.9.0
## [1.8.0] - 2025-01-04

### 🚀 Features

- Publish to crates.io
- Comments

### ⚙️ Miscellaneous Tasks

- *(version)* V1.8.0
## [1.7.0] - 2025-01-03

### 🚀 Features

- Install cargo-edit

### ⚙️ Miscellaneous Tasks

- *(version)* V1.7.0
## [1.6.0] - 2025-01-03

### 🚀 Features

- Set correct versoin

### ⚙️ Miscellaneous Tasks

- *(version)* V1.6.0
## [1.5.0] - 2025-01-03

### 🚀 Features

- Some new features Friday 2
- Try to publish without semantic release
- Token to trigger publish

### ⚙️ Miscellaneous Tasks

- *(version)* V1.5.0
- *(version)* V1.5.0
## [1.4.0] - 2025-01-03

### 🚀 Features

- Some new features Friday

### 🐛 Bug Fixes

- Consitency

### ⚙️ Miscellaneous Tasks

- *(version)* V1.4.0
## [1.3.0] - 2024-12-26

### ⚙️ Miscellaneous Tasks

- *(version)* V1.3.0
## [1.2.0] - 2024-12-26

### 🚀 Features

- Some new features

### ⚙️ Miscellaneous Tasks

- *(version)* V1.2.0
## [0.0.1] - 2024-12-26

### 🚀 Features

- Feat
- Feat
- Better release and changelog
- After v1.3.0
- After v1.4.0
- Release magic?
- Release magic II
- Restore changelog
- Restore changelog II
- 1.8 test
- Reset changelog
- 1.10 test
- Kill examples
- Cog.sh
- Manual
- Set depth

### 🐛 Bug Fixes

- Username & email

### ⚙️ Miscellaneous Tasks

- *(version)* V1.2.0
- *(version)* V1.5.0
- *(version)* V1.6.0
- *(version)* V1.7.0
- *(version)* V1.8.0
- *(version)* V1.9.0
- *(version)* V1.10.0
- *(version)* V1.11.0
- *(version)* V0.1.0
- *(version)* V0.1.0
- *(version)* V0.1.0
- *(version)* V0.0.1
## [1.1.0] - 2024-12-26

### 🚀 Features

- Reverse 2
## [1.0.0] - 2024-12-26

### 🚀 Features

- Cargo
- Cargo token

### ⚙️ Miscellaneous Tasks

- Initial commit
