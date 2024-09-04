(local toml_edit (require :toml_edit))
(local config (toml_edit.parse ...))

(fn transform [exif]
  (let [handle (assert (io.popen (.. "realpath " (. exif 1)))) directory (handle:read)]
    {
    "uri" (.. "file://" directory "/" (. exif 2))
    "mime" (. exif 5)
    "size" (. exif 6)
    "width" (tonumber (. exif 3))
    "height" (tonumber (. exif 4))
    "type" "Image"
    }))

(let [handle (assert (io.popen (.. "exiftool -p '$Directory|$FileName|$ImageWidth|$ImageHeight|$MimeType|$FileSize' " config.path)))]
  (each [line (handle:lines)]
    (add_uri (transform (icollect [s (string.gmatch line "[^|]+")] s)))))


