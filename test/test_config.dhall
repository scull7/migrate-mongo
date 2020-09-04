let xdgHomeDir =
  env:XDG_CONFIG_HOME ?
  env:HOME ?
  "./.config"


in
{ migration_dir         = "./migrations"
}
