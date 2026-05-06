#let data = json("/data.json")
#let legeerklaering = data.legeerklaering
#let validationResult = data.validationResult
#let mottattDato = data.at("mottattDato", default: "")

#let iso_to_nor_date(date_str) = {
  if date_str == none { return "" }
  let s = str(date_str)
  if s == "" { return "" }
  let date_part = s.split("T").at(0, default: s)
  let parts = date_part.split("-")
  if parts.len() >= 3 {
    parts.at(2) + "." + parts.at(1) + "." + parts.at(0)
  } else {
    s
  }
}

#let breaklines(text) = {
  if text == none { return [] }
  let parts = str(text).split("\n")
  parts.join(linebreak())
}

#let check(val) = if val == true [X]
#let yesno(val) = if val == true [Ja] else [Nei]
#let header_fill = rgb("#add8e6")
#let tf(_, y) = if y == 0 { header_fill } else { none }

#set page(margin: 1cm)
#set text(font: "Source Sans Pro", size: 10pt)
#set table(stroke: 1pt + black, inset: 4pt)
#show "\u{2011}": "-"
#show "\u{2642}": "Mann"
#show "\u{2640}": "Kvinne"
#show "\u{1FA7A}": "Stetoskop"
#show "\u{1F539}": "[*]"
#show "\u{1F449}": "->"

// Header with NAV logo
#grid(
  columns: (80%, 20%),
  table(
    columns: (100%),
    fill: (_, _) => header_fill,
    [*#if validationResult.status == "INVALID" [AVVIST ]LEGEERKLÆRING*],
  ),
  align(right + horizon, image("resources/NAVLogoRed.png", width: 100%, alt: "NAV logo")),
)

