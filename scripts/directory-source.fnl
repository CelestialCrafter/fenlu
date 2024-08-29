(local toml_edit (require :toml_edit))
(local fennel (require :fennel))

(local config (toml_edit.parse ...))

(fn transform [exif]
  (let [handle (assert (io.popen (.. "realpath " (. exif 1)))) directory (handle:read)]
    {
      "uri" (.. "file://" directory "/" (. exif 2))
      "width" (. exif 3)
      "height" (. exif 4)
      "mime" (. exif 5)
      "size" (. exif 6)
    }))

(let [handle (assert (io.popen (.. "exiftool -p '$Directory|$FileName|$ImageWidth|$ImageHeight|$MimeType|$FileSize' " config.path)))]
  (each [line (handle:lines)]
    (add_uri (transform (icollect [s (string.gmatch line "[^|]+")] s)))))


