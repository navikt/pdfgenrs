#let data = json("/data/bench/html-bench.json")
#set document(title: data.at("title", default: "Benchmark"))
= #data.at("title", default: "Benchmark")
#data.at("body", default: "")
#for item in data.at("items", default: ()) [
  - *#item.at("name", default: "")* — #item.at("value", default: "")
]
