curl "https://origin.warframe.com/PublicExport/index_en.txt.lzma" |
xz -d |
grep "ExportUpgrades_en.json" |
tr -d "\r" |
xargs -I{} curl "http://content.warframe.com/PublicExport/Manifest/{}" |
perl -pe 's/[\x00-\x1F]/ sprintf "\\u%04X", ord $& /eg' |
jq '.ExportUpgrades | map(select(.name|(contains("Riven") or contains("Unfused Artifact") or contains("Transmute Core"))|not)) | group_by(.name) | map(first)'
