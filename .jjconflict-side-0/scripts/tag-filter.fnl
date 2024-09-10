(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn has-tag [tags desired]
  (accumulate [found false _ tag (ipairs tags)]
    (or found (= tag desired))))

(fn parse-conditions [query]
  (string.gmatch query "(.-);"))

(local tags (icollect [tag (parse-conditions config.query)] tag))
(fn filter [media]
  (accumulate [has-all true _ tagop (ipairs tags)]
    (and has-all (let [tag (tagop:sub 2) op (tagop:sub 1 1) result (has-tag media.tags tag)]
      (if (= op "!")
          (not result)
          result)))))

