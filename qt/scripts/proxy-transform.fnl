(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn transform [metadata]
  (metadata.url:gsub "://.+/" (.. "://" config.proxy_authority "/"))
  metadata)

