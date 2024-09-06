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
          "!" (not= lhs rhs)
          "=" (= lhs rhs)
          _ default))))

(fn filter [metadata]
  (if (not= metadata.type "Image")
      true
      (and (apply-op metadata.width (tonumber (?. config.width :value)) (?. config.width :op))
           (apply-op metadata.height (tonumber (?. config.height :value)) (?. config.height :op)))))

