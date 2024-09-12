(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn transform [media]
  (var suffix "")
  (if (= media.source "kemono-source.fnl")
      (set suffix "kemono/"))
  (if (= media.source "pixiv-source.fnl")
      (set suffix "pixiv/"))
  (if (= media.source "arxiv-source.fnl")
      (set suffix "arxiv/"))
  (if (= (string.sub media.uri 1 (string.len "http")) "http")
      ; https://www.rfc-editor.org/rfc/rfc2396#section-3.2
      (set media.uri (media.uri:gsub "://.-[/?]" (.. "://" config.proxy_authority "/" suffix))))
  media)

