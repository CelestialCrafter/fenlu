(local curl (require :cURL))
(local json (require :dkjson))
(local toml_edit (require :toml_edit))

(local config (toml_edit.parse ...))

(fn sleep [sec] (os.execute (.. "sleep " sec)))

(fn transform [post] (let [date (: (: (: (post.updateDate:gsub "-" "/") :gsub "T" "/") :gsub ":" "/") :gsub "%+.+" "")]
                       {
                       "title" post.title
                       "uri" (.. "http://i.pximg.net/img-master/img/" date "/" post.id "_p0_master1200.jpg")
                       "width" post.width
                       "height" post.height
                       "type" "Image"
                       "tags" post.tags
                       }))

(fn request [offset]
  (let [out []
            url (.. "https://www.pixiv.net/ajax/user/" config.account.user_id "/illusts/bookmarks?tag=&offset=" offset "&limit=" config.max "&rest=" (if config.nsfw "hide" "show") "&lang=en")]
    (with-open [h (curl.easy {
                             :url url
                             :httpheader ["User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) Gecko/20100101 Firefox/103.0"
                                          (.. "Cookie: PHPSESSID=" config.account.user_id "_" config.account.token)]
                             :writefunction {:write #(table.insert out $2)}
                             })]
      (h:perform))
    (json.decode (table.concat out ""))))

(var offset 0)
(var prevPosts config.max)
(while (>= prevPosts config.max)
  (let [data (request offset)]
    (set offset (+ offset config.max))
    (set prevPosts (length data.body.works))
    (each [_ post (ipairs data.body.works)]
      ; only images & undeleted works
      (if (and (= post.illustType 0) (not= post.updateDate "1970-01-01T00:00:00+09:00"))
          (add_uri (transform post))))))
(sleep config.request_delay)
