// JSON data is injected by the server as a virtual file at /data.json.
// Access it with: #let data = json("/data.json")
#let data = json("/data.json")
#let bs = data.at("brevSporring", default: (:))

#set document(title: "Kontroll forespørsel")
#set page(margin: (top: 1.5cm, bottom: 1.5cm, left: 1.5cm, right: 1.5cm))
#set text(font: "Source Sans Pro", size: 11pt)

#let checkbox-item(label) = pad(left: 3mm, bottom: 3mm)[
  #box(width: 3mm, height: 3mm, stroke: 1pt + black, baseline: 20%)
  #h(5mm)
  #label
]

// ── Header ───────────────────────────────────────────────────────────────────
#rect(stroke: 1pt + black, width: 100%, inset: 0pt)[
  #grid(
    columns: (2fr, 1fr),
    grid.cell(
      inset: 4mm,
      stroke: (right: 1pt + black),
    )[
      #text(size: 16pt, weight: "bold")[
        Pålegg om utlevering av opplysninger om innskudds- og/eller gjeldskonti
      ]
    ],
    grid.cell(inset: 4mm)[
      #text(size: 9pt, weight: "bold")[Utfyllingsdato:]
      #linebreak()
      #text(size: 9pt)[#bs.at("dagensDato", default: "")]
    ],
  )
]

// ── Request paragraph ─────────────────────────────────────────────────────────
#rect(
  stroke: (left: 1pt + black, right: 1pt + black, bottom: 1pt + black),
  width: 100%,
  inset: 3mm,
)[
  #text(size: 10pt)[
    Vi ber om at dere sender oss fullstendige elektronisk konto-/kundeinformasjon
    for virksomhet/person innen 5 virkedager fra dagens dato
    #bs.at("dagensDato", default: "").
  ]
]

// ── Person information ────────────────────────────────────────────────────────
#rect(
  stroke: (left: 1pt + black, right: 1pt + black, bottom: 1pt + black),
  width: 100%,
  inset: 0pt,
)[
  #pad(left: 2mm, top: 2mm, bottom: 1mm)[
    #text(size: 11pt, weight: "bold")[Opplysninger om person]
  ]
  #grid(
    columns: (40%, 60%),
    inset: (x: 2mm, y: 1mm),
    [*Fornavn / Etternavn: \**],
    [#bs.at("fornavn", default: "") #bs.at("etternavn", default: "")],
    [*Fødselsnummer: \**],
    [#bs.at("customerId", default: "")],
  )
]

// ── Account types ─────────────────────────────────────────────────────────────
#rect(
  stroke: (left: 1pt + black, right: 1pt + black, bottom: 1pt + black),
  width: 100%,
  inset: 0pt,
)[
  #pad(left: 2mm, top: 2mm, bottom: 1mm)[
    #text(size: 11pt, weight: "bold")[
      Vi ber om at dere sender oss fullstendige elektronisk konto-/kundeinformasjon for:
    ]
  ]
  #pad(left: 2mm, bottom: 2mm)[
    #text(size: 11pt, weight: "bold")[
      Periode fra #bs.at("fraDato", default: "") - til #bs.at("tilDato", default: "")
    ]
  ]

  #checkbox-item[Konti opprettet inneværende år med forklarende tekst til transaksjonene]
  #checkbox-item[Alle kundens innskuddskonti med forklarende tekst til transaksjonene]
  #checkbox-item[Andre konti kunden disponerer med forklarende tekst til transaksjonene]
  #checkbox-item[Disponentoversikt kundens konti]
  #checkbox-item[Kredittkort]
  #checkbox-item[Boligkreditt/flexilån eller tilsvarende med transaksjoner i perioden]
  #checkbox-item[Andre lånekonti]

  #pad(left: 3mm, bottom: 3mm)[
    #box(width: 3mm, height: 3mm, stroke: 1pt + black, baseline: 20%)
    #h(5mm)
    Underbilag
    #linebreak()
    #pad(left: 8mm)[
      #text(size: 9pt)[
        Returnér opprinnelig tilsendte transaksjonsutrekk, merket med aktuelle rader.
        Ingen kolonner er fjernet.
      ]
    ]
  ]

  #pad(left: 3mm, bottom: 3mm)[
    #box(width: 3mm, height: 3mm, stroke: 1pt + black, baseline: 20%)
    #h(5mm)
    Annen dokumentasjon
    #linebreak()
    #pad(left: 0mm)[
      #text(size: 9pt)[Spesifiser :]
      #v(4cm)
    ]
  ]

  #pad(left: 3mm, bottom: 3mm)[
    Kontoer :
    #v(4cm)
  ]
]
