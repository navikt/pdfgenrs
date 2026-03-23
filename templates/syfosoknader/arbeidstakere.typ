// JSON data is injected by the server as a virtual file at /data.json.
// Access it with: #let data = json("/data.json")
#let data = json("/data.json")

#set document(title: "Søknad om sykepenger")
#set page(margin: (top: 2cm, bottom: 2cm, left: 2.5cm, right: 2.5cm))
#set text(font: "Source Sans Pro", size: 11pt)

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
