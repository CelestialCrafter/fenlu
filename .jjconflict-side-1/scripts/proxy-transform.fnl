(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn transform [metadata]
  ; https://www.rfc-editor.org/rfc/rfc2396#section-3.2
  (set metadata.uri (metadata.uri:gsub "://.-[/?]" (.. "://" config.proxy_authority "/")))
  metadata)

