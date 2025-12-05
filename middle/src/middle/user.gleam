import gleam/dynamic/decode
import gleam/int
import gleam/json
import gleam/result

pub type Id {
  Id(inner: Int)
}

pub fn id_to_string(id: Id) -> String {
  id.inner |> int.to_string()
}

pub fn id_from_string(id: String) -> Result(Id, Nil) {
  id |> int.parse() |> result.map(Id)
}

pub type User {
  User(id: Id, name: String)
}

pub fn id_to_json(id: Id) -> json.Json {
  id.inner |> json.int
}

pub fn id_decoder() {
  use id <- decode.then(decode.int)
  id |> Id |> decode.success
}

pub fn to_json(user: User) {
  [#("id", user.id |> id_to_json), #("name", user.name |> json.string)]
  |> json.object()
}
