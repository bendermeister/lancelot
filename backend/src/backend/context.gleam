import gleam/option
import pog

pub type Context {
  Context(id: String, db: pog.Connection, auth: option.Option(String))
}
