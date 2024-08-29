(local lfs (require :lfs))
(local toml (require :toml))

(local config (toml.parse ...))
(let [schema "file://" path (. config "path")]
  (each [file (lfs.dir path)] (add_uri (.. schema path file))))