// Section 0: Erklæringen gjelder
#table(
  columns: (10%, 65%, 25%),
  fill: tf,
  [*0*], [*Erklæringen gjelder*], [],
  [*0.1*], [Arbeidsvurdering ved sykefravær], [#check(legeerklaering.arbeidsvurderingVedSykefravaer)],
  [*0.2*], [Arbeidsavklaringspenger], [#check(legeerklaering.arbeidsavklaringspenger)],
  [*0.3*], [Yrkesrettet attføring], [#check(legeerklaering.yrkesrettetAttforing)],
  [*0.4*], [Uføretrygd(Uførepensjon)], [#check(legeerklaering.uforepensjon)],
)

// Section 1: Opplysninger om pasienten og om arbeidsforhold
#let ag = legeerklaering.pasient.arbeidsgiver
#let ag_adresse = {
  let parts = ()
  let adr = ag.at("adresse", default: none)
  if adr != none and str(adr) != "" { parts.push(str(adr)) }
  let pnr = ag.at("postnummer", default: none)
  if pnr != none { parts.push(str(pnr)) }
  let pst = ag.at("poststed", default: none)
  if pst != none and str(pst) != "" { parts.push(str(pst)) }
  parts.join(" ")
}

#table(
  columns: (10%, 65%, 25%),
  fill: tf,
  [*1*], [*Opplysninger om pasienten og om arbeidsforhold*], [],
  [*1.1*], [Etternavn], [#legeerklaering.pasient.etternavn],
  [*1.1*], [Fornavn], [#legeerklaering.pasient.fornavn],
  [*1.2*], [Fødselsnummer], [#legeerklaering.pasient.fnr],
  [*1.3*], [Telefon], [#legeerklaering.pasient.at("tlfNummer", default: "")],
  [*1.4*], [Navn på pasientens fastlege], [#legeerklaering.signatur.navn],
  [*1.5*], [Yrke], [#legeerklaering.pasient.yrke],
  [*1.6*], [Arbeidsgiverens navn], [#ag.navn],
  [*1.7*], [Arbeidsgiverens addresse], [#ag_adresse],
)

// Section 2: Diagnose og sykdomsopplysninger
#let syk = legeerklaering.sykdomsopplysninger
#let diag_cells = {
  let cells = ()
  let hd = syk.at("hoveddiagnose", default: none)
  if hd != none {
    cells.push([*2.1*])
    cells.push([*Hoveddiagnose*])
    cells.push([*2.1.1 Kode* \ #hd.kode])
    cells.push([*2.1.2 Diagnose* \ #hd.tekst])
  }
  let bd = syk.at("bidiagnose", default: ())
  if type(bd) == array and bd.len() > 0 {
    cells.push([*2.2*])
    cells.push([*Bidiagnoser*])
    cells.push([*2.2.1 Kode* \ #bd.map(b => b.kode).join(linebreak())])
    cells.push([*2.2.2 Diagnose* \ #bd.map(b => b.tekst).join(linebreak())])
  }
  cells
}

#table(
  columns: (5%, 25%, 20%, 50%),
  fill: tf,
  [*2*], [*Diagnose og sykdomsopplysninger*], [], [],
  ..diag_cells,
  [*2.4*], [*Helt arbeidsufør f.o.m.*], [#iso_to_nor_date(syk.arbeidsuforFra)], [],
  [*2.5*], table.cell(colspan: 3)[*Sykehistorie med symptomer og behandling* \ #breaklines(syk.sykdomshistorie)],
  [*2.6*], table.cell(colspan: 3)[*Status presens (angi dato). Resultat av relevante undersøkelser.* \ #breaklines(syk.statusPresens)],
  [*2.7*], [*Bør NAV-kontoret vurdere om det er en:*], table.cell(colspan: 2)[*2.7.1 Yrkesskade/Yrkessykdom* \ #yesno(syk.borNavKontoretVurdereOmDetErEnYrkesskade)],
)

// Section 3: Plan for medisinsk utreding og behandling
#let plan = legeerklaering.plan
#let plan_cells = {
  let cells = ()
  let utredning = plan.at("utredning", default: none)
  if utredning != none {
    cells.push([*3.1*])
    cells.push([*Er pasienten henvist til Utreding* \ #breaklines(utredning.tekst)])
    cells.push([*3.1.1 Dato for henvisn.* \ #iso_to_nor_date(utredning.dato)])
    cells.push([*3.1.2 Antatt ventetid (uker)* \ #utredning.antattVentetIUker])
  }
  let behandling = plan.at("behandling", default: none)
  if behandling != none {
    cells.push([*3.2*])
    cells.push([*Pasienten henvist til Behandling* \ #behandling.tekst])
    cells.push([*3.2.1 Dato for henvisn..* \ #iso_to_nor_date(behandling.dato)])
    cells.push([*3.2.2 Antatt ventetid (uker).* \ #behandling.antattVentetIUker])
  }
  cells
}

#table(
  columns: (5%, 30%, 25%, 40%),
  fill: tf,
  [*3*], [*Plan for medisinsk utreding og behandling*], [], [],
  ..plan_cells,
  [*3.3*], table.cell(colspan: 3)[*Utredningsplan. Oppgi planlagte undersøkelser og tidspunkt/varighet* \ #breaklines(plan.utredningsplan)],
  [*3.4*], table.cell(colspan: 3)[*Behandlingsplan. Oppgi planlagt behandling og tidspunkt/varighet* \ #breaklines(plan.behandlingsplan)],
  [*3.5*], table.cell(colspan: 3)[*Ny vurdering av tidligere utrednings-/behandlingsplan* \ #breaklines(plan.vurderingAvTidligerePlan)],
  [*3.6*], table.cell(colspan: 3)[*Når er det hensiktsmessig, i henhold til utrednings og behandlingsopplegget at NAV-kontoret ber om nye legeopplysninger?* \ #breaklines(plan.narSporreOmNyeLegeopplysninger)],
  [*3.7*], table.cell(colspan: 3)[*Hvis videre behandling ikke er aktuelt, gi begrunnelse* \ #breaklines(plan.videreBehandlingIkkeAktueltGrunn)],
)

// Section 4: Forslag til tiltak utover medisinsk behandling
#let tiltak = legeerklaering.forslagTilTiltak
#let tiltak_items = {
  let items = ()
  if tiltak.at("kjopAvHelsetjenester", default: false) == true { items.push([a. Kjøp av helsetjenester]) }
  if tiltak.at("reisetilskudd", default: false) == true { items.push([b. Reisetilskott i stedet for sykepenger/ arbeidsavklaringspenger]) }
  if tiltak.at("aktivSykmelding", default: false) == true { items.push([c. Aktiv syk-melding]) }
  if tiltak.at("hjelpemidlerArbeidsplassen", default: false) == true { items.push([d. Hjelpemidler på arbeidsplassen]) }
  if tiltak.at("arbeidsavklaringspenger", default: false) == true { items.push([e. Arbeidsavklarings-penger]) }
  if tiltak.at("friskmeldingTilArbeidsformidling", default: false) == true { items.push([f. Friskemelding til arbeidsformidling]) }
  let andreTiltak = tiltak.at("andreTiltak", default: none)
  if andreTiltak != none and str(andreTiltak) != "" { items.push([g. Andre (hvilke?) #andreTiltak]) }
  items
}

#table(
  columns: (10%, 60%, 30%),
  fill: tf,
  [*4*], [*Forslag til tiltak utover medisinsk behandling*], [],
  [*4.1*],
  [*Er det ut fra en medisinsk vurdering aktuelt med noen av følgende tiltak nå?* \ #yesno(tiltak.behov)],
  [#if tiltak.behov == true and tiltak_items.len() > 0 {
    [*Tiltak nå* \ #list(..tiltak_items)]
  }],
  [*4.2*], [*Evt. nærmere opplysninger* \ #tiltak.at("naermereOpplysninger", default: "")], [],
  [*4.3*], [*Hvis ja, oppgi ev. begrensninger i forhold til tiltak. Hvis nei, gi begrunnelse* \ #tiltak.at("tekst", default: "")], [],
)

// Section 5: Medisinsk begrunnet vurdering av funksjons- og arbeidsevne
#let funksjon = legeerklaering.funksjonsOgArbeidsevne
#let pasient_status = {
  let statuses = ()
  if funksjon.at("inntektsgivendeArbeid", default: false) == true { statuses.push("i intektsgivende arbeid") }
  if funksjon.at("hjemmearbeidende", default: false) == true { statuses.push("hjemmearbeidende") }
  if funksjon.at("student", default: false) == true { statuses.push("student") }
  let annet = funksjon.at("annetArbeid", default: none)
  if annet != none and str(annet) != "" { statuses.push(str(annet)) }
  statuses.join(", ")
}

#table(
  columns: (5%, 35%, 25%, 35%),
  fill: tf,
  [*5*], [*Medisinsk begrunnet vurdering av funksjons- og arbeidsevne*], [], [],
  [*5.1*], table.cell(colspan: 3)[*Beskriv hvordan funksjonsevnen generelt er nedsatt på grunn av sykdom* \ #breaklines(funksjon.vurderingFunksjonsevne)],
  [*5.2*], table.cell(colspan: 3)[*Pasienten er:* \ #pasient_status],
  [*5.3*], [*Beskriv kort type arbeid og hvilke krav som stilles*], table.cell(colspan: 2)[#breaklines(funksjon.at("kravTilArbeid", default: ""))],
  [*5.4*],
  [*Vurdering av arbeidsevnen ang tidligere arbeid*],
  [*5.4.1 Gjenoppta det tidligere arbeid* \ #yesno(funksjon.kanGjenopptaTidligereArbeid)],
  [*5.4.2 Når kan pasient gjenoppta det tidligere arbeid* \
    #if funksjon.at("kanGjenopptaTidligereArbeidNa", default: false) == true [Nå]
    #if funksjon.at("kanGjenopptaTidligereArbeidEtterBehandling", default: false) == true [Etter Behandling]
  ],
  [*5.5*],
  [*Vurdering av arbeidsevnen angående å ta annet arbeid*],
  [*5.5.1 Kan ta annet arbeid* \ #yesno(funksjon.kanTaAnnetArbeid)],
  [*5.5.2 Når kan pasient ta annet arbeid* \
    #if funksjon.at("kanTaAnnetArbeidNa", default: false) == true [Nå]
    #if funksjon.at("kanTaAnnetArbeidEtterBehandling", default: false) == true [Etter Behandling]
  ],
  [*5.6*],
  [*Vurdering av arbeidsevnen og eventuelle hensyn*],
  [*5.6.1 Hva kan pasient ikke gjøre i det nåværende arbeid?* \ #funksjon.at("kanIkkeGjenopptaNaverendeArbeid", default: "")],
  [*5.6.2 Hvilke andre hensyn må eventuelt tas ved valg av annet yrke/arbeid?* \ #funksjon.at("kanIkkeTaAnnetArbeid", default: "")],
)

// Section 6: Prognose
#let prog = legeerklaering.prognose
#table(
  columns: (5%, 70%, 25%),
  fill: tf,
  [*6*], [*Prognose*], [],
  [*6.1*], [*a) Antas behandlingen å føre til bedring av arbeidsevnen?* \ #yesno(prog.vilForbedreArbeidsevne)], [],
  [*6.2*], [*b) Anslå varigheten av sykdom/skade (ev. lyte)* \ #prog.anslattVarighetSykdom], [],
  [*6.3*], [*c) Anslå varigheten av funksjonsnedsettelsen* \ #prog.anslattVarighetFunksjonsnedsetting], [],
  [*6.4*], [*d) Anslå varigheten av den nedsatte arbeidsevnen* \ #prog.anslattVarighetNedsattArbeidsevne], [],
)

// Section 7: Årsakssammenheng
#table(
  columns: (5%, 95%),
  fill: tf,
  [*7*], [*Årsakssammenheng*],
  [*7.1*], [*Anslå hvor stor betydning funksjonsnedsettelsen har for at arbeidsevnen er nedsatt* \ #breaklines(legeerklaering.at("arsakssammenheng", default: ""))],
)

// Section 8: Andre opplysninger
#table(
  columns: (5%, 95%),
  fill: tf,
  [*8*], [*Andre opplysninger*],
  [*8.1*], [#breaklines(legeerklaering.at("andreOpplysninger", default: ""))],
)

// Section 9: Samarbeid/kontakt
#let kontakt = legeerklaering.kontakt
#let kontakt_items = {
  let items = ()
  if kontakt.at("skalKontakteBehandlendeLege", default: false) == true { items.push([Behandlende lege]) }
  if kontakt.at("skalKontakteArbeidsgiver", default: false) == true { items.push([Arbeidsgiver]) }
  if kontakt.at("skalKontakteBasisgruppe", default: false) == true { items.push([Basisgruppe]) }
  let annen = kontakt.at("kontakteAnnenInstans", default: none)
  if annen != none and str(annen) != "" { items.push([#annen]) }
  items
}

#table(
  columns: (50%, 50%),
  fill: tf,
  [*9*], [*Samarbeid/kontakt*],
  [*9.1 Kryss av for den du ønsker NAV-kontoret skal ta kontakt med*],
  [#if kontakt_items.len() > 0 { list(..kontakt_items) }],
)

// Section 10: Forbehold
#let pasient_ikke_vite = legeerklaering.at("pasientenBurdeIkkeVite", default: none)
#table(
  columns: (70%, 30%),
  fill: tf,
  [*10*], [*Forbehold*],
  [*10.1 Er det noe i legeerklæringen som pasienten ikke bør få vite av medisinske grunner?*],
  [#if pasient_ikke_vite != none and str(pasient_ikke_vite) != "" [#pasient_ikke_vite] else [Nei]],
)

// Section 11: Legens underskrift mv.
#let sig = legeerklaering.signatur
#table(
  columns: (30%, 70%),
  fill: tf,
  [*11*], [*Legens underskrift mv.*],
  [*11.1 Dato*], [#iso_to_nor_date(sig.dato)],
  [*11.2 Legens adresse*], [#sig.adresse, #sig.postnummer #sig.poststed],
  [*11.3 Legens underskrift*], [#sig.navn],
  [*11.4 Legens telefonnummer*], [#sig.at("tlfNummer", default: "")],
)

// Section 12: Begrunnelse for avvising (only shown when INVALID)
#if validationResult.status == "INVALID" {
  table(
    columns: (5%, 95%),
    fill: tf,
    [*12*], [*Begrunnelse for avvising*],
    [], [#list(..validationResult.ruleHits.map(h => [#h.messageForSender]))],
  )
}

// Footer
#table(
  columns: (50%, 50%),
  [Teknisk sporingsinformasjon for denne legeerklæringen: #legeerklaering.id],
  [Mottatt dato NAV: #iso_to_nor_date(mottattDato)],
)
