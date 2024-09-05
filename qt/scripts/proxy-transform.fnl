(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn transform [metadata]
  (set metadata.uri (metadata.uri:gsub "://.-[/?]" (.. "://" config.proxy_authority "/")))
  metadata)

