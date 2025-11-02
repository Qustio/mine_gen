prefix = get_option('prefix')

pkgdatadir = prefix / get_option('datadir') / meson.project_name()
gnome = import('gnome')

gnome.compile_resources('cubiomesgtk',
  'cubiomesgtk.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'cubiomesgtk')
conf.set_quoted('LOCALEDIR', prefix / get_option('localedir'))
conf.set_quoted('PKGDATADIR', pkgdatadir)

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)
# Copy the config.rs output to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'cubiomes-gtk' / 'src' / 'config.rs',
  meson.project_source_root() / 'cubiomes-gtk' / 'src' / 'config.rs',
  check: true
)

cargo_bin  = find_program('cargo')
cargo_opt  = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opt += [ '--features', 'meson' ]
cargo_env  = {
  'CARGO_HOME': meson.project_build_root()  / 'cargo'
}

if get_option('buildtype') == 'release'
  cargo_opt += [ '--release' ]
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_opt += [ '--target-dir', meson.project_build_root()  / 'out' ]

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: rust_target,
  console: true,
  env: cargo_env,
  command: [
    cargo_bin,
    'build',
    cargo_opt,
  ]
)

custom_target(
  'cp-binary',
  depends: cargo_build,
  build_by_default: true,
  build_always_stale: true,
  install: true,
  install_dir: get_option('bindir'),
  output: meson.project_name(),
  command: [
    'cp',
    meson.project_build_root() / 'out' / rust_target / meson.project_name(),
    '@OUTPUT@',
  ],
)

test(
  'Cargo-test',
  cargo_bin,
  args: [
    'test',
    cargo_opt
  ],
  timeout: 1800
)
