#let data = json("/data/bench/large-output.json")
#set document(title: data.at("title", default: "Large output benchmark"))
#set page(margin: 1cm)

= #data.at("title", default: "Large output benchmark")

#for item in data.at("items", default: ()) [
  == #item.at("title", default: "")
  #item.at("body", default: "")
]
