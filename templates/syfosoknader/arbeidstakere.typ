// JSON data is injected by the server as a virtual file at /data/syfosoknader/arbeidstakere.json.
// Access it with: #let data = json("/data/syfosoknader/arbeidstakere.json")
#let data = json("/data/syfosoknader/arbeidstakere.json")

#set document(title: "Søknad om sykepenger")
#set page(margin: (top: 2cm, bottom: 2cm, left: 2.5cm, right: 2.5cm))
#set text(font: ("Source Sans 3", "Noto Color Emoji"), lang: "nb", size: 10pt, fallback: false)

#align(center)[
  #text(size: 18pt, weight: "bold")[Søknad om sykepenger]
]

#v(1em)

#grid(
  columns: (auto, 1fr),
  gutter: 0.5em,
  [*Navn:*], [#data.at("navn", default: "")],
  [*Fødselsnummer:*], [#data.at("fnr", default: "")],
  [*Periode:*], [#data.at("fom", default: "") – #data.at("tom", default: "")],
)
