(local curl (require :cURL))
(local json (require :dkjson))
(local toml_edit (require :toml_edit))

(local config (toml_edit.parse ...))

(fn transform [post]
  {
  "title" post.title
  "uri" (.. "http://kemono.su" post.file.path)
  "width" 0
  "height" 0
  "type" "Image"
  })

(fn request []
  (let [out []
            url "https://kemono.su/api/v1/account/favorites?type=post"]
    (with-open [h (curl.easy {
                             :url url
                             :httpheader [(.. "Cookie: session=" config.account.token)]
                             :writefunction {:write #(table.insert out $2)}
                             })]
      (h:perform))
    (json.decode (table.concat out ""))))

(let [data (request)]
  (each [_ post (ipairs data)]
    (add_uri (transform post))))
