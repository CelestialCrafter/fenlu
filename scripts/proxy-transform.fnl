(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn transform [media]
  (if (= (string.sub media.uri 1 (string.len "http")) "http")
      ; https://www.rfc-editor.org/rfc/rfc2396#section-3.2
      (set media.uri (media.uri:gsub "://.-[/?]" (.. "://" config.proxy_authority "/"))))
  media)

