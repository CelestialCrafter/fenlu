(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn apply-op [lhs rhs op]
  (let [default true]
    (if (not (and lhs rhs))
        default
        (case op
          ">=" (>= lhs rhs)
          "<=" (<= lhs rhs)
          ">" (> lhs rhs)
          "<" (< lhs rhs)
          "!=" (not= lhs rhs)
          "=" (= lhs rhs)
          _ default))))

(fn parse-conditions [query]
  (string.gmatch query "([wh]) ([<>!=]+) (%w+);"))

(fn filter [media]
  (if (not= media.type "Image")
      true
      (let [ops (icollect [lhs op rhs (parse-conditions config.query)]
                  (apply-op
                    (if (= lhs "w") media.width media.height)
                    (tonumber rhs)
                    op))]
        (accumulate [acc true _ n (ipairs ops)]
          (and acc n)))))

