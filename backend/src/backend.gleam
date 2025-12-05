import backend/context
import backend/migration
import backend/web
import dot_env
import dot_env/env
import gleam/erlang/process
import gleam/io
import gleam/option.{None}
import gleam/otp/static_supervisor as supervisor
import mist
import pog
import wisp
import wisp/wisp_mist

pub fn main() -> Nil {
  io.println("Hello from backend!")

  // read environment variables
  dot_env.new()
  |> dot_env.set_path("./.env")
  |> dot_env.set_debug(False)
  |> dot_env.load()

  let assert Ok(db_host) = env.get_string("LANCELOT_DB_HOST")
  let assert Ok(db_port) = env.get_int("LANCELOT_DB_PORT")
  let assert Ok(db_user) = env.get_string("LANCELOT_DB_USER")
  let assert Ok(db_password) = env.get_string("LANCELOT_DB_PASSWORD")
  let assert Ok(db_database) = env.get_string("LANCELOT_DB_DATABASE")

  let assert Ok(server_host) = env.get_string("LANCELOT_HOST")
  let assert Ok(server_port) = env.get_int("LANCELOT_PORT")

  let db = process.new_name("db")

  let assert Ok(_) =
    start_application_supervisor(
      db,
      db_host,
      db_port,
      db_user,
      db_password,
      db_database,
    )

  // migrate the database
  let assert Ok(_) =
    db
    |> pog.named_connection()
    |> migration.migrate()

  // request handler for each incoming request
  // - a context gets created with a new id and a database connection
  let request_handler = fn(req: wisp.Request) {
    let db = pog.named_connection(db)
    let ctx = context.Context(id: wisp.random_string(16), db:, auth: None)
    web.handle_request(ctx, req)
  }

  let assert Ok(_) =
    wisp_mist.handler(request_handler, wisp.random_string(64))
    |> mist.new()
    |> mist.port(server_port)
    |> mist.bind(server_host)
    |> mist.start()

  process.sleep_forever()
}

fn start_application_supervisor(
  name pool_name: process.Name(pog.Message),
  host host: String,
  port port: Int,
  user user: String,
  password password: String,
  database database: String,
) {
  // postgres configuration
  let pool_child =
    pog.default_config(pool_name)
    |> pog.host(host)
    |> pog.port(port)
    |> pog.password(option.Some(password))
    |> pog.user(user)
    |> pog.database(database)
    |> pog.pool_size(16)
    |> pog.supervised()

  // supervisor for postgres connection pool
  supervisor.new(supervisor.RestForOne)
  |> supervisor.add(pool_child)
  |> supervisor.start
}
