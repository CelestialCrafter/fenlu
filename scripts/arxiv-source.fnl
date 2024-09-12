(local curl (require :cURL))
(local lom (require :lxp.lom))
(local toml_edit (require :toml_edit))

(local config (toml_edit.parse ...))

(fn find-pdf-link [node]
  (let [urls (icollect [child (lom.list_children node)]
               (if (and (= child.tag "link") (= child.attr.title "pdf"))
                   child.attr.href))]
    (. urls 1)))

(fn transform [paper]
  {
  "title" (. (lom.find_elem paper "title") 1)
  "uri" (find-pdf-link paper)
  "author" (. (lom.find_elem paper "name") 1)
  "summary" (. (lom.find_elem paper "summary") 1)
  "type" "PDF"
  })

(fn request []
  (let [out []
            url (.. "http://export.arxiv.org/api/query?max_results=2000&search_query=" config.query)]
    (with-open [h (curl.easy {
                             :url url
                             :writefunction {:write #(table.insert out $2)}
                             })]
      (h:perform))
    (lom.parse (table.concat out ""))))

(if (not= config.query "")
    (each [paper (lom.list_children (request) "entry")]
      (add_uri (transform paper))))
